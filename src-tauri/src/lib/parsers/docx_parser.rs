#![allow(dead_code)]


use docx_rust::document::BodyContent;
use docx_rust::{DocxError, DocxFile};
use std::path::Path;
use crate::parsers::DocxChunk;

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

fn gen_docx_chunk(content: &str, chunk_index: i32) -> DocxChunk {
    DocxChunk {
        chunk_id: "".to_string(),
        text: content.to_string(),
        text_hash: "".to_string(),
        chunk_index: chunk_index as i64,
        token_count: 0,
        page_start: 0,
        page_end: 0,
        embedded_at: None,
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

fn chunk_docx_text(file: &DocxFile) -> Result<(), DocxError> {
    println!("chunk_docx_file");

    let docx = file.parse()?;

    let full_text = &docx.document.body.text();


    Ok(())
}
