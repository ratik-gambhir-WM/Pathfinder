use std::{
    env, fs,
    path::{Component, Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

const MAX_PDF_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRoomTreeNode {
    pub children: Option<Vec<DataRoomTreeNode>>,
    pub default_expanded: bool,
    pub error: Option<String>,
    pub id: String,
    pub kind: String,
    pub name: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealDataRoom {
    pub deal_id: String,
    pub root_name: String,
    pub root_path: String,
    pub tree: Vec<DataRoomTreeNode>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentPreview {
    pub file_name: String,
    pub mime_type: String,
    pub pdf_base64: String,
    pub source_kind: String,
}

pub fn list_deal_data_room(deal_id: String) -> Result<DealDataRoom, String> {
    let configured_root = deal_data_room_root(&deal_id)
        .ok_or_else(|| format!("no local data-room root is configured for deal \"{deal_id}\""))?;
    let root = configured_root.canonicalize().map_err(|err| {
        format!(
            "the configured data-room root is unavailable ({}): {err}",
            configured_root.display()
        )
    })?;

    if !root.is_dir() {
        return Err(format!(
            "the configured data-room root is not a directory: {}",
            root.display()
        ));
    }

    let root_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Data Room")
        .to_string();
    let root_node = build_directory_node(&root, &root, Path::new(""), true);

    Ok(DealDataRoom {
        deal_id,
        root_name,
        root_path: root.display().to_string(),
        tree: vec![root_node],
    })
}

pub fn build_document_preview(
    deal_id: &str,
    relative_path: &str,
) -> Result<DocumentPreview, String> {
    let root = canonical_deal_root(deal_id)?;
    let file_path = resolve_relative_file(&root, relative_path)?;
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Document")
        .to_string();
    let extension = file_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let (pdf_bytes, source_kind) = match extension.as_str() {
        "pdf" => (read_pdf(&file_path)?, "native".to_string()),
        "docx" | "xlsx" | "pptx" => (
            convert_office_to_pdf(&file_path)?,
            format!("converted-from-{extension}"),
        ),
        _ => {
            let shown_extension = if extension.is_empty() {
                "files without an extension".to_string()
            } else {
                format!(".{extension} files")
            };
            return Err(format!(
                "Preview is unsupported for {shown_extension}. Supported formats are PDF, DOCX, XLSX, and PPTX."
            ));
        }
    };

    if pdf_bytes.len() as u64 > MAX_PDF_BYTES {
        return Err(format!(
            "The generated PDF is too large to preview ({} MB; limit is {} MB).",
            pdf_bytes.len() / (1024 * 1024),
            MAX_PDF_BYTES / (1024 * 1024)
        ));
    }

    Ok(DocumentPreview {
        file_name,
        mime_type: "application/pdf".to_string(),
        pdf_base64: general_purpose::STANDARD.encode(pdf_bytes),
        source_kind,
    })
}

fn deal_data_room_root(deal_id: &str) -> Option<PathBuf> {
    let root = match deal_id {
        "project-alpha" => "/Users/rgambhir/BetaNXT/02 - Data Room (CIM, Target Docs)",
        "project-beta" => "/Users/rgambhir/OmegaHealthcare/02. Discovery",
        "logistics-merger" => "/Users/rgambhir/Telluride-Discovery",
        _ => return None,
    };

    Some(PathBuf::from(root))
}

fn canonical_deal_root(deal_id: &str) -> Result<PathBuf, String> {
    let configured_root = deal_data_room_root(deal_id)
        .ok_or_else(|| format!("no local data-room root is configured for deal \"{deal_id}\""))?;
    let root = configured_root.canonicalize().map_err(|err| {
        format!(
            "the configured data-room root is unavailable ({}): {err}",
            configured_root.display()
        )
    })?;

    if !root.is_dir() {
        return Err(format!(
            "the configured data-room root is not a directory: {}",
            root.display()
        ));
    }

    Ok(root)
}

fn resolve_relative_file(root: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let relative = Path::new(relative_path);
    if relative_path.trim().is_empty()
        || relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(
            "document path must be a non-empty path relative to the deal data room".to_string(),
        );
    }

    let candidate = root.join(relative);
    let canonical_candidate = candidate.canonicalize().map_err(|err| {
        format!(
            "the selected document is inaccessible ({}): {err}",
            relative.display()
        )
    })?;

    if !canonical_candidate.starts_with(root) {
        return Err("the selected document is outside the configured deal data room".to_string());
    }

    if !canonical_candidate.is_file() {
        return Err(format!(
            "the selected path is not a file: {}",
            relative.display()
        ));
    }

    Ok(canonical_candidate)
}

fn build_directory_node(
    root: &Path,
    directory: &Path,
    relative_path: &Path,
    default_expanded: bool,
) -> DataRoomTreeNode {
    let name = if relative_path.as_os_str().is_empty() {
        root.file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("Data Room")
            .to_string()
    } else {
        directory
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("Folder")
            .to_string()
    };
    let id = if relative_path.as_os_str().is_empty() {
        "data-room-root".to_string()
    } else {
        relative_path.to_string_lossy().to_string()
    };

    match fs::read_dir(directory) {
        Ok(entries) => {
            let mut children = Vec::new();

            for entry_result in entries {
                let entry = match entry_result {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                let entry_name = entry.file_name().to_string_lossy().to_string();
                if should_ignore_entry(&entry_name) {
                    continue;
                }

                let child_relative = relative_path.join(&entry_name);
                let file_type = match entry.file_type() {
                    Ok(file_type) => file_type,
                    Err(err) => {
                        children.push(inaccessible_file_node(&entry_name, &child_relative, err));
                        continue;
                    }
                };

                if file_type.is_dir() {
                    children.push(build_directory_node(
                        root,
                        &entry.path(),
                        &child_relative,
                        false,
                    ));
                } else if file_type.is_file() {
                    children.push(file_node(&entry_name, &child_relative));
                }
            }

            children.sort_by(|left, right| {
                let left_folder = left.kind == "folder";
                let right_folder = right.kind == "folder";
                right_folder
                    .cmp(&left_folder)
                    .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            });

            DataRoomTreeNode {
                children: Some(children),
                default_expanded,
                error: None,
                id,
                kind: "folder".to_string(),
                name,
                relative_path: None,
            }
        }
        Err(err) => DataRoomTreeNode {
            children: Some(Vec::new()),
            default_expanded,
            error: Some(format!("This folder cannot be read: {err}")),
            id,
            kind: "folder".to_string(),
            name,
            relative_path: None,
        },
    }
}

fn file_node(name: &str, relative_path: &Path) -> DataRoomTreeNode {
    let extension = relative_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let kind = match extension.as_str() {
        "pdf" => "pdf",
        "xlsx" => "sheet",
        _ => "doc",
    };
    let relative_path = relative_path.to_string_lossy().to_string();

    DataRoomTreeNode {
        children: None,
        default_expanded: false,
        error: None,
        id: relative_path.clone(),
        kind: kind.to_string(),
        name: name.to_string(),
        relative_path: Some(relative_path),
    }
}

fn inaccessible_file_node(
    name: &str,
    relative_path: &Path,
    error: std::io::Error,
) -> DataRoomTreeNode {
    let mut node = file_node(name, relative_path);
    node.error = Some(format!("This item cannot be read: {error}"));
    node
}

fn should_ignore_entry(name: &str) -> bool {
    name.starts_with('.') || name.starts_with("~$")
}

fn read_pdf(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = path.metadata().map_err(|err| {
        format!(
            "failed to inspect the selected PDF ({}): {err}",
            path.display()
        )
    })?;
    if metadata.len() > MAX_PDF_BYTES {
        return Err(format!(
            "The PDF is too large to preview ({} MB; limit is {} MB).",
            metadata.len() / (1024 * 1024),
            MAX_PDF_BYTES / (1024 * 1024)
        ));
    }

    let bytes = fs::read(path).map_err(|err| {
        format!(
            "failed to read the selected PDF ({}): {err}",
            path.display()
        )
    })?;
    if !bytes.starts_with(b"%PDF-") {
        return Err("the selected .pdf file does not contain a valid PDF header".to_string());
    }

    Ok(bytes)
}

fn convert_office_to_pdf(path: &Path) -> Result<Vec<u8>, String> {
    let converter = find_soffice().ok_or_else(|| {
        "Office preview conversion is unavailable because LibreOffice/soffice was not found. Install LibreOffice or set PATHFINDER_SOFFICE to its executable path.".to_string()
    })?;
    let temp_root = unique_preview_temp_dir();
    let output_dir = temp_root.join("output");
    let profile_dir = temp_root.join("profile");
    fs::create_dir_all(&output_dir)
        .and_then(|_| fs::create_dir_all(&profile_dir))
        .map_err(|err| format!("failed to create a temporary conversion directory: {err}"))?;

    let profile_url = format!("file://{}", profile_dir.display());
    let output = Command::new(&converter)
        .arg(format!("-env:UserInstallation={profile_url}"))
        .arg("--headless")
        .arg("--convert-to")
        .arg("pdf")
        .arg("--outdir")
        .arg(&output_dir)
        .arg(path)
        .env("XDG_CACHE_HOME", &profile_dir)
        .output();

    let result = match output {
        Ok(output) if output.status.success() => (|| {
            let generated_pdf = fs::read_dir(&output_dir)
                .map_err(|err| format!("failed to inspect converted PDF output: {err}"))?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .find(|candidate| {
                    candidate
                        .extension()
                        .and_then(|value| value.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
                })
                .ok_or_else(|| {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    format!(
                        "LibreOffice completed without producing a PDF. Output: {} {}",
                        stdout.trim(),
                        stderr.trim()
                    )
                })?;
            fs::read(&generated_pdf)
                .map_err(|err| format!("failed to read the converted PDF: {err}"))
        })(),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(format!(
                "LibreOffice could not convert this document (exit {}). {} {}",
                output.status,
                stdout.trim(),
                stderr.trim()
            ))
        }
        Err(err) => Err(format!(
            "failed to start Office preview converter ({}): {err}",
            converter.display()
        )),
    };

    let _ = fs::remove_dir_all(&temp_root);
    result
}

fn find_soffice() -> Option<PathBuf> {
    if let Some(configured) = env::var_os("PATHFINDER_SOFFICE") {
        let path = PathBuf::from(configured);
        if path.is_file() {
            return Some(path);
        }
    }

    if let Some(path) = env::var_os("PATH") {
        for directory in env::split_paths(&path) {
            let candidate = directory.join("soffice");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    let mut candidates = vec![
        PathBuf::from("/Applications/LibreOffice.app/Contents/MacOS/soffice"),
        PathBuf::from("/opt/homebrew/bin/soffice"),
        PathBuf::from("/usr/local/bin/soffice"),
        PathBuf::from("/usr/bin/soffice"),
    ];
    if let Some(home) = env::var_os("HOME") {
        candidates.push(
            PathBuf::from(home).join(
                ".cache/codex-runtimes/codex-primary-runtime/dependencies/bin/override/soffice",
            ),
        );
    }

    candidates.into_iter().find(|path| path.is_file())
}

fn unique_preview_temp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    env::temp_dir().join(format!(
        "pathfinder-document-preview-{}-{timestamp}",
        std::process::id()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTUAL_PDF_RELATIVE_PATH: &str = "4 Security and Compliance/4.1 Cybersecurity/4.1.2 Cybersecurity testing and remediation/BetaNXT Standard - Application Security Testing.pdf";

    #[test]
    fn every_mock_deal_has_a_configured_data_room_root() {
        for deal_id in ["project-alpha", "project-beta", "logistics-merger"] {
            assert!(deal_data_room_root(deal_id).is_some());
        }
    }

    #[test]
    fn ignored_entries_cover_hidden_and_office_lock_files() {
        assert!(should_ignore_entry(".DS_Store"));
        assert!(should_ignore_entry("~$working-copy.docx"));
        assert!(!should_ignore_entry("working-copy.docx"));
    }

    #[test]
    fn file_node_uses_relative_path_as_stable_id() {
        let node = file_node("Example.pdf", Path::new("folder/Example.pdf"));
        assert_eq!(node.id, "folder/Example.pdf");
        assert_eq!(node.kind, "pdf");
        assert_eq!(node.relative_path.as_deref(), Some("folder/Example.pdf"));
    }

    #[test]
    fn available_office_fixture_converts_to_a_real_pdf() {
        let fixture = Path::new(
            "/Users/rgambhir/BetaNXT/02 - Data Room (CIM, Target Docs)/List of Items.docx",
        );
        if !fixture.is_file() || find_soffice().is_none() {
            return;
        }

        let bytes = convert_office_to_pdf(fixture).expect("DOCX fixture should convert to PDF");
        assert!(bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn actual_pdf_fixture_builds_a_native_preview() {
        let fixture = deal_data_room_root("project-alpha")
            .unwrap()
            .join(ACTUAL_PDF_RELATIVE_PATH);
        if !fixture.is_file() {
            return;
        }

        let preview = build_document_preview("project-alpha", ACTUAL_PDF_RELATIVE_PATH)
            .expect("actual PDF fixture should build a preview");
        let bytes = general_purpose::STANDARD
            .decode(preview.pdf_base64)
            .expect("preview should contain valid base64");

        assert_eq!(preview.mime_type, "application/pdf");
        assert_eq!(preview.source_kind, "native");
        assert!(bytes.starts_with(b"%PDF-"));
    }
}
