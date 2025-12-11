//! Category 13: Combined Emphasis
//!
//! Tests for ***text***, **_nested_**, etc.
//! Status: Testing required
//!
//! Test cases from example_test.md lines 316-331

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_triple_asterisks_bold_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "***Bold and italic with asterisks***";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold_italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::BoldItalic).collect();

    // Could be BoldItalic, or separate Bold + Italic, or nested combinations
    assert!(
        bold_italic.len() >= 1 || (bold.len() >= 1 && italic.len() >= 1),
        "Should find bold+italic with triple asterisks"
    );
}

#[test]
fn test_triple_underscores_bold_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "___Bold and italic with underscores___";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold_italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::BoldItalic).collect();

    // Could be BoldItalic, or separate Bold + Italic, or nested combinations
    assert!(
        bold_italic.len() >= 1 || (bold.len() >= 1 && italic.len() >= 1),
        "Should find bold+italic with triple underscores"
    );
}

#[test]
fn test_bold_with_nested_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**_Bold with nested italic_**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(bold.len() >= 1, "Should find bold span");
    assert!(italic.len() >= 1, "Should find italic span nested in bold");
}

#[test]
fn test_italic_with_nested_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*__Italic with nested bold__*";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(italic.len() >= 1, "Should find italic span");
    assert!(bold.len() >= 1, "Should find bold span nested in italic");
}

#[test]
fn test_inline_triple_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ***all bold italic*** text.";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold_italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::BoldItalic).collect();

    assert!(
        bold_italic.len() >= 1 || (bold.len() >= 1 && italic.len() >= 1),
        "Should find bold+italic inline"
    );
}

#[test]
fn test_bold_with_italic_word_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**Bold with *italic* word inside**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(bold.len() >= 1, "Should find bold span");
    assert!(italic.len() >= 1, "Should find italic word inside bold");
}

#[test]
fn test_italic_with_bold_word_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Italic with **bold** word inside*";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(italic.len() >= 1, "Should find italic span");
    assert!(bold.len() >= 1, "Should find bold word inside italic");
}

#[test]
fn test_adjacent_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**foo** **bar**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(bold.len() >= 2, "Should find two separate bold spans");
}
