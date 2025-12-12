//! Category 5: Unordered Lists
//!
//! Tests for -, *, + list markers
//! Status: PARTIAL ISSUES
//!
//! Test cases from example_test.md lines 80-122

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_dash_list_marker() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item with dash\n- Another item\n- Third item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 3, "Should find three dash list markers");
}

#[test]
fn test_asterisk_list_marker() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "* Item with asterisk\n* Another item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 2, "Should find two asterisk list markers");
}

#[test]
fn test_plus_list_marker() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "+ Item with plus\n+ Another item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 2, "Should find two plus list markers");
}

#[test]
fn test_nested_lists() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Parent item\n  - Nested item (2 spaces)\n    - Deeply nested (4 spaces)\n  - Back to level 2\n- Back to level 1";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 5, "Should find five list markers in nested list");
}

#[test]
fn test_list_with_multiple_paragraphs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- First paragraph of item.\n\n  Second paragraph of same item (indented).\n\n- Another item.";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 2, "Should find two list items with multi-paragraph content");
}

#[test]
fn test_list_with_code_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item with code:\n\n  ```javascript\n  function hello() {\n    console.log(\"Hello\");\n  }\n  ```";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 1, "Should find list item");

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    println!("Found {} code blocks inside list item", code_blocks.len());
}

#[test]
fn test_list_with_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item with blockquote:\n\n  > Quote inside list item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 1, "Should find list item");

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    assert!(!quotes.is_empty(), "Should find blockquote inside list item");
}

#[test]
fn test_tight_vs_loose_lists() {
    let mut parser = MarkdownParser::new().unwrap();

    // Tight list (no blank lines)
    let tight = "- Item 1\n- Item 2\n- Item 3";
    let spans_tight = parser.parse(tight);
    let markers_tight: Vec<_> = spans_tight.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(markers_tight.len(), 3, "Tight list should have 3 items");

    // Loose list (with blank lines)
    parser.reset();
    let loose = "- Item 1\n\n- Item 2\n\n- Item 3";
    let spans_loose = parser.parse(loose);
    let markers_loose: Vec<_> = spans_loose.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(markers_loose.len(), 3, "Loose list should have 3 items");
}
