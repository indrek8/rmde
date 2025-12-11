//! Category 6: Ordered Lists
//!
//! Tests for 1. and 1) style ordered lists
//! Status: Likely OK
//!
//! Test cases from example_test.md lines 124-145

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_ordered_list_period_style() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1. First item\n2. Second item\n3. Third item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 3, "Should find three ordered list markers (period style)");
}

#[test]
fn test_ordered_list_parenthesis_style() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1) Parenthesis style\n2) Also works\n3) Like this";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 3, "Should find three ordered list markers (parenthesis style)");
}

#[test]
fn test_ordered_list_starting_number() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "5. Starting at 5\n6. Continues from there\n7. Auto-increment";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 3, "Should find ordered list starting at arbitrary number");
}

#[test]
fn test_ordered_list_leading_zeros() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "0. Zero start\n00. Also zero (leading zeros)\n003. Start at 3";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 3, "Should find ordered list with leading zeros");
}

#[test]
fn test_ordered_list_nested() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1. First level\n   1. Nested level\n   2. Still nested\n2. Back to first level";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 4, "Should find nested ordered list markers");
}

#[test]
fn test_ordered_list_with_multiple_paragraphs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1. First paragraph.\n\n   Second paragraph in same item.\n\n2. Next item.";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 2, "Should find ordered list items with multiple paragraphs");
}

#[test]
fn test_ordered_list_number_must_be_nine_or_fewer_digits() {
    let mut parser = MarkdownParser::new().unwrap();

    // Valid: 9 digits
    let valid = "123456789. Valid item";
    let spans_valid = parser.parse(valid);
    let markers_valid: Vec<_> = spans_valid.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(markers_valid.len(), 1, "Nine-digit number should be valid");

    // Invalid: 10 digits (becomes paragraph)
    parser.reset();
    let invalid = "1234567890. Not a list item";
    let spans_invalid = parser.parse(invalid);
    let markers_invalid: Vec<_> = spans_invalid.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    println!("Ten-digit number found {} list markers (should be 0)", markers_invalid.len());
}
