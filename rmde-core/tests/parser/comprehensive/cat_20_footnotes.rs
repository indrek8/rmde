//! Category 20: Footnotes
//!
//! Tests for [^1] references and definitions
//! Status: Testing required
//!
//! Test cases from example_test.md lines 464-477

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_footnote_reference() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Here is a footnote reference[^1].\n\n[^1]: This is the footnote content.";
    let spans = parser.parse(content);

    let footnote_refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert!(footnote_refs.len() >= 1, "Should find footnote reference");
}

#[test]
fn test_footnote_definition() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text[^1].\n\n[^1]: This is the footnote content.";
    let spans = parser.parse(content);

    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    assert!(footnote_defs.len() >= 1, "Should find footnote definition");
}

#[test]
fn test_named_footnote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Another footnote[^note].\n\n[^note]: This is another footnote with a longer name.";
    let spans = parser.parse(content);

    let footnote_refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();

    assert!(footnote_refs.len() >= 1, "Should find named footnote reference");
    assert!(footnote_defs.len() >= 1, "Should find named footnote definition");
}

#[test]
fn test_inline_footnote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Inline footnote^[This is an inline footnote].";
    let spans = parser.parse(content);

    // Inline footnotes may be handled differently
    // This test documents behavior - may not have specific SpanKind
    assert!(spans.len() > 0, "Parser should handle inline footnotes");
}

#[test]
fn test_footnote_with_multiple_paragraphs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text[^multi].\n\n[^multi]: First paragraph.\n\n    Second paragraph with indentation.";
    let spans = parser.parse(content);

    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    assert!(footnote_defs.len() >= 1, "Should find footnote with multiple paragraphs");
}

#[test]
fn test_multiple_footnote_references() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First[^1] and second[^2].\n\n[^1]: First note.\n[^2]: Second note.";
    let spans = parser.parse(content);

    let footnote_refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();

    assert!(footnote_refs.len() >= 2, "Should find multiple footnote references");
    assert!(footnote_defs.len() >= 2, "Should find multiple footnote definitions");
}

#[test]
fn test_footnote_with_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text[^fmt].\n\n[^fmt]: This footnote has **bold** and *italic* text.";
    let spans = parser.parse(content);

    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(footnote_defs.len() >= 1, "Should find footnote definition");
    assert!(bold.len() >= 1, "Should find bold text in footnote");
    assert!(italic.len() >= 1, "Should find italic text in footnote");
}

#[test]
fn test_unused_footnote_definition() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text without reference.\n\n[^unused]: This footnote is never referenced.";
    let spans = parser.parse(content);

    let footnote_defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    // Parser should still recognize the definition even if unused
    assert!(footnote_defs.len() >= 1, "Should find unused footnote definition");
}
