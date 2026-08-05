use super::*;

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
    let fixture =
        Path::new("/Users/rgambhir/BetaNXT/02 - Data Room (CIM, Target Docs)/List of Items.docx");
    if !fixture.is_file() || find_soffice().is_none() {
        return;
    }

    let bytes = convert_office_to_pdf(fixture).expect("DOCX fixture should convert to PDF");
    assert!(bytes.starts_with(b"%PDF-"));
}
