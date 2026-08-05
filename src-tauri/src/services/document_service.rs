use std::{env, path::Path};

use crate::core::parsers::pdf::parse_pdf_document as parse_pdf_document_from_parser;
pub use crate::core::parsers::pdf::PdfDocumentAssembly;
use crate::core::parsers::{ParsedQuarryFile, QuarryFile};
use crate::core::{
    clients::{helix::HelixClient, openai::OpenAiClient},
    nodes::document_node::{ChunkNode, DocumentNode},
};
use crate::events::file::{
    FileBatchFinishedEvent, FileProcessedEvent, ProcessFilesEvent, FILE_BATCH_FINISHED_EVENT,
    FILE_PROCESSED_EVENT,
};
use crate::repository::document_repository::{
    ensure_document_indexes, persist_chunks_for_document, persist_quarry_file,
};
use crate::state::AppState;
use tauri::{AppHandle, Emitter, Manager, Runtime};

pub async fn process_files<R: Runtime>(app: AppHandle<R>, request: ProcessFilesEvent) {
    let total = request.paths.len();
    let mut succeeded = 0;
    let state = app.state::<AppState>();
    let helix = state.gen_helix_db_client();
    let pipeline = initialize_document_pipeline(helix).await;

    for (index, path) in request.paths.into_iter().enumerate() {
        let result = match &pipeline {
            Ok(openai) => process_file(&path, openai, helix).await,
            Err(error) => Err(error.clone()),
        };
        let (file_id, error) = match result {
            Ok(file_id) => {
                succeeded += 1;
                (Some(file_id), None)
            }
            Err(error) => (None, Some(error)),
        };

        let response = FileProcessedEvent {
            request_id: request.request_id.clone(),
            path,
            file_id,
            completed: index + 1,
            total,
            success: error.is_none(),
            error,
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

async fn initialize_document_pipeline(helix: &HelixClient) -> Result<OpenAiClient, String> {
    let openai = OpenAiClient::new()?;
    ensure_document_indexes(helix)
        .await
        .map_err(|error| format!("failed to initialize Helix document indexes: {error}"))?;
    Ok(openai)
}

async fn process_file(
    path: &str,
    openai: &OpenAiClient,
    helix: &HelixClient,
) -> Result<String, String> {
    let file = QuarryFile::from_local_path(path)?;
    let parsed = file.parse().await?;
    let (document, mut chunks) = document_graph_parts(parsed);

    embed_chunks(Path::new(path), &mut chunks, openai).await?;
    persist_quarry_file(helix, document.clone())
        .await
        .map_err(|error| format!("failed to persist document `{path}` in Helix: {error}"))?;
    persist_chunks_for_document(helix, &document, chunks)
        .await
        .map_err(|error| format!("failed to persist chunks for `{path}` in Helix: {error}"))?;

    Ok(path.to_string())
}

fn document_graph_parts(parsed: ParsedQuarryFile) -> (DocumentNode, Vec<ChunkNode>) {
    match parsed {
        ParsedQuarryFile::Pdf(assembly) => (assembly.document, assembly.chunks),
        ParsedQuarryFile::Docx(assembly) => (assembly.document, assembly.chunks),
    }
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
    embed_chunks(path, &mut assembly.chunks, openai_client).await?;
    Ok(assembly)
}

async fn embed_chunks(
    path: &Path,
    chunks: &mut [ChunkNode],
    openai_client: &OpenAiClient,
) -> Result<(), String> {
    if chunks.is_empty() {
        return Ok(());
    }

    let contents = chunks
        .iter()
        .map(|chunk| chunk.text.as_str())
        .collect::<Vec<_>>();
    let model = env::var("OPENAI_EMBEDDING_MODEL").ok();
    let embeddings = openai_client
        .gen_embeddings(&contents, model.as_deref())
        .await
        .map_err(|err| format!("failed to embed chunks for {}: {err}", path.display()))?;

    attach_embeddings_to_chunks(chunks, embeddings)
}

fn attach_chunk_embeddings(
    assembly: &mut PdfDocumentAssembly,
    embeddings: Vec<Vec<f64>>,
) -> Result<(), String> {
    attach_embeddings_to_chunks(&mut assembly.chunks, embeddings)
}

fn attach_embeddings_to_chunks(
    chunks: &mut [ChunkNode],
    embeddings: Vec<Vec<f64>>,
) -> Result<(), String> {
    if chunks.len() != embeddings.len() {
        return Err(format!(
            "OpenAI returned {} embeddings for {} PDF chunks",
            embeddings.len(),
            chunks.len()
        ));
    }

    for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
        chunk.embedding = Some(embedding.into_iter().map(|value| value as f32).collect());
    }

    Ok(())
}

#[cfg(test)]
#[path = "../../tests/services/document_service_tests.rs"]
mod tests;
