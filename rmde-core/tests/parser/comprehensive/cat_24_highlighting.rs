//! Category 24: Highlighting
//!
//! Tests for ==text== highlighting syntax
//! Status: Should be OK (Highlight = 84 in SpanKind)
//!
//! Test cases for the highlighting feature

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_basic_highlighting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ==highlighted text==.";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    assert!(highlights.len() >= 1, "Should find highlighted text with == syntax");
}

#[test]
fn test_highlight_full_sentence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==This entire sentence is highlighted.==";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    assert!(highlights.len() >= 1, "Should highlight entire sentence");
}

#[test]
fn test_highlight_with_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==**Bold and highlighted**==";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();
    let bold: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic)
    ).collect();

    assert!(highlights.len() >= 1, "Should find highlight span");
    assert!(bold.len() >= 1, "Should find bold span inside highlight");
}

#[test]
fn test_highlight_with_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==*Italic and highlighted*==";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();
    let italic: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::BoldItalic)
    ).collect();

    assert!(highlights.len() >= 1, "Should find highlight span");
    assert!(italic.len() >= 1, "Should find italic span inside highlight");
}

#[test]
fn test_bold_containing_highlight() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**Bold with ==highlight== inside**";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();
    let bold: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic)
    ).collect();

    assert!(highlights.len() >= 1, "Should find highlight inside bold");
    assert!(bold.len() >= 1, "Should find bold span");
}

#[test]
fn test_multiple_highlights() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First ==highlight== and second ==highlight==.";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    assert!(highlights.len() >= 2, "Should find two separate highlight spans");
}

#[test]
fn test_highlight_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==This highlight\nspans multiple lines==";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    assert!(highlights.len() >= 1, "Should support multiline highlighting");
}

#[test]
fn test_empty_highlight() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "====";
    let spans = parser.parse(content);

    let highlights: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    // Empty highlight may or may not be rendered - document behavior
    // Most parsers don't render empty formatting
    assert_eq!(highlights.len(), 0, "Empty highlight should not be rendered");
}
