use std::path::Path;

use anyhow::Result;
use docx_rust::DocxError;

use crate::models::document::SpreadsheetTextChunk;

pub fn parse_docx_file(path: &Path) -> Result<(), DocxError> {
    crate::parsers::docx::parse_docx_file(path)
}

pub fn parse_spreadsheet(path: impl AsRef<Path>) -> Result<Vec<SpreadsheetTextChunk>> {
    crate::parsers::spreadsheet::parse_spreadsheet(path)
}
