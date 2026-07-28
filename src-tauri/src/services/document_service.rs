use std::path::Path;

pub use crate::core::parsers::pdf::PdfDocumentAssembly;

/// Compatibility entry point for the parser-owned PDF assembly workflow.
pub fn assemble_pdf_document(
    path: &Path,
    user_id: impl Into<String>,
) -> Result<PdfDocumentAssembly, String> {
    crate::core::parsers::pdf::assemble_pdf_document(path, user_id)
}



#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::*;

    const MANUAL_PDF_ENV: &str = "QUARRY_MANUAL_PDF_PATH";
    const MANUAL_PDF_NAME: &str = "BetaNXT Standard - Cloud Security.pdf";
    const LOCAL_MANUAL_PDF_PATH: &str = "/Users/rgambhir/BetaNXT Standard - Cloud Security.pdf";

    #[test]
    #[ignore = "manual inspection test; requires BetaNXT Standard - Cloud Security.pdf"]
    fn prints_betanxt_pdf_assembly_as_pretty_json() {
        let Some(path) = manual_pdf_path() else {
            eprintln!(
                "SKIPPED: {MANUAL_PDF_NAME} was not found. Set {MANUAL_PDF_ENV} to its full path \
                 and rerun:\n  {MANUAL_PDF_ENV}=\"/path/to/{MANUAL_PDF_NAME}\" cargo test \
                 services::document_service::tests::prints_betanxt_pdf_assembly_as_pretty_json \
                 -- --ignored --nocapture"
            );
            return;
        };

        assert!(
            path.is_file(),
            "{MANUAL_PDF_ENV} does not point to an accessible file: {}",
            path.display()
        );

        let assembly = assemble_pdf_document(&path, "manual-inspection-user")
            .unwrap_or_else(|err| panic!("failed to assemble {}: {err}", path.display()));
        let json = serde_json::to_string_pretty(&assembly)
            .expect("PDF document assembly should serialize as JSON");

        println!("{json}");
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
}
