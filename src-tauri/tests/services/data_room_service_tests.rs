use super::*;

const ACTUAL_PDF_RELATIVE_PATH: &str = "4 Security and Compliance/4.1 Cybersecurity/4.1.2 Cybersecurity testing and remediation/BetaNXT Standard - Application Security Testing.pdf";

#[test]
fn every_mock_deal_has_a_configured_data_room_root() {
    for deal_id in ["project-alpha", "project-beta", "logistics-merger"] {
        assert!(deal_data_room_root(deal_id).is_some());
    }
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
