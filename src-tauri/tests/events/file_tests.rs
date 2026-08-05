use super::*;

#[test]
fn process_files_payload_uses_frontend_field_names() {
    let payload = r#"{
        "requestId": "request-123",
        "paths": ["/documents/report.pdf", "/documents/model.xlsx"]
    }"#;

    let request = serde_json::from_str::<ProcessFilesEvent>(payload).unwrap();

    assert_eq!(request.request_id, "request-123");
    assert_eq!(
        request.paths,
        vec!["/documents/report.pdf", "/documents/model.xlsx"]
    );
}

#[test]
fn emitted_file_payload_serializes_with_frontend_field_names() {
    let event = FileProcessedEvent {
        request_id: "request-123".to_string(),
        path: "/documents/report.pdf".to_string(),
        file_id: Some("/documents/report.pdf".to_string()),
        completed: 1,
        total: 2,
        success: true,
        error: None,
    };

    let json = serde_json::to_value(event).unwrap();

    assert_eq!(json["requestId"], "request-123");
    assert_eq!(json["fileId"], "/documents/report.pdf");
    assert_eq!(json["completed"], 1);
    assert_eq!(json["total"], 2);
    assert_eq!(json["success"], true);
    assert!(json["error"].is_null());
}
