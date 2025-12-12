//! Category 11: Emphasis (Italic)
//!
//! Tests for *text* and _text_ italic syntax
//! Status: Testing required
//!
//! Test cases from example_test.md lines 293-303

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_single_asterisks_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Single asterisks for emphasis*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.len() >= 1, "Should find italic span with single asterisks");
}

#[test]
fn test_single_underscores_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "_Single underscores for emphasis_";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.len() >= 1, "Should find italic span with single underscores");
}

#[test]
fn test_inline_emphasis_in_sentence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is *inline* emphasis in a sentence.";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.len() >= 1, "Should find inline italic emphasis in sentence");
}

#[test]
fn test_emphasis_spanning_multiple_lines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Emphasis can span *multiple\nlines* in a paragraph.";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.len() >= 1, "Should find italic emphasis spanning multiple lines");
}

#[test]
fn test_intraword_emphasis_with_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo*bar*baz";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.len() >= 1, "Asterisks should work mid-word for intraword emphasis");
}

#[test]
fn test_intraword_emphasis_with_underscores_should_not_work() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo_bar_baz";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    // According to CommonMark, underscores don't work mid-word
    // This test documents current behavior - may need adjustment based on parser implementation
    assert_eq!(italic.len(), 0, "Underscores should not work mid-word");
}

#[test]
fn test_empty_emphasis_not_rendered() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    // Empty emphasis should not be rendered as italic
    assert_eq!(italic.len(), 0, "Empty emphasis should not create italic span");
}
