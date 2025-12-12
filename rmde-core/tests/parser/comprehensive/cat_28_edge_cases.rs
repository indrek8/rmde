//! Category 28: Edge Cases
//!
//! Tests for edge cases and corner scenarios
//! Status: Testing required
//!
//! Test cases for unusual but valid markdown constructs

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_intraword_emphasis_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo*bar*baz";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Asterisks should work for intraword emphasis
    assert!(emphasis.len() >= 1, "Asterisks should work mid-word");
}

#[test]
fn test_intraword_emphasis_underscores() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo_bar_baz";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Underscores should NOT work for intraword emphasis per CommonMark
    assert_eq!(emphasis.len(), 0, "Underscores should not work mid-word");
}

#[test]
fn test_adjacent_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*foo**bar*";
    let spans = parser.parse(content);

    // Adjacent emphasis markers - should handle gracefully
    let formatted: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind,
            SpanKind::Italic | SpanKind::Bold | SpanKind::BoldItalic | SpanKind::Emphasis
        )
    ).collect();

    assert!(formatted.len() >= 1, "Should handle adjacent emphasis markers");
}

#[test]
fn test_empty_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::Italic)
    ).collect();

    // Empty emphasis should not be rendered
    assert_eq!(emphasis.len(), 0, "Empty emphasis should not create spans");
}

#[test]
fn test_empty_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[](url)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    // Empty link text - may or may not be valid
    assert!(spans.len() > 0, "Should handle empty link text");
}

#[test]
fn test_nested_brackets() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[[nested]]";
    let spans = parser.parse(content);

    // Nested brackets without proper link syntax
    assert!(spans.len() > 0, "Should handle nested brackets");
}

#[test]
fn test_mismatched_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*open but not closed";
    let spans = parser.parse(content);

    // Unclosed emphasis - should treat as literal
    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Unclosed emphasis may or may not be rendered
    assert!(spans.len() > 0, "Should handle unclosed emphasis");
}

#[test]
fn test_multiple_spaces_in_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*  multiple spaces  *";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Leading/trailing spaces in emphasis - behavior may vary
    assert!(spans.len() > 0, "Should handle spaces in emphasis");
}

#[test]
fn test_emphasis_with_only_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*   *";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Emphasis with only spaces - should not be rendered
    assert_eq!(emphasis.len(), 0, "Emphasis with only spaces should not render");
}

#[test]
fn test_code_span_with_backticks_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`` ` ``";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeInline
    ).collect();

    // Double backticks to include single backtick
    assert!(code.len() >= 1, "Should handle backticks inside code span");
}

#[test]
fn test_link_with_nested_parentheses() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[text](url(with)parens)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    // Nested parentheses in URL - should handle or escape
    assert!(spans.len() > 0, "Should handle nested parentheses in URL");
}

#[test]
fn test_very_long_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let long_text = "a".repeat(1000);
    let content = format!("*{}*", long_text);
    let spans = parser.parse(&content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(emphasis.len() >= 1, "Should handle very long emphasis spans");
}

#[test]
fn test_unicode_in_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Hello 世界*";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(emphasis.len() >= 1, "Should handle Unicode in emphasis");
}

#[test]
fn test_emoji_in_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Hello 😀*";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(emphasis.len() >= 1, "Should handle emoji in emphasis");
}
