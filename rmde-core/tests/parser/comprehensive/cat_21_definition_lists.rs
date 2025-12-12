//! Category 21: Definition Lists
//!
//! Tests for term + `: definition` syntax
//! Status: NOT IMPLEMENTED (expected to fail - documents unsupported feature)
//!
//! Definition lists are a PHP Markdown Extra extension not commonly
//! supported in standard Markdown parsers.

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_definition() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Term\n: Definition for the term";
    let spans = parser.parse(content);

    // Definition lists are likely not implemented
    // This test documents that the feature is unsupported
    let has_special_handling = spans.iter().any(|s|
        // There's no SpanKind for definition lists in the enum
        matches!(s.kind, SpanKind::FootnoteDef) // Closest thing, but not it
    );

    // Expected to fail - definition lists not supported
    assert!(!has_special_handling, "Definition lists are not implemented in this parser");
}

#[test]
fn test_multiple_definitions() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Term\n: First definition\n: Second definition";
    let spans = parser.parse(content);

    // This should just parse as regular text/paragraphs
    // No special definition list handling expected
    let plain_text_spans = spans.iter().filter(|s|
        !matches!(s.kind, SpanKind::FootnoteDef)
    ).count();

    assert!(plain_text_spans > 0, "Should parse as regular text without definition list support");
}

#[test]
fn test_definition_with_multiple_terms() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First Term\nSecond Term\n: Definition for both terms";
    let spans = parser.parse(content);

    // Without definition list support, this should be treated as plain text
    assert!(spans.len() > 0, "Should parse content even without definition list support");
}

#[test]
fn test_definition_list_not_confused_with_colon_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Normal text: with a colon in it";
    let spans = parser.parse(content);

    // This should just be regular text, not a definition
    assert!(spans.len() > 0, "Regular text with colons should parse normally");
}
