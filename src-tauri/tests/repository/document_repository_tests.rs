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
