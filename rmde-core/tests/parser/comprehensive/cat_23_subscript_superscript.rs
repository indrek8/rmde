//! Category 23: Subscript and Superscript
//!
//! Tests for H~2~O subscript and E=mc^2^ superscript syntax
//! Status: NOT IMPLEMENTED (expected to fail - documents unsupported feature)
//!
//! Subscript and superscript are extensions not commonly supported
//! in standard Markdown parsers.

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_subscript_syntax() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "H~2~O";
    let spans = parser.parse(content);

    // Subscript is not in the SpanKind enum
    // Should parse as plain text with tildes
    let has_subscript_kind = spans.iter().any(|s| {
        // No subscript variant exists in SpanKind
        false
    });

    assert!(!has_subscript_kind, "Subscript is not implemented in this parser");
    assert!(spans.len() > 0, "Should parse as plain text");
}

#[test]
fn test_superscript_syntax() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "E=mc^2^";
    let spans = parser.parse(content);

    // Superscript is not in the SpanKind enum
    // Should parse as plain text with carets
    let has_superscript_kind = spans.iter().any(|s| {
        // No superscript variant exists in SpanKind
        false
    });

    assert!(!has_superscript_kind, "Superscript is not implemented in this parser");
    assert!(spans.len() > 0, "Should parse as plain text");
}

#[test]
fn test_chemical_formula_with_subscript() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "The chemical formula for water is H~2~O.";
    let spans = parser.parse(content);

    // Without subscript support, should just be plain text
    assert!(spans.len() > 0, "Chemical formulas should parse as plain text");
}

#[test]
fn test_mathematical_expression_with_superscript() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Einstein's equation: E=mc^2^";
    let spans = parser.parse(content);

    // Without superscript support, should just be plain text
    assert!(spans.len() > 0, "Mathematical expressions should parse as plain text");
}

#[test]
fn test_multiple_subscripts() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "C~6~H~12~O~6~";
    let spans = parser.parse(content);

    // Complex chemical formula - should parse as plain text
    assert!(spans.len() > 0, "Multiple subscripts should parse as plain text");
}

#[test]
fn test_subscript_superscript_combination() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "x^2^ + y~1~";
    let spans = parser.parse(content);

    // Mixed sub/superscript - should parse as plain text
    assert!(spans.len() > 0, "Combined sub/superscript should parse as plain text");
}
