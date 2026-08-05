use super::*;

#[test]
fn command_context_preserves_success_values() {
    assert_eq!(Ok::<_, String>(42).command_context("example"), Ok(42));
}

#[test]
fn command_context_prefixes_errors_with_command_name() {
    assert_eq!(
        Err::<(), _>("bad input").command_context("example"),
        Err("example failed: bad input".to_string())
    );
}
