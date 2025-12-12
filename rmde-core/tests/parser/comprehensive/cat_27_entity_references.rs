//! Category 27: Entity References
//!
//! Tests for &copy;, &#169;, and &#xA9; HTML entity syntax
//! Status: Testing required
//!
//! HTML entities should be recognized by the parser but rendering
//! is the responsibility of the renderer, not the parser.

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_named_entity_copyright() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Copyright &copy; 2024";
    let spans = parser.parse(content);

    // Parser should handle the content - entities are typically
    // left for the renderer to handle
    assert!(spans.len() > 0, "Should parse content with named entities");
}

#[test]
fn test_decimal_entity() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Copyright &#169; 2024";
    let spans = parser.parse(content);

    // Decimal numeric entities
    assert!(spans.len() > 0, "Should parse content with decimal entities");
}

#[test]
fn test_hexadecimal_entity() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Copyright &#xA9; 2024";
    let spans = parser.parse(content);

    // Hexadecimal numeric entities
    assert!(spans.len() > 0, "Should parse content with hex entities");
}

#[test]
fn test_common_named_entities() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "&lt; &gt; &amp; &quot; &apos;";
    let spans = parser.parse(content);

    // Common HTML entities
    assert!(spans.len() > 0, "Should parse common HTML entities");
}

#[test]
fn test_entity_in_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Copyright &copy; 2024*";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(emphasis.len() >= 1, "Should handle entities inside emphasis");
}

#[test]
fn test_entity_in_link_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Copyright &copy;](url)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    assert!(links.len() >= 1, "Should handle entities in link text");
}

#[test]
fn test_entity_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`&copy;`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeInline
    ).collect();

    // Entities in code should be literal, not processed
    assert!(code.len() >= 1, "Should find code span with literal entity");
}

#[test]
fn test_invalid_entity() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "&invalid;";
    let spans = parser.parse(content);

    // Invalid entities should be left as-is or handled gracefully
    assert!(spans.len() > 0, "Should handle invalid entities gracefully");
}

#[test]
fn test_incomplete_entity() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "&copy text";
    let spans = parser.parse(content);

    // Entity without semicolon should not be processed
    assert!(spans.len() > 0, "Should handle incomplete entities");
}

#[test]
fn test_numeric_entity_zero() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "&#0;";
    let spans = parser.parse(content);

    // Edge case: zero entity
    assert!(spans.len() > 0, "Should handle zero numeric entity");
}

#[test]
fn test_multiple_entities() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "&copy; &reg; &trade;";
    let spans = parser.parse(content);

    // Multiple named entities in sequence
    assert!(spans.len() > 0, "Should handle multiple entities");
}

#[test]
fn test_entity_in_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Copyright &copy; Notice";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Heading1)
    ).collect();

    assert!(headings.len() >= 1, "Should handle entities in headings");
}
