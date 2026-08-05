use std::{env, fs, path::Path};

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    core::{
        clients::openai::{OpenAiClient, ResponsesFileInput},
        display_relative_path, infer_supported_mime_type,
    },
    repository::deal_repository::{
        create_deal, get_deal_by_id, upsert_deal_metadata, CreateDealRecord, Deal, DealMetadata,
        UpsertDealMetadataRecord,
    },
    state::AppState,
};

const DEFAULT_DEAL_EXTRACTION_MODEL: &str = "gpt-5.6-luna";
const SOW_MATCH_TERMS: [&str; 2] = ["sow", "scope of work"];
const PROJECT_TIMELINE_MATCH_TERMS: [&str; 6] = [
    "project timeline",
    "timeline",
    "project plan",
    "workplan",
    "work plan",
    "schedule",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDealAndExtractInput {
    pub deal_name: String,
    pub main_data_room_folder: String,
    pub deal_type: String,
    pub pe_firm: String,
    pub target_company: Option<String>,
    pub buyer_or_platform_company: Option<String>,
    pub parent_or_seller_company: Option<String>,
    pub carve_out_business: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealSourceFile {
    pub path: String,
    pub filename: String,
    pub relative_path: String,
    pub size_bytes: u64,
    pub matched_on: Vec<String>,
    pub text_extracted: bool,
    pub text_truncated: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealExtraction {
    pub key_questions: Vec<String>,
    pub investment_thesis: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDealAndExtractResponse {
    pub deal: Deal,
    pub files: Vec<DealSourceFile>,
    pub extraction: DealExtraction,
    pub metadata: DealMetadata,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDealAndFindFilesResponse {
    pub deal: Deal,
    pub files: Vec<DealSourceFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractDealQuestionsAndThesisInput {
    pub deal_id: i64,
    pub sow_file_path: Option<String>,
    pub project_timeline_file_path: Option<String>,
}

struct MatchedDealFile {
    source_file: DealSourceFile,
    data_base64: Option<String>,
    mime_type: Option<&'static str>,
}

pub async fn save_deal_and_extract(
    state: &AppState,
    input: SaveDealAndExtractInput,
) -> Result<SaveDealAndFindFilesResponse, String> {
    validate_deal_input(&input)?;
    let deal = save_deal(state, &input)?;
    let matched_files = discover_sow_and_timeline_files(Path::new(&deal.main_data_room_folder))?;
    let files = matched_files
        .into_iter()
        .map(|file| file.source_file)
        .collect::<Vec<_>>();

    Ok(SaveDealAndFindFilesResponse { deal, files })
}

pub async fn extract_deal_questions_and_thesis_for_selected_files(
    state: &AppState,
    input: ExtractDealQuestionsAndThesisInput,
) -> Result<SaveDealAndExtractResponse, String> {
    let deal = get_deal_by_id(state, input.deal_id)?
        .ok_or_else(|| format!("deal not found for id `{}`", input.deal_id))?;
    let matched_files = load_selected_deal_files(&deal, &input)?;
    let extraction = extract_deal_questions_and_thesis_from_files(&deal, &matched_files).await?;
    let metadata = persist_deal_metadata(state, &deal, &extraction)?;
    let files = matched_files
        .into_iter()
        .map(|file| file.source_file)
        .collect::<Vec<_>>();

    Ok(SaveDealAndExtractResponse {
        deal,
        files,
        extraction,
        metadata,
    })
}

fn persist_deal_metadata(
    state: &AppState,
    deal: &Deal,
    extraction: &DealExtraction,
) -> Result<DealMetadata, String> {
    let key_questions_json = serde_json::to_string(&extraction.key_questions)
        .map_err(|err| format!("failed to serialize deal key questions: {err}"))?;
    let (document_count, data_room_size_bytes) =
        measure_data_room(Path::new(&deal.main_data_room_folder))?;

    //Chnange to Helix??
    upsert_deal_metadata(
        state,
        UpsertDealMetadataRecord {
            deal_id: deal.id,
            key_questions_json: &key_questions_json,
            investment_thesis: &extraction.investment_thesis,
            document_count,
            data_room_size_bytes,
        },
    )
}

fn measure_data_room(root: &Path) -> Result<(i64, i64), String> {
    if !root.exists() {
        return Err(format!(
            "data room folder does not exist: {}",
            root.display()
        ));
    }

    if !root.is_dir() {
        return Err(format!(
            "data room path is not a folder: {}",
            root.display()
        ));
    }

    let mut document_count = 0_i64;
    let mut data_room_size_bytes = 0_i64;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if is_ignored_source_file(path) {
            continue;
        }

        let metadata = fs::metadata(path)
            .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?;
        let file_size_bytes = i64::try_from(metadata.len())
            .map_err(|_| "data room file size exceeds supported range".to_string())?;

        document_count += 1;
        data_room_size_bytes = data_room_size_bytes
            .checked_add(file_size_bytes)
            .ok_or_else(|| "data room size exceeds supported range".to_string())?;
    }

    Ok((document_count, data_room_size_bytes))
}

fn save_deal(state: &AppState, input: &SaveDealAndExtractInput) -> Result<Deal, String> {
    save_deal_with_repository(input, |record| create_deal(state, record))
}

fn save_deal_with_repository<'a>(
    input: &'a SaveDealAndExtractInput,
    persist: impl FnOnce(CreateDealRecord<'a>) -> Result<Deal, String>,
) -> Result<Deal, String> {
    let deal_name = input.deal_name.trim();
    let main_data_room_folder = input.main_data_room_folder.trim();
    let deal_type = input.deal_type.trim();
    let pe_firm = input.pe_firm.trim();
    let target_company = trim_optional(input.target_company.as_deref());
    let buyer_or_platform_company = trim_optional(input.buyer_or_platform_company.as_deref());
    let parent_or_seller_company = trim_optional(input.parent_or_seller_company.as_deref());
    let carve_out_business = trim_optional(input.carve_out_business.as_deref());

    persist(CreateDealRecord {
        deal_name,
        main_data_room_folder,
        deal_type,
        pe_firm,
        target_company,
        buyer_or_platform_company,
        parent_or_seller_company,
        carve_out_business,
    })
}

fn discover_sow_and_timeline_files(root: &Path) -> Result<Vec<MatchedDealFile>, String> {
    collect_sow_and_timeline_files_with_options(root, false)
}

fn collect_sow_and_timeline_files_with_options(
    root: &Path,
    include_file_data: bool,
) -> Result<Vec<MatchedDealFile>, String> {
    if !root.exists() {
        return Err(format!(
            "data room folder does not exist: {}",
            root.display()
        ));
    }

    if !root.is_dir() {
        return Err(format!(
            "data room path is not a folder: {}",
            root.display()
        ));
    }

    let search_roots = admin_search_roots(root);
    let is_admin_first_search = !search_roots.is_empty();
    let mut files = Vec::new();

    for search_root in &search_roots {
        collect_matching_files_from_root_with_options(
            root,
            search_root,
            &mut files,
            include_file_data,
        )?;
    }

    if is_admin_first_search && !has_required_source_file_types(&files) {
        collect_matching_files_from_root_with_options(root, root, &mut files, include_file_data)?;
    }

    files.sort_by(|left, right| {
        left.source_file
            .relative_path
            .cmp(&right.source_file.relative_path)
    });
    files.dedup_by(|left, right| left.source_file.path == right.source_file.path);

    Ok(files)
}

fn has_required_source_file_types(files: &[MatchedDealFile]) -> bool {
    has_matched_source_type(files, "SOW") && has_matched_source_type(files, "Project Timeline")
}

fn has_matched_source_type(files: &[MatchedDealFile], source_type: &str) -> bool {
    files.iter().any(|file| {
        file.source_file
            .matched_on
            .iter()
            .any(|match_name| match_name == source_type)
    })
}

fn load_selected_deal_files(
    deal: &Deal,
    input: &ExtractDealQuestionsAndThesisInput,
) -> Result<Vec<MatchedDealFile>, String> {
    let selected_paths = selected_deal_file_paths(input)?;
    let data_room_root = Path::new(&deal.main_data_room_folder);

    selected_paths
        .iter()
        .map(|path| build_selected_matched_file(data_room_root, Path::new(path)))
        .collect()
}

fn selected_deal_file_paths(
    input: &ExtractDealQuestionsAndThesisInput,
) -> Result<Vec<&str>, String> {
    let selected_paths = [
        input.sow_file_path.as_deref().map(str::trim),
        input.project_timeline_file_path.as_deref().map(str::trim),
    ];
    let mut paths = Vec::new();

    for path in selected_paths.into_iter().flatten() {
        if path.is_empty() {
            continue;
        }

        if !paths.contains(&path) {
            paths.push(path);
        }
    }

    Ok(paths)
}

fn build_selected_matched_file(
    data_room_root: &Path,
    path: &Path,
) -> Result<MatchedDealFile, String> {
    if !path.exists() {
        return Err(format!("selected file does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("selected path is not a file: {}", path.display()));
    }

    if is_ignored_source_file(path) {
        return Err(format!(
            "selected file is a temporary or system file: {}",
            path.display()
        ));
    }

    let canonical_root = data_room_root
        .canonicalize()
        .map_err(|err| format!("failed to resolve data room folder: {err}"))?;
    let canonical_path = path
        .canonicalize()
        .map_err(|err| format!("failed to resolve selected file {}: {err}", path.display()))?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err(format!(
            "selected file is outside the deal data room: {}",
            path.display()
        ));
    }

    let metadata = fs::metadata(path)
        .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?;
    if metadata.len() == 0 {
        return Err(format!("selected file is empty: {}", path.display()));
    }

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("failed to derive filename from {}", path.display()))?
        .to_string();
    let mut matched_on = matching_terms(&filename);
    matched_on.sort();
    matched_on.dedup();

    Ok(MatchedDealFile {
        data_base64: encode_supported_file(path).transpose()?,
        mime_type: infer_supported_mime_type(path),
        source_file: DealSourceFile {
            path: path.display().to_string(),
            filename,
            relative_path: display_relative_path(data_room_root, path),
            size_bytes: metadata.len(),
            matched_on,
            text_extracted: false,
            text_truncated: false,
        },
    })
}

fn collect_matching_files_from_root_with_options(
    data_room_root: &Path,
    search_root: &Path,
    files: &mut Vec<MatchedDealFile>,
    include_file_data: bool,
) -> Result<(), String> {
    for entry in WalkDir::new(search_root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if is_ignored_source_file(path) {
            continue;
        }

        let metadata = fs::metadata(path)
            .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?;
        if metadata.len() == 0 {
            continue;
        }

        let relative_path = display_relative_path(data_room_root, path);
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("failed to derive filename from {}", path.display()))?
            .to_string();
        let mut matched_on = matching_terms(&filename);
        matched_on.sort();
        matched_on.dedup();

        if matched_on.is_empty() {
            continue;
        }

        let data_base64 = if include_file_data {
            encode_supported_file(path).transpose()?
        } else {
            None
        };

        files.push(MatchedDealFile {
            data_base64,
            mime_type: infer_supported_mime_type(path),
            source_file: DealSourceFile {
                path: path.display().to_string(),
                filename,
                relative_path,
                size_bytes: metadata.len(),
                matched_on,
                text_extracted: false,
                text_truncated: false,
            },
        });
    }

    Ok(())
}

fn admin_search_roots(root: &Path) -> Vec<std::path::PathBuf> {
    let mut admin_roots = WalkDir::new(root)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_dir())
        .map(|entry| entry.into_path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(is_admin_folder_name)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    admin_roots.sort();
    admin_roots.dedup();

    admin_roots
}

fn is_admin_folder_name(folder_name: &str) -> bool {
    let normalized = folder_name
        .to_ascii_lowercase()
        .replace(['.', '_', '-'], " ");

    normalized
        .split_whitespace()
        .any(|part| part == "admin" || part == "administration")
}

async fn extract_deal_questions_and_thesis_from_files(
    deal: &Deal,
    files: &[MatchedDealFile],
) -> Result<DealExtraction, String> {
    if files.is_empty() {
        return Ok(DealExtraction {
            key_questions: Vec::new(),
            investment_thesis: String::new(),
        });
    }

    let attachable_files = files
        .iter()
        .filter(|file| {
            file.mime_type.is_some()
                && file
                    .data_base64
                    .as_deref()
                    .map(|data| !data.trim().is_empty())
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    if attachable_files.is_empty() {
        return Ok(DealExtraction {
            key_questions: Vec::new(),
            investment_thesis: String::new(),
        });
    }

    let client = OpenAiClient::new()?;
    let model = env::var("OPENAI_DEAL_EXTRACTION_MODEL")
        .unwrap_or_else(|_| DEFAULT_DEAL_EXTRACTION_MODEL.to_string());
    let prompt = build_deal_extraction_prompt(deal, &attachable_files);
    let file_inputs = attachable_files
        .iter()
        .filter_map(|file| {
            Some(ResponsesFileInput::FileData {
                filename: file.source_file.filename.as_str(),
                mime_type: file.mime_type?,
                data_base64: file.data_base64.as_deref()?,
            })
        })
        .collect::<Vec<_>>();
    let response = client
        .gen_model_response_with_files_and_reasoning(
            Some(&prompt),
            Some("You extract private equity diligence outputs from deal documents. Return only strict JSON with no Markdown."),
            Some(&model),
            Some(&file_inputs),
            Some("none"),
        )
        .await?;

    parse_deal_extraction(&response)
}

fn build_deal_extraction_prompt(deal: &Deal, files: &[&MatchedDealFile]) -> String {
    let file_manifest = files
        .iter()
        .map(|file| {
            format!(
                "- {} ({}, {} bytes, matched on: {})",
                file.source_file.relative_path,
                file.mime_type.unwrap_or("unknown"),
                file.source_file.size_bytes,
                file.source_file.matched_on.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let narrative_rule = if deal.deal_type == "Sell-side" {
        let company_name = deal
            .target_company
            .as_deref()
            .filter(|company| !company.trim().is_empty())
            .unwrap_or("the company");
        format!(
            "- investmentThesis must contain a concise equity story for {company_name}. Base it only on the attached files and deal metadata. Focus on the company's buyer-facing value proposition, growth story, differentiation, and reasons a buyer should care.\n"
        )
    } else {
        "- Always return investmentThesis as an empty string.\n".to_string()
    };

    format!(
        "Deal metadata:\n\
Deal name: {}\n\
Deal type: {}\n\
PE firm: {}\n\
Target company: {}\n\
Buyer/platform company: {}\n\
Parent/seller company: {}\n\
Carve-out business: {}\n\n\
Review the attached files listed below. Extract only the questions that are explicitly labeled as key questions in the attached Word document. The relevant section heading must be labeled Key Questions or Key Diligence Questions. Do not create, infer, rewrite, synthesize, or add any key questions of your own.\n\n\
Return strict JSON with exactly these keys: \"keyQuestions\" and \"investmentThesis\".\n\n\
Rules:\n\
- keyQuestions must contain only verbatim or near-verbatim questions found under a Key Questions or Key Diligence Questions label in the attached Word document.\n\
- Do not include questions from unlabeled sections, timelines, risks, assumptions, dependencies, milestones, workstreams, or next steps.\n\
- Do not infer questions from scope or timeline content.\n\
- If no attached Word document has a Key Questions or Key Diligence Questions section, return an empty keyQuestions array.\n\
{}\
- Use the attached files as the source of truth.\n\
- Do not include Markdown, commentary, citations, or extra keys.\n\n\
Attached file manifest:\n{}",
        deal.deal_name,
        deal.deal_type,
        deal.pe_firm,
        deal.target_company.as_deref().unwrap_or(""),
        deal.buyer_or_platform_company.as_deref().unwrap_or(""),
        deal.parent_or_seller_company.as_deref().unwrap_or(""),
        deal.carve_out_business.as_deref().unwrap_or(""),
        narrative_rule,
        file_manifest
    )
}

fn parse_deal_extraction(response: &str) -> Result<DealExtraction, String> {
    let trimmed = response.trim();
    let json_text = trimmed
        .strip_prefix("```json")
        .and_then(|text| text.strip_suffix("```"))
        .or_else(|| {
            trimmed
                .strip_prefix("```")
                .and_then(|text| text.strip_suffix("```"))
        })
        .unwrap_or(trimmed)
        .trim();

    serde_json::from_str::<DealExtraction>(json_text)
        .map_err(|err| format!("failed to parse deal extraction JSON: {err}; response: {response}"))
}

fn encode_supported_file(path: &Path) -> Option<Result<String, String>> {
    infer_supported_mime_type(path)?;

    Some(
        fs::read(path)
            .map(|bytes| general_purpose::STANDARD.encode(bytes))
            .map_err(|err| {
                format!(
                    "failed to read {} for OpenAI request: {err}",
                    path.display()
                )
            }),
    )
}

fn is_ignored_source_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|filename| filename.starts_with("~$"))
        .unwrap_or(false)
}

fn matching_terms(content: &str) -> Vec<String> {
    let haystack = content.to_ascii_lowercase();
    let mut matches = Vec::new();

    if SOW_MATCH_TERMS.iter().any(|term| haystack.contains(term)) {
        matches.push("SOW".to_string());
    }

    if PROJECT_TIMELINE_MATCH_TERMS
        .iter()
        .any(|term| haystack.contains(term))
    {
        matches.push("Project Timeline".to_string());
    }

    matches
}

fn validate_deal_input(input: &SaveDealAndExtractInput) -> Result<(), String> {
    if input.deal_name.trim().is_empty() {
        return Err("dealName is required".to_string());
    }

    if input.main_data_room_folder.trim().is_empty() {
        return Err("mainDataRoomFolder is required".to_string());
    }

    if input.deal_type.trim().is_empty() {
        return Err("dealType is required".to_string());
    }

    if input.pe_firm.trim().is_empty() {
        return Err("peFirm is required".to_string());
    }

    Ok(())
}

fn trim_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::RefCell,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "quarry-deal-service-{name}-{}-{nanos}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write_file(&self, relative_path: &str, content: &[u8]) -> PathBuf {
            let path = self.path.join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&path, content).unwrap();
            path
        }

        fn create_dir(&self, relative_path: &str) -> PathBuf {
            let path = self.path.join(relative_path);
            fs::create_dir_all(&path).unwrap();
            path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn deal_input() -> SaveDealAndExtractInput {
        SaveDealAndExtractInput {
            deal_name: " Project Gamma ".to_string(),
            main_data_room_folder: " /data-room ".to_string(),
            deal_type: " Buy-side ".to_string(),
            pe_firm: " West Monroe Capital ".to_string(),
            target_company: Some(" Target Co ".to_string()),
            buyer_or_platform_company: Some(" Platform Co ".to_string()),
            parent_or_seller_company: Some(" ".to_string()),
            carve_out_business: None,
        }
    }

    fn deal_fixture() -> Deal {
        Deal {
            id: 7,
            deal_name: "Project Gamma".to_string(),
            main_data_room_folder: "/tmp/data-room".to_string(),
            deal_type: "Buy-side".to_string(),
            pe_firm: "West Monroe Capital".to_string(),
            status: "active".to_string(),
            target_company: Some("Target Co".to_string()),
            buyer_or_platform_company: Some("Platform Co".to_string()),
            parent_or_seller_company: None,
            carve_out_business: None,
            created_at: "2026-07-24T00:00:00Z".to_string(),
            updated_at: "2026-07-24T00:00:00Z".to_string(),
        }
    }

    fn selected_file_input(
        sow_file_path: impl Into<String>,
        project_timeline_file_path: impl Into<String>,
    ) -> ExtractDealQuestionsAndThesisInput {
        ExtractDealQuestionsAndThesisInput {
            deal_id: 7,
            sow_file_path: Some(sow_file_path.into()),
            project_timeline_file_path: Some(project_timeline_file_path.into()),
        }
    }

    fn matched_file(relative_path: &str, matched_on: Vec<String>) -> MatchedDealFile {
        MatchedDealFile {
            data_base64: Some("YWJjMTIz".to_string()),
            mime_type: Some("application/pdf"),
            source_file: DealSourceFile {
                path: format!("/tmp/{relative_path}"),
                filename: Path::new(relative_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                relative_path: relative_path.to_string(),
                size_bytes: 123,
                matched_on,
                text_extracted: false,
                text_truncated: false,
            },
        }
    }

    #[test]
    fn validate_deal_input_accepts_required_fields() {
        assert!(validate_deal_input(&deal_input()).is_ok());
    }

    #[test]
    fn validate_deal_input_rejects_missing_required_fields() {
        let mut input = deal_input();
        input.deal_name = " ".to_string();
        assert_eq!(
            validate_deal_input(&input),
            Err("dealName is required".to_string())
        );

        let mut input = deal_input();
        input.main_data_room_folder = " ".to_string();
        assert_eq!(
            validate_deal_input(&input),
            Err("mainDataRoomFolder is required".to_string())
        );

        let mut input = deal_input();
        input.deal_type = " ".to_string();
        assert_eq!(
            validate_deal_input(&input),
            Err("dealType is required".to_string())
        );

        let mut input = deal_input();
        input.pe_firm = " ".to_string();
        assert_eq!(
            validate_deal_input(&input),
            Err("peFirm is required".to_string())
        );
    }

    #[test]
    fn save_deal_with_repository_trims_input_and_uses_mock_repository() {
        let captured = RefCell::new(None);
        let deal = save_deal_with_repository(&deal_input(), |record| {
            captured.replace(Some((
                record.deal_name.to_string(),
                record.main_data_room_folder.to_string(),
                record.deal_type.to_string(),
                record.pe_firm.to_string(),
                record.target_company.map(str::to_string),
                record.buyer_or_platform_company.map(str::to_string),
                record.parent_or_seller_company.map(str::to_string),
                record.carve_out_business.map(str::to_string),
            )));
            Ok(deal_fixture())
        })
        .unwrap();

        assert_eq!(deal.id, 7);
        assert_eq!(
            captured.into_inner().unwrap(),
            (
                "Project Gamma".to_string(),
                "/data-room".to_string(),
                "Buy-side".to_string(),
                "West Monroe Capital".to_string(),
                Some("Target Co".to_string()),
                Some("Platform Co".to_string()),
                None,
                None,
            )
        );
    }

    #[test]
    fn save_deal_with_repository_propagates_repository_errors() {
        let error = save_deal_with_repository(&deal_input(), |_| Err("insert failed".to_string()));

        assert_eq!(error.unwrap_err(), "insert failed");
    }

    #[test]
    fn collect_sow_and_timeline_files_errors_for_missing_or_non_directory_roots() {
        let root = TestDir::new("bad-root");
        let file_path = root.write_file("file.txt", b"hello");

        let missing_error =
            match collect_sow_and_timeline_files_with_options(&root.path().join("missing"), true) {
                Err(error) => error,
                Ok(_) => panic!("expected missing root to return an error"),
            };
        assert!(missing_error.contains("does not exist"));

        let non_directory_error =
            match collect_sow_and_timeline_files_with_options(&file_path, true) {
                Err(error) => error,
                Ok(_) => panic!("expected file root to return an error"),
            };
        assert!(non_directory_error.contains("not a folder"));
    }

    #[test]
    fn collect_sow_and_timeline_files_prefers_admin_matches() {
        let root = TestDir::new("admin-first");
        root.write_file(".01 Admin/Project Timeline.pdf", b"timeline");
        root.write_file("Commercial/SOW.pdf", b"sow");

        let files = collect_sow_and_timeline_files_with_options(root.path(), true).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(
            files[0].source_file.relative_path,
            ".01 Admin/Project Timeline.pdf"
        );
        assert_eq!(files[0].source_file.matched_on, vec!["Project Timeline"]);
        assert_eq!(files[1].source_file.relative_path, "Commercial/SOW.pdf");
        assert_eq!(files[1].source_file.matched_on, vec!["SOW"]);
    }

    #[test]
    fn collect_sow_and_timeline_files_uses_admin_matches_when_both_types_are_found() {
        let root = TestDir::new("admin-both");
        root.write_file(".01 Admin/Project Timeline.pdf", b"timeline");
        root.write_file(".01 Admin/SOW.pdf", b"sow");
        root.write_file("Commercial/SOW.pdf", b"sow");

        let files = collect_sow_and_timeline_files_with_options(root.path(), true).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(
            files[0].source_file.relative_path,
            ".01 Admin/Project Timeline.pdf"
        );
        assert_eq!(files[1].source_file.relative_path, ".01 Admin/SOW.pdf");
    }

    #[test]
    fn collect_sow_and_timeline_files_falls_back_to_root_when_admin_has_no_matches() {
        let root = TestDir::new("admin-fallback");
        root.write_file(".01 Admin/readme.txt", b"admin notes");
        root.write_file("Commercial/SOW.pdf", b"sow");

        let files = collect_sow_and_timeline_files_with_options(root.path(), true).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].source_file.relative_path, "Commercial/SOW.pdf");
        assert_eq!(files[0].source_file.matched_on, vec!["SOW"]);
    }

    #[test]
    fn collect_matching_files_from_root_matches_file_names_only() {
        let root = TestDir::new("filename-only");
        root.write_file("SOW Folder/agenda.txt", b"agenda");
        root.write_file("Admin/Final SOW.txt", b"scope");
        root.write_file("Admin/Project Timeline.txt", b"timeline");
        root.write_file("Admin/empty SOW.txt", b"");

        let mut files = Vec::new();
        collect_matching_files_from_root_with_options(root.path(), root.path(), &mut files, true)
            .unwrap();
        files.sort_by(|left, right| {
            left.source_file
                .relative_path
                .cmp(&right.source_file.relative_path)
        });

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].source_file.relative_path, "Admin/Final SOW.txt");
        assert_eq!(files[0].source_file.matched_on, vec!["SOW"]);
        assert_eq!(files[0].mime_type, Some("text/plain"));
        let encoded_scope = general_purpose::STANDARD.encode(b"scope");
        assert_eq!(
            files[0].data_base64.as_deref(),
            Some(encoded_scope.as_str())
        );
        assert_eq!(
            files[1].source_file.relative_path,
            "Admin/Project Timeline.txt"
        );
        assert_eq!(files[1].source_file.matched_on, vec!["Project Timeline"]);
    }

    #[test]
    fn collect_matching_files_from_root_ignores_office_lock_files() {
        let root = TestDir::new("lock-files");
        root.write_file("Admin/~$ Final SOW.docx", b"lock");
        root.write_file("Admin/Final SOW.txt", b"scope");

        let mut files = Vec::new();
        collect_matching_files_from_root_with_options(root.path(), root.path(), &mut files, false)
            .unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].source_file.relative_path, "Admin/Final SOW.txt");
    }

    #[test]
    fn load_selected_deal_files_encodes_selected_files_and_rejects_outside_paths() {
        let root = TestDir::new("selected-files");
        let sow = root.write_file("Admin/Final SOW.txt", b"scope");
        let timeline = root.write_file("Admin/Project Timeline.txt", b"timeline");
        let outside_root = TestDir::new("selected-files-outside");
        let outside = outside_root.write_file("Project Timeline.txt", b"outside");
        let mut deal = deal_fixture();
        deal.main_data_room_folder = root.path().display().to_string();

        let files = load_selected_deal_files(
            &deal,
            &selected_file_input(sow.display().to_string(), timeline.display().to_string()),
        )
        .unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].source_file.relative_path, "Admin/Final SOW.txt");
        assert_eq!(files[0].source_file.matched_on, vec!["SOW"]);
        assert_eq!(
            files[0].data_base64.as_deref(),
            Some(general_purpose::STANDARD.encode(b"scope").as_str())
        );
        assert_eq!(
            files[1].source_file.relative_path,
            "Admin/Project Timeline.txt"
        );

        let outside_error = match load_selected_deal_files(
            &deal,
            &selected_file_input(sow.display().to_string(), outside.display().to_string()),
        ) {
            Err(error) => error,
            Ok(_) => panic!("expected outside selected file to return an error"),
        };
        assert!(outside_error.contains("outside the deal data room"));
    }

    #[test]
    fn selected_deal_file_paths_ignores_blank_paths_and_dedupes_matches() {
        assert_eq!(
            selected_deal_file_paths(&selected_file_input(" /tmp/SOW.pdf ", " /tmp/SOW.pdf "))
                .unwrap(),
            vec!["/tmp/SOW.pdf"]
        );

        assert_eq!(
            selected_deal_file_paths(&selected_file_input(" ", "/tmp/timeline.pdf")).unwrap(),
            vec!["/tmp/timeline.pdf"]
        );

        assert!(
            selected_deal_file_paths(&ExtractDealQuestionsAndThesisInput {
                deal_id: 7,
                sow_file_path: None,
                project_timeline_file_path: None,
            })
            .unwrap()
            .is_empty()
        );
    }

    #[test]
    fn measure_data_room_counts_non_ignored_files_and_bytes() {
        let root = TestDir::new("data-room-measure");
        root.write_file("Admin/SOW.txt", b"scope");
        root.write_file("Commercial/Timeline.txt", b"timeline");
        root.write_file("Admin/~$ Draft SOW.docx", b"lock");

        let (document_count, data_room_size_bytes) = measure_data_room(root.path()).unwrap();

        assert_eq!(document_count, 2);
        assert_eq!(data_room_size_bytes, 13);
    }

    #[test]
    fn admin_search_roots_finds_admin_like_folders_within_depth_limit() {
        let root = TestDir::new("admin-roots");
        let admin = root.create_dir(".01 Admin");
        let nested_admin = root.create_dir("A/B/Administration");
        root.create_dir("A/B/C/Admin");
        root.create_dir("Commercial");

        let roots = admin_search_roots(root.path());

        assert_eq!(roots, vec![admin, nested_admin]);
    }

    #[test]
    fn is_admin_folder_name_accepts_numbered_and_delimited_admin_names() {
        assert!(is_admin_folder_name(".01 Admin"));
        assert!(is_admin_folder_name("02_Administration"));
        assert!(is_admin_folder_name("03-admin"));
        assert!(!is_admin_folder_name("Commercial"));
    }

    #[test]
    fn extract_deal_questions_and_thesis_returns_empty_without_files() {
        let extraction = tokio_test_block_on(extract_deal_questions_and_thesis_from_files(
            &deal_fixture(),
            &[],
        ))
        .unwrap();

        assert!(extraction.key_questions.is_empty());
        assert_eq!(extraction.investment_thesis, "");
    }

    #[test]
    fn extract_deal_questions_and_thesis_returns_empty_without_attachable_files() {
        let files = [MatchedDealFile {
            source_file: DealSourceFile {
                path: "/tmp/Final SOW.unsupported".to_string(),
                filename: "Final SOW.unsupported".to_string(),
                relative_path: "Final SOW.unsupported".to_string(),
                size_bytes: 10,
                matched_on: vec!["SOW".to_string()],
                text_extracted: false,
                text_truncated: false,
            },
            data_base64: None,
            mime_type: None,
        }];

        let extraction = tokio_test_block_on(extract_deal_questions_and_thesis_from_files(
            &deal_fixture(),
            &files,
        ))
        .unwrap();

        assert!(extraction.key_questions.is_empty());
        assert_eq!(extraction.investment_thesis, "");
    }

    #[test]
    fn build_deal_extraction_prompt_includes_metadata_manifest_and_date_rule() {
        let files = [
            matched_file("Admin/SOW v1.pdf", vec!["SOW".to_string()]),
            matched_file(
                "Admin/Project Timeline v2.pdf",
                vec!["Project Timeline".to_string()],
            ),
        ];
        let file_refs = files.iter().collect::<Vec<_>>();

        let prompt = build_deal_extraction_prompt(&deal_fixture(), &file_refs);

        assert!(prompt.contains("Deal name: Project Gamma"));
        assert!(prompt.contains("Target company: Target Co"));
        assert!(prompt.contains("Admin/SOW v1.pdf"));
        assert!(prompt.contains("Admin/Project Timeline v2.pdf"));
        assert!(prompt.contains("explicitly labeled as key questions"));
        assert!(
            prompt.contains("Do not create, infer, rewrite, synthesize, or add any key questions")
        );
        assert!(prompt.contains("Always return investmentThesis as an empty string"));
        assert!(prompt.contains("\"keyQuestions\""));
        assert!(prompt.contains("\"investmentThesis\""));
    }

    #[test]
    fn build_deal_extraction_prompt_asks_for_equity_story_on_sell_side() {
        let files = [matched_file("Admin/SOW v1.pdf", vec!["SOW".to_string()])];
        let file_refs = files.iter().collect::<Vec<_>>();
        let mut deal = deal_fixture();
        deal.deal_type = "Sell-side".to_string();

        let prompt = build_deal_extraction_prompt(&deal, &file_refs);

        assert!(
            prompt.contains("investmentThesis must contain a concise equity story for Target Co")
        );
        assert!(prompt.contains("buyer-facing value proposition"));
        assert!(!prompt.contains("Always return investmentThesis as an empty string"));
    }

    #[test]
    fn parse_deal_extraction_parses_raw_and_fenced_json() {
        let raw = parse_deal_extraction(
            r#"{"keyQuestions":["What is the implementation risk?"],"investmentThesis":"Strong target."}"#,
        )
        .unwrap();
        assert_eq!(raw.key_questions, vec!["What is the implementation risk?"]);
        assert_eq!(raw.investment_thesis, "Strong target.");

        let fenced = parse_deal_extraction(
            "```json\n{\"keyQuestions\":[\"What changed?\"],\"investmentThesis\":\"Updated case.\"}\n```",
        )
        .unwrap();
        assert_eq!(fenced.key_questions, vec!["What changed?"]);
        assert_eq!(fenced.investment_thesis, "Updated case.");
    }

    #[test]
    fn parse_deal_extraction_errors_for_invalid_json() {
        let error = parse_deal_extraction("not json").unwrap_err();

        assert!(error.contains("failed to parse deal extraction JSON"));
    }

    #[test]
    fn encode_supported_file_base64_encodes_supported_files_and_skips_unsupported_files() {
        let root = TestDir::new("encode");
        let supported = root.write_file("SOW.txt", b"scope");
        let unsupported = root.write_file("SOW.bin", b"scope");

        assert_eq!(
            encode_supported_file(&supported).unwrap().unwrap(),
            general_purpose::STANDARD.encode(b"scope")
        );
        assert!(encode_supported_file(&unsupported).is_none());
    }

    #[test]
    fn matching_terms_detects_case_insensitive_sow_and_project_timeline() {
        assert_eq!(matching_terms("final SOW.pdf"), vec!["SOW"]);
        assert_eq!(matching_terms("scope of work.docx"), vec!["SOW"]);
        assert_eq!(
            matching_terms("PROJECT TIMELINE v2.xlsx"),
            vec!["Project Timeline"]
        );
        assert_eq!(
            matching_terms("Diligence Schedule.xlsx"),
            vec!["Project Timeline"]
        );
        assert_eq!(
            matching_terms("Project Plan.xlsx"),
            vec!["Project Timeline"]
        );
        assert_eq!(matching_terms("Workplan.pdf"), vec!["Project Timeline"]);
        assert_eq!(
            matching_terms("SOW - Project Timeline.pdf"),
            vec!["SOW", "Project Timeline"]
        );
        assert!(matching_terms("Commercial model.pdf").is_empty());
    }

    #[test]
    fn trim_optional_trims_values_and_removes_blanks() {
        assert_eq!(trim_optional(Some("  Target Co  ")), Some("Target Co"));
        assert_eq!(trim_optional(Some("   ")), None);
        assert_eq!(trim_optional(None), None);
    }

    fn tokio_test_block_on<T>(future: impl std::future::Future<Output = T>) -> T {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }
}
