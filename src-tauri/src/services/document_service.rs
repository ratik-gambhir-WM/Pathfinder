use crate::clients::openai::{OpenAiClient, ResponsesFileInput};
use crate::common::{
    build_summary_prompt, display_relative_path, infer_supported_mime_type, CollectedFile,
};
use crate::models::document::{ParsedFileData, ParsedFileData2};
use crate::parsers::gen_parsed_file;
use crate::parsers::TextChunk;
use crate::prompts::{
    DATA_ROOM_TECH_DILIGENCE_SUMMARY_PROMPT, DOCUMENT_SUMMARY_SYSTEM_PROMPT,
    PRODUCT_AND_APPLICATION_DEEP_DIVE_PROMPT,
};
use crate::utils::openai_api_key;
use base64::engine::general_purpose;
use base64::Engine;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

pub enum ParsedFile {
    Docx(ParsedFileData),
    PPTX(ParsedFileData),
    Spreadsheet(ParsedFileData),
    Image(ParsedFileData),
}

pub enum DirectoryFile {
    Docx(ParsedFileData),
    PPTX(ParsedFileData),
    Spreadsheet(ParsedFileData),
    Image(ParsedFileData),
}

const DEFAULT_DOCUMENT_SUMMARY_MODEL: &str = "gpt-5.5";
const MAX_FILE_BYTES: usize = 50 * 1024 * 1024;
const MAX_TOTAL_REQUEST_FILE_BYTES: usize = 50 * 1024 * 1024;

pub fn parse_docx_file(path: &Path) -> Result<Vec<TextChunk>, String> {
    crate::parsers::docx::parse_docx_file(path)
}

pub async fn summarize_dir(path: String) -> Result<String, String> {
    let root = PathBuf::from(&path);
    if !root.exists() {
        return Err(format!("path does not exist: {}", root.display()));
    }
    if !root.is_dir() {
        return Err(format!("path is not a directory: {}", root.display()));
    }
    let api_key = openai_api_key()?;
    let client = OpenAiClient::new(&api_key);
    let model = env::var("OPENAI_DOCUMENT_SUMMARY_MODEL")
        .unwrap_or_else(|_| DEFAULT_DOCUMENT_SUMMARY_MODEL.to_string());
    let (files, skipped_files) = collect_dir_content(&root)?;
    let prompt = build_document_summary_prompt(&root, &files, &skipped_files);
    let file_inputs: Vec<ResponsesFileInput<'_>> = files
        .iter()
        .map(|file| ResponsesFileInput::FileData {
            filename: file.filename.as_str(),
            mime_type: &*file.mime_type,
            data_base64: file.data_base64.as_str(),
        })
        .collect();

    let summary = client
        .gen_model_response_with_files(
            Some(&prompt),
            Some(DOCUMENT_SUMMARY_SYSTEM_PROMPT),
            Some(&model),
            Some(&file_inputs),
        )
        .await?;

    println!("{summary}");
    // write_summary(&summary)?;

    if !skipped_files.is_empty() {
        eprintln!("skipped {} unsupported or empty files", skipped_files.len());
    }

    Ok(summary)
}

fn build_document_summary_prompt(
    root: &Path,
    files: &Vec<CollectedFile>,
    skipped_files: &[String],
) -> String {
    format!(
        "{}\n\n{}\n\n{}",
        build_summary_prompt(root, files, skipped_files),
        DATA_ROOM_TECH_DILIGENCE_SUMMARY_PROMPT.trim(),
        PRODUCT_AND_APPLICATION_DEEP_DIVE_PROMPT.trim()
    )
}

fn collect_dir_content(root: &Path) -> Result<(Vec<CollectedFile>, Vec<String>), String> {
    let mut files = Vec::new();
    let mut skipped_files = Vec::new();
    let mut total_file_bytes = 0usize;
    let mut total_limit_reached = false;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        let relative_path = display_relative_path(root, &path);
        let Some(mime_type) = infer_supported_mime_type(&path) else {
            skipped_files.push(relative_path);
            continue;
        };
        println!("{}", path.display());

        if total_limit_reached {
            skipped_files.push(format!(
                "{relative_path} (skipped: total request file size limit already reached)"
            ));
            continue;
        }

        let file_size_bytes = fs::metadata(&path)
            .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?
            .len() as usize;
        if file_size_bytes == 0 {
            skipped_files.push(format!("{relative_path} (empty)"));
            continue;
        }

        if file_size_bytes > MAX_FILE_BYTES {
            skipped_files.push(format!(
                "{relative_path} (skipped: file exceeds 50 MB limit)"
            ));
            continue;
        }

        if total_file_bytes + file_size_bytes > MAX_TOTAL_REQUEST_FILE_BYTES {
            skipped_files.push(format!(
                "{relative_path} (skipped: total request file size would exceed 50 MB limit)"
            ));
            total_limit_reached = true;
            continue;
        }

        let file_bytes =
            fs::read(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        if file_bytes.is_empty() {
            skipped_files.push(format!("{relative_path} (empty)"));
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("failed to derive filename from {}", path.display()))?
            .to_string();

        files.push(CollectedFile {
            filename,
            relative_path,
            mime_type,
            size_bytes: file_size_bytes,
            data_base64: general_purpose::STANDARD.encode(file_bytes),
        });
        total_file_bytes += file_size_bytes;
    }

    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    skipped_files.sort();

    Ok((files, skipped_files))
}

fn collect_supported_files(
    root: &Path,
    to_chunk: Option<bool>,
) -> Result<(Vec<ParsedFileData2>, Vec<String>), String> {
    let mut files: Vec<ParsedFileData2> = Vec::new();
    let mut skipped_files: Vec<String> = Vec::new();
    let mut total_file_bytes = 0usize;
    let mut total_limit_reached = false;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let str = path.to_str().unwrap();

        if total_limit_reached {
            skipped_files.push(format!(
                "{str} (skipped: total request file size limit already reached)"
            ));
            continue;
        }

        let file_size_bytes = fs::metadata(&path)
            .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?
            .len() as usize;
        if file_size_bytes == 0 {
            skipped_files.push(format!("{str} (empty)"));
            continue;
        }

        if file_size_bytes > MAX_FILE_BYTES {
            skipped_files.push(format!("{str} (skipped: file exceeds 50 MB limit)"));
            continue;
        }

        if total_file_bytes + file_size_bytes > MAX_TOTAL_REQUEST_FILE_BYTES {
            skipped_files.push(format!(
                "{str} (skipped: total request file size would exceed 50 MB limit)"
            ));
            total_limit_reached = true;
            continue;
        }

        let parsed_file = gen_parsed_file(entry, to_chunk);
        files.push(parsed_file);
        total_file_bytes += file_size_bytes;
    }
    Ok((files, skipped_files))
}
