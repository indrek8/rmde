//! Category 14: Strikethrough (GFM)
//!
//! Tests for ~~text~~ strikethrough syntax
//! Status: Testing required
//!
//! Test cases from example_test.md lines 334-345

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~Strikethrough text~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert!(strike.len() >= 1, "Should find strikethrough span");
}

#[test]
fn test_inline_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This has ~~deleted~~ text inline.";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert!(strike.len() >= 1, "Should find inline strikethrough");
}

#[test]
fn test_bold_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**~~Bold strikethrough~~**";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert!(strike.len() >= 1, "Should find strikethrough span");
    assert!(bold.len() >= 1, "Should find bold span");
}

#[test]
fn test_italic_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*~~Italic strikethrough~~*";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(strike.len() >= 1, "Should find strikethrough span");
    assert!(italic.len() >= 1, "Should find italic span");
}

#[test]
fn test_strikethrough_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~**Strikethrough bold**~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert!(strike.len() >= 1, "Should find strikethrough span");
    assert!(bold.len() >= 1, "Should find bold span inside strikethrough");
}
