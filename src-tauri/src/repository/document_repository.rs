use serde_json::Value;

use crate::core::{
    clients::helix::HelixClient,
    helix_queries::insert_quarry_file::{
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
mod tests {
    use super::*;

    fn document() -> DocumentNode {
        DocumentNode {
            document_id: "document-1".to_string(),
            user_id: "user-1".to_string(),
            file_name: "report.pdf".to_string(),
            source_type: "pdf".to_string(),
            local_path: Some("/documents/report.pdf".to_string()),
            file_size_bytes: 1_024,
            token_count: 250,
            content_hash: "2f23a7c94f06e9e82af01399b51f15f87b8f8786b743b3c1e01c71bc7654c70a"
                .to_string(),
            rendered_pdf_path: Some("/documents/report.pdf".to_string()),
        }
    }

    fn chunk(chunk_id: &str, sequence_number: u32) -> ChunkNode {
        ChunkNode {
            chunk_id: chunk_id.to_string(),
            document_id: "document-1".to_string(),
            user_id: "user-1".to_string(),
            text: format!("Text for {chunk_id}."),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            sequence_number,
            page_numbers: Some(vec![sequence_number]),
            start_offset: 0,
            end_offset: 10,
            token_count: 3,
            content_hash: format!("hash-{chunk_id}"),
            section_title: None,
        }
    }

    #[test]
    fn relationship_validation_accepts_multiple_chunks_for_one_document() {
        let document = document();

        for chunk in [chunk("chunk-1", 1), chunk("chunk-2", 2)] {
            validate_document_chunk_relationship(&document, &chunk).unwrap();
        }
    }

    #[test]
    fn relationship_validation_rejects_mismatched_document_or_user() {
        let document = document();
        let mut mismatched_document_chunk = chunk("chunk-1", 1);
        mismatched_document_chunk.document_id = "document-2".to_string();
        assert!(
            validate_document_chunk_relationship(&document, &mismatched_document_chunk)
                .unwrap_err()
                .contains("document-2")
        );

        let mut mismatched_user_chunk = chunk("chunk-2", 2);
        mismatched_user_chunk.user_id = "user-2".to_string();
        assert!(
            validate_document_chunk_relationship(&document, &mismatched_user_chunk)
                .unwrap_err()
                .contains("user-2")
        );
    }
}
