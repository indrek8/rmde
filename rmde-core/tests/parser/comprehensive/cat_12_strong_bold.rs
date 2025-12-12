//! Category 12: Strong (Bold)
//!
//! Tests for **text** and __text__ bold syntax
//! Status: Testing required
//!
//! Test cases from example_test.md lines 306-313

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_double_asterisks_strong() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**Double asterisks for strong**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(bold.len() >= 1, "Should find bold span with double asterisks");
}

#[test]
fn test_double_underscores_strong() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "__Double underscores for strong__";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(bold.len() >= 1, "Should find bold span with double underscores");
}

#[test]
fn test_inline_strong_in_sentence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is **inline** strong in a sentence.";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(bold.len() >= 1, "Should find inline bold strong in sentence");
}

#[test]
fn test_intraword_bold_with_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**foo**bar";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(bold.len() >= 1, "Double asterisks should work adjacent to text");
}

#[test]
fn test_intraword_bold_with_underscores_should_not_work() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "__foo__bar";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    // According to CommonMark, double underscores don't work mid-word
    // This test documents expected behavior
    assert_eq!(bold.len(), 0, "Double underscores should not work mid-word");
}

#[test]
fn test_empty_bold_not_rendered() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "****";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    // Empty bold should not be rendered
    assert_eq!(bold.len(), 0, "Empty bold should not create bold span");
}
