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
