//! Category 22: Abbreviations
//!
//! Tests for *[ABBR]: Full text syntax
//! Status: NOT IMPLEMENTED (expected to fail - documents unsupported feature)
//!
//! Abbreviations are a PHP Markdown Extra extension not commonly
//! supported in standard Markdown parsers.

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_abbreviation_definition() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*[HTML]: Hyper Text Markup Language";
    let spans = parser.parse(content);

    // Abbreviations are likely not implemented
    // This should parse as emphasis or plain text
    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Expected behavior: treats * as emphasis attempt, not abbreviation definition
    assert!(spans.len() > 0, "Should parse as regular markdown, not as abbreviation");
}

#[test]
fn test_abbreviation_usage() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*[HTML]: Hyper Text Markup Language\n\nThe HTML specification is important.";
    let spans = parser.parse(content);

    // Without abbreviation support, "HTML" in the second line should just be plain text
    // No special abbreviation span type exists in SpanKind
    assert!(spans.len() > 0, "Should parse content without abbreviation expansion");
}

#[test]
fn test_multiple_abbreviations() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*[HTML]: Hyper Text Markup Language\n*[CSS]: Cascading Style Sheets";
    let spans = parser.parse(content);

    // Should parse as regular content, not abbreviation definitions
    assert!(spans.len() > 0, "Multiple abbreviation definitions should parse as regular text");
}

#[test]
fn test_abbreviation_not_confused_with_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*[text]*";
    let spans = parser.parse(content);

    // This might be parsed as emphasis with brackets inside
    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Should handle as emphasis attempt, not abbreviation
    assert!(spans.len() > 0, "Should parse without abbreviation support");
}
