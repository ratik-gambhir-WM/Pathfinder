use super::*;

#[test]
fn empty_text_has_no_ranges() {
    assert!(token_bounded_ranges("").is_empty());
}

#[test]
fn ranges_are_token_bounded_contiguous_and_unicode_safe() {
    let text = format!(
        "{}🙂\n{}",
        "first section ".repeat(MAX_TOKEN_CHUNK),
        "second section ".repeat(MAX_TOKEN_CHUNK)
    );

    let ranges = token_bounded_ranges(&text);

    assert!(ranges.len() > 1);
    assert_eq!(
        ranges
            .iter()
            .map(|range| &text[range.start_offset..range.end_offset])
            .collect::<String>(),
        text
    );
    for (index, range) in ranges.iter().enumerate() {
        assert!(range.token_count <= MAX_TOKEN_CHUNK);
        assert_eq!(
            range.start_offset,
            ranges
                .get(index.wrapping_sub(1))
                .map(|previous| previous.end_offset)
                .unwrap_or(0)
        );
    }
    assert_eq!(ranges.last().unwrap().end_offset, text.len());
}
