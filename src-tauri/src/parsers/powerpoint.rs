#![allow(dead_code)]

use std::path::Path;

use pptx_to_md::{ImageHandlingMode, ParserConfig, PptxContainer, SlideElement};

pub fn parse_powerpoint_file(path: &Path) -> Result<String, String> {
    ensure_supported_powerpoint_file(path)?;

    let config = ParserConfig::builder()
        .extract_images(false)
        .compress_images(false)
        .image_handling_mode(ImageHandlingMode::Manually)
        .include_slide_comment(false)
        .build();
    let mut container = PptxContainer::open(path, config)
        .map_err(|err| format!("failed to open PowerPoint file {}: {err}", path.display()))?;
    let mut slides = container
        .parse_all()
        .map_err(|err| format!("failed to parse PowerPoint file {}: {err}", path.display()))?;

    slides.sort_by_key(|slide| slide.slide_number);

    let full_text = slides
        .into_iter()
        .map(|slide| slide_text(&slide.elements))
        .filter(|text| !text.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");

    if full_text.trim().is_empty() {
        return Ok(String::new());
    }

    Ok(full_text)
}

fn ensure_supported_powerpoint_file(path: &Path) -> Result<(), String> {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("pptx") => Ok(()),
        Some("ppt") => Err("legacy .ppt files are not supported; expected .pptx".to_string()),
        Some(extension) => Err(format!(
            "unsupported PowerPoint extension .{extension}; expected pptx"
        )),
        None => Err(format!(
            "could not infer PowerPoint type for {}; expected .pptx",
            path.display()
        )),
    }
}

fn slide_text(elements: &[SlideElement]) -> String {
    let mut sorted_elements = elements.to_vec();
    sorted_elements.sort_by_key(|element| {
        let position = element.position();
        (position.y, position.x)
    });

    sorted_elements
        .iter()
        .filter_map(element_text)
        .filter(|text| !text.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn element_text(element: &SlideElement) -> Option<String> {
    match element {
        SlideElement::Text(text, _) => Some(
            text.runs
                .iter()
                .map(|run| run.extract())
                .collect::<String>(),
        ),
        SlideElement::List(list, _) => Some(
            list.items
                .iter()
                .map(|item| {
                    item.runs
                        .iter()
                        .map(|run| run.extract())
                        .collect::<String>()
                })
                .filter(|text| !text.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        SlideElement::Table(table, _) => Some(
            table
                .rows
                .iter()
                .map(|row| {
                    row.cells
                        .iter()
                        .map(|cell| {
                            cell.runs
                                .iter()
                                .map(|run| run.extract())
                                .collect::<String>()
                        })
                        .collect::<Vec<_>>()
                        .join("\t")
                })
                .filter(|text| !text.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        SlideElement::Image(_, _) | SlideElement::Unknown => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pptx_to_md::{
        ElementPosition, ListElement, ListItem, Run, TableCell, TableElement, TableRow, TextElement,
    };

    fn run(text: &str) -> Run {
        Run {
            text: text.to_string(),
            formatting: Default::default(),
        }
    }

    #[test]
    fn extracts_text_from_slide_elements_in_visual_order() {
        let elements = vec![
            SlideElement::Text(
                TextElement {
                    runs: vec![run("Subtitle")],
                },
                ElementPosition { x: 200, y: 200 },
            ),
            SlideElement::List(
                ListElement {
                    items: vec![
                        ListItem {
                            level: 0,
                            is_ordered: false,
                            runs: vec![run("First bullet")],
                        },
                        ListItem {
                            level: 0,
                            is_ordered: false,
                            runs: vec![run("Second bullet")],
                        },
                    ],
                },
                ElementPosition { x: 100, y: 300 },
            ),
            SlideElement::Table(
                TableElement {
                    rows: vec![TableRow {
                        cells: vec![
                            TableCell {
                                runs: vec![run("Name")],
                            },
                            TableCell {
                                runs: vec![run("Amount")],
                            },
                        ],
                    }],
                },
                ElementPosition { x: 100, y: 400 },
            ),
            SlideElement::Text(
                TextElement {
                    runs: vec![run("Title")],
                },
                ElementPosition { x: 100, y: 100 },
            ),
        ];

        assert_eq!(
            slide_text(&elements),
            "Title\nSubtitle\nFirst bullet\nSecond bullet\nName\tAmount"
        );
    }
}
