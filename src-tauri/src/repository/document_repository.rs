use serde_json::Value;

use crate::core::{
    clients::helix::HelixClient,
    helix_queries::files::insert_quarry_file::{
        create_document_indexes, insert_chunk_for_document, insert_quarry_file,
    },
    nodes::document_node::{ChunkNode, DocumentNode},
};

#[derive(Debug)]
pub struct PersistedDocumentGraph {
    pub quarry_file: Value,
    pub chunks: Vec<Value>,
}

/// Creates the QuarryFile node once before chunks are added.
pub async fn persist_quarry_file(
    helix: &HelixClient,
    document: DocumentNode,
) -> Result<Value, String> {
    let query = insert_quarry_file(document)?;

    helix.execute_dynamic_query(move || query).await
}

/// Adds one chunk to an existing QuarryFile.
///
/// The Helix query resolves the original node using both `document_id` and
/// `user_id`; it creates neither the chunk nor the edge when no match exists.
pub async fn persist_chunk_for_document(
    helix: &HelixClient,
    chunk: ChunkNode,
) -> Result<Value, String> {
    let query = insert_chunk_for_document(chunk)?;

    helix.execute_dynamic_query(move || query).await
}

/// Sequentially adds all supplied chunks to an already-persisted QuarryFile.
///
/// Each chunk is its own Helix transaction so callers can use this while chunks
/// are produced. Processing stops on the first failed request.
pub async fn persist_chunks_for_document(
    helix: &HelixClient,
    document: &DocumentNode,
    chunks: Vec<ChunkNode>,
) -> Result<Vec<Value>, String> {
    let mut persisted = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        validate_document_chunk_relationship(document, &chunk)?;
        persisted.push(persist_chunk_for_document(helix, chunk).await?);
    }

    Ok(persisted)
}

/// Creates the QuarryFile once and then adds each chunk as it is processed.
pub async fn persist_document_and_chunks(
    helix: &HelixClient,
    document: DocumentNode,
    chunks: Vec<ChunkNode>,
) -> Result<PersistedDocumentGraph, String> {
    for chunk in &chunks {
        validate_document_chunk_relationship(&document, chunk)?;
    }

    let quarry_file = persist_quarry_file(helix, document.clone()).await?;
    let chunks = persist_chunks_for_document(helix, &document, chunks).await?;

    Ok(PersistedDocumentGraph {
        quarry_file,
        chunks,
    })
}

/// Compatibility wrapper for callers that currently persist the first chunk
/// together with its document.
pub async fn persist_document_and_chunk(
    helix: &HelixClient,
    document: DocumentNode,
    chunk: ChunkNode,
) -> Result<PersistedDocumentGraph, String> {
    persist_document_and_chunks(helix, document, vec![chunk]).await
}

/// Executes the idempotent QuarryFile and Chunk index batch.
pub async fn ensure_document_indexes(helix: &HelixClient) -> Result<Value, String> {
    helix.execute_dynamic_query(create_document_indexes).await
}

fn validate_document_chunk_relationship(
    document: &DocumentNode,
    chunk: &ChunkNode,
) -> Result<(), String> {
    if chunk.document_id != document.document_id {
        return Err(format!(
            "chunk `{}` belongs to document `{}`, not `{}`",
            chunk.chunk_id, chunk.document_id, document.document_id
        ));
    }
    if chunk.user_id != document.user_id {
        return Err(format!(
            "chunk `{}` belongs to user `{}`, not `{}`",
            chunk.chunk_id, chunk.user_id, document.user_id
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../tests/repository/document_repository_tests.rs"]
mod tests;
