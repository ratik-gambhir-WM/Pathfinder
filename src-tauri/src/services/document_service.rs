use std::{env, path::Path};

use crate::core::clients::openai::OpenAiClient;
use crate::core::parsers::pdf::parse_pdf_document as parse_pdf_document_from_parser;
pub use crate::core::parsers::pdf::PdfDocumentAssembly;
use crate::core::parsers::QuarryFile;
use crate::events::file::{
    FileBatchFinishedEvent, FileProcessedEvent, ProcessFilesEvent, FILE_BATCH_FINISHED_EVENT,
    FILE_PROCESSED_EVENT,
};
use tauri::{AppHandle, Emitter, Runtime};

pub async fn process_files<R: Runtime>(app: AppHandle<R>, request: ProcessFilesEvent) {
    let total = request.paths.len();
    let mut succeeded = 0;

    for (index, path) in request.paths.into_iter().enumerate() {
        let result = process_file(&path).await;
        if result.is_ok() {
            succeeded += 1;
        }

        let response = FileProcessedEvent {
            request_id: request.request_id.clone(),
            path,
            completed: index + 1,
            total,
            success: result.is_ok(),
            error: result.err(),
        };

        if let Err(error) = app.emit(FILE_PROCESSED_EVENT, response) {
            eprintln!("failed to emit {FILE_PROCESSED_EVENT}: {error}");
        }
    }

    let response = FileBatchFinishedEvent {
        request_id: request.request_id,
        total,
        succeeded,
        failed: total - succeeded,
    };

    if let Err(error) = app.emit(FILE_BATCH_FINISHED_EVENT, response) {
        eprintln!("failed to emit {FILE_BATCH_FINISHED_EVENT}: {error}");
    }
}

pub(crate) async fn process_file(path: &str) -> Result<(), String> {
    let file = QuarryFile::from_path(path)?;
    file.parse().await.map(|_| ())
}

/// Parses a PDF and bulk-embeds its chunks for graph storage.
pub async fn parse_pdf_document(
    path: &Path,
    user_id: impl Into<String>,
) -> Result<PdfDocumentAssembly, String> {
    let assembly = parse_pdf_document_from_parser(path, user_id)?;
    if assembly.chunks.is_empty() {
        return Ok(assembly);
    }

    let openai_client = OpenAiClient::new()?;
    embed_pdf_chunks(path, assembly, &openai_client).await
}

async fn embed_pdf_chunks(
    path: &Path,
    mut assembly: PdfDocumentAssembly,
    openai_client: &OpenAiClient,
) -> Result<PdfDocumentAssembly, String> {
    let contents = assembly
        .chunks
        .iter()
        .map(|chunk| chunk.text.as_str())
        .collect::<Vec<_>>();
    let model = env::var("OPENAI_EMBEDDING_MODEL").ok();
    let embeddings = openai_client
        .gen_embeddings(&contents, model.as_deref())
        .await
        .map_err(|err| format!("failed to embed PDF chunks for {}: {err}", path.display()))?;

    attach_chunk_embeddings(&mut assembly, embeddings)?;
    Ok(assembly)
}

fn attach_chunk_embeddings(
    assembly: &mut PdfDocumentAssembly,
    embeddings: Vec<Vec<f64>>,
) -> Result<(), String> {
    if assembly.chunks.len() != embeddings.len() {
        return Err(format!(
            "OpenAI returned {} embeddings for {} PDF chunks",
            embeddings.len(),
            assembly.chunks.len()
        ));
    }

    for (chunk, embedding) in assembly.chunks.iter_mut().zip(embeddings) {
        chunk.embedding = Some(embedding.into_iter().map(|value| value as f32).collect());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use super::*;
    use crate::core::nodes::document_node::{ChunkNode, DocumentNode};

    const MANUAL_PDF_ENV: &str = "QUARRY_MANUAL_PDF_PATH";
    const MANUAL_PDF_NAME: &str = "BetaNXT Standard - Cloud Security.pdf";
    const LOCAL_MANUAL_PDF_PATH: &str = "/Users/rgambhir/BetaNXT Standard - Cloud Security.pdf";
    const MANUAL_OUTPUT_NAME: &str = "betanxt_pdf_assembly_with_embeddings.json";

    #[tokio::test]
    #[ignore = "manual test; requires the PDF, OPENAI_API_KEY, and network access"]
    async fn writes_betanxt_pdf_assembly_with_embeddings_as_pretty_json() {
        let Some(path) = manual_pdf_path() else {
            eprintln!(
                "SKIPPED: {MANUAL_PDF_NAME} was not found. Set {MANUAL_PDF_ENV} to its full path \
                 and rerun:\n  {MANUAL_PDF_ENV}=\"/path/to/{MANUAL_PDF_NAME}\" cargo test \
                 services::document_service::tests::writes_betanxt_pdf_assembly_with_embeddings_as_pretty_json \
                 -- --ignored --nocapture"
            );
            return;
        };

        assert!(
            path.is_file(),
            "{MANUAL_PDF_ENV} does not point to an accessible file: {}",
            path.display()
        );

        let assembly = parse_pdf_document(&path, "manual-inspection-user")
            .await
            .unwrap_or_else(|err| panic!("failed to assemble {}: {err}", path.display()));
        assert!(
            !assembly.chunks.is_empty(),
            "{} should produce at least one chunk",
            path.display()
        );
        assert!(
            assembly.chunks.iter().all(|chunk| chunk
                .embedding
                .as_ref()
                .is_some_and(|embedding| !embedding.is_empty())),
            "every PDF chunk should include a non-empty embedding"
        );

        let json = serde_json::to_string_pretty(&assembly)
            .expect("PDF document assembly should serialize as JSON");
        let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("manual-test-output");
        fs::create_dir_all(&output_dir).unwrap_or_else(|err| {
            panic!(
                "failed to create manual test output directory {}: {err}",
                output_dir.display()
            )
        });
        let output_path = output_dir.join(MANUAL_OUTPUT_NAME);
        fs::write(&output_path, json).unwrap_or_else(|err| {
            panic!(
                "failed to write embedded PDF assembly to {}: {err}",
                output_path.display()
            )
        });

        println!("wrote embedded PDF assembly to {}", output_path.display());
    }

    fn manual_pdf_path() -> Option<PathBuf> {
        if let Some(path) = env::var_os(MANUAL_PDF_ENV) {
            return Some(PathBuf::from(path));
        }

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        [
            PathBuf::from(LOCAL_MANUAL_PDF_PATH),
            manifest_dir.join(MANUAL_PDF_NAME),
            manifest_dir.join("tests").join(MANUAL_PDF_NAME),
            manifest_dir
                .parent()
                .map(|root| root.join(MANUAL_PDF_NAME))
                .unwrap_or_default(),
        ]
        .into_iter()
        .find(|path| path.is_file())
    }

    #[test]
    fn attaches_embeddings_to_existing_chunk_nodes_in_order() {
        let mut assembly = PdfDocumentAssembly {
            document: DocumentNode {
                document_id: "document-1".to_string(),
                user_id: "user-1".to_string(),
                file_name: "report.pdf".to_string(),
                source_type: "pdf".to_string(),
                local_path: Some("/documents/report.pdf".to_string()),
                file_size_bytes: 1_024,
                token_count: 5,
                content_hash: "492053dbb30e118c67ec2c4a0c9ef0968f970fd8f13239e82ca50ad4f4bd2134"
                    .to_string(),
                rendered_pdf_path: Some("/documents/report.pdf".to_string()),
            },
            chunks: vec![ChunkNode {
                chunk_id: "chunk-1".to_string(),
                document_id: "document-1".to_string(),
                user_id: "user-1".to_string(),
                text: "A short PDF page.".to_string(),
                embedding: None,
                sequence_number: 1,
                page_numbers: Some(vec![1]),
                start_offset: 0,
                end_offset: 17,
                token_count: 5,
                content_hash: "content-hash".to_string(),
                section_title: None,
            }],
        };
        let embeddings = vec![vec![0.25, -0.5, 0.75]];

        attach_chunk_embeddings(&mut assembly, embeddings.clone()).unwrap();

        assert_eq!(assembly.chunks[0].embedding, Some(vec![0.25, -0.5, 0.75]));
    }

    #[test]
    fn rejects_embedding_count_mismatches_without_partially_mutating_chunks() {
        let mut assembly = PdfDocumentAssembly {
            document: DocumentNode {
                document_id: "document-1".to_string(),
                user_id: "user-1".to_string(),
                file_name: "report.pdf".to_string(),
                source_type: "pdf".to_string(),
                local_path: Some("/documents/report.pdf".to_string()),
                file_size_bytes: 1_024,
                token_count: 0,
                content_hash: "492053dbb30e118c67ec2c4a0c9ef0968f970fd8f13239e82ca50ad4f4bd2134"
                    .to_string(),
                rendered_pdf_path: None,
            },
            chunks: Vec::new(),
        };

        let error = attach_chunk_embeddings(&mut assembly, vec![vec![0.25]]).unwrap_err();

        assert_eq!(error, "OpenAI returned 1 embeddings for 0 PDF chunks");
        assert!(assembly.chunks.is_empty());
    }
}
