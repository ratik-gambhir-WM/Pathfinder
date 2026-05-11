#![allow(dead_code)]


use docx_rust::{DocxError, DocxFile};
use std::path::Path;
use docx_rust::document::BodyContent;
use tokio::io::AsyncWriteExt;

pub fn parse_docx_file(path: &Path) -> Result<(), DocxError> {
    println!("parse_docx_file");

    let file = DocxFile::from_file(path).map_err(|err| err);
    match file {
        Ok(file) =>  {
            println!("parse_docx_file - ok");

            chunk_docx_file( & file)
        },
        Err(err) => {
            println!("parse_docx_file - error {}", err.to_string());

            Err(err)
        },
    }

}



fn chunk_docx_file(file: &DocxFile) -> Result<(), DocxError> {
    println!("chunk_docx_file");

    let docx = file.parse()?;
    let full_text = &docx.document.body.text();
    let headers = &docx.headers.len();
    let tables = &docx.document.body.content.iter().for_each(|content| {
        match content {
            BodyContent::Paragraph(paragraph) => {
                let text = paragraph.text();
                let runs = &paragraph.rsid_r;

                println!("PARAGRAPH: {}", text);
            }
            BodyContent::Table(table) => {
                println!("TABLE: {}", table.rows.len().to_string());
            }
            BodyContent::Sdt(sdr) => {}
            BodyContent::SectionProperty(_) => {}
            BodyContent::TableCell(_) => {}
            BodyContent::Run(_) => {}
        }
    });

    println!("chunk_docx_file - headers {}", headers.to_string());
    // println!("full_text: {}", full_text);

    Ok(())
}
