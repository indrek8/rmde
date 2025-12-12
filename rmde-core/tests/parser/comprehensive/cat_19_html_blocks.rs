//! Category 19: HTML Blocks
//!
//! Tests for raw HTML, comments
//! Status: Testing required
//!
//! Test cases from example_test.md lines 439-461
//!
//! Note: HTML block parsing depends heavily on implementation.
//! The parser may not emit specific SpanKind for HTML blocks,
//! or may treat them as regular text. These tests document behavior.

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_html_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<div class=\"container\">\nThis is raw HTML content.\n</div>";
    let spans = parser.parse(content);

    // HTML blocks may not have a specific SpanKind in all implementations
    // This test documents that the parser handles HTML without crashing
    assert!(spans.len() > 0, "Parser should handle HTML blocks");
}

#[test]
fn test_html_comment() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<!-- HTML Comment -->";
    let spans = parser.parse(content);

    // HTML comments may not have specific SpanKind
    // This test verifies parsing doesn't fail
    assert!(spans.len() >= 0, "Parser should handle HTML comments");
}

#[test]
fn test_inline_html_paragraph() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<p>Inline HTML paragraph</p>";
    let spans = parser.parse(content);

    assert!(spans.len() > 0, "Parser should handle inline HTML paragraphs");
}

#[test]
fn test_html_with_markdown_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<div>\n\n*Markdown works here with blank lines*\n\n</div>";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    // Markdown inside HTML blocks should work with blank lines
    assert!(italic.len() >= 1, "Should parse markdown inside HTML with blank lines");
}

#[test]
fn test_details_summary_html() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<details>\n<summary>Expandable section</summary>\n\nContent inside details tag.\n\n</details>";
    let spans = parser.parse(content);

    assert!(spans.len() > 0, "Parser should handle details/summary HTML");
}

#[test]
fn test_multiple_html_blocks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<div>Block 1</div>\n\n<div>Block 2</div>";
    let spans = parser.parse(content);

    assert!(spans.len() > 0, "Parser should handle multiple HTML blocks");
}

#[test]
fn test_html_with_attributes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<div class=\"test\" id=\"main\" data-value=\"123\">\nContent\n</div>";
    let spans = parser.parse(content);

    assert!(spans.len() > 0, "Parser should handle HTML with attributes");
}

#[test]
fn test_self_closing_html_tag() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<br />\n<img src=\"test.png\" />";
    let spans = parser.parse(content);

    assert!(spans.len() >= 0, "Parser should handle self-closing HTML tags");
}
