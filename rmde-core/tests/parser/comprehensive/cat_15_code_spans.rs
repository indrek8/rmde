//! Category 15: Code Spans
//!
//! Tests for `code`, ``code with `backtick` ``, etc.
//! Status: Testing required
//!
//! Test cases from example_test.md lines 348-361

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_code_span() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`Simple code span`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find inline code span");
}

#[test]
fn test_code_span_with_backtick_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "``Code span with `backtick` inside``";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find code span with backtick inside");
}

#[test]
fn test_triple_backticks_with_double_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "``` ``Double backticks`` inside triple ```";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find code span with double backticks inside");
}

#[test]
fn test_inline_code_in_sentence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is `inline code` in a sentence.";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find inline code in sentence");
}

#[test]
fn test_code_with_variable() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`var x = 10;`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find code span with variable");
}

#[test]
fn test_code_with_entity() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`&copy;`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(code.len() >= 1, "Should find code span with entity (entity should be literal)");
}

#[test]
fn test_code_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`code`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();

    assert!(code.len() >= 1, "Should find code span");
    // Markers are optional depending on implementation
    if markers.len() > 0 {
        assert_eq!(markers.len(), 2, "Should find opening and closing markers if markers are tracked");
    }
}

#[test]
fn test_empty_code_span() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "``";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    // Empty code spans are typically not rendered
    // This documents the expected behavior
    assert_eq!(code.len(), 0, "Empty code span should not be rendered");
}
