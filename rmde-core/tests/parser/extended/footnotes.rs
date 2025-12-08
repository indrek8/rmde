//! Footnote tests
//!
//! Tests for footnote syntax per MARKDOWN-SYNTAX.md § Footnotes

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_footnote_reference() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "See this claim[^1] for proof.";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert_eq!(refs.len(), 1, "Should find one footnote reference");
    assert_eq!(refs[0].start, 14);
    assert_eq!(refs[0].end, 18);
    assert_eq!(&content[refs[0].start..refs[0].end], "[^1]");
}

#[test]
fn test_footnote_definition() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[^1]: This is the footnote text.";
    let spans = parser.parse(content);

    let defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    assert_eq!(defs.len(), 1, "Should find one footnote definition");
    assert_eq!(defs[0].start, 0);
    assert_eq!(defs[0].end, 5);
    assert_eq!(&content[defs[0].start..defs[0].end], "[^1]:");
}

#[test]
fn test_footnote_multiple_references() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First reference[^1] and second[^2] reference.";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert_eq!(refs.len(), 2, "Should find two footnote references");
    assert_eq!(&content[refs[0].start..refs[0].end], "[^1]");
    assert_eq!(&content[refs[1].start..refs[1].end], "[^2]");
}

#[test]
fn test_footnote_alphanumeric_id() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Reference[^note-1] and [^another_note].";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert_eq!(refs.len(), 2, "Should support alphanumeric IDs with dashes/underscores");
    assert_eq!(&content[refs[0].start..refs[0].end], "[^note-1]");
    assert_eq!(&content[refs[1].start..refs[1].end], "[^another_note]");
}

#[test]
fn test_footnote_unclosed() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text [^unclosed without bracket.";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert_eq!(refs.len(), 0, "Unclosed footnote should not match");
}

#[test]
fn test_footnote_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Code `[^1]` should not be footnote.";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    assert_eq!(refs.len(), 0, "Footnote inside code span should not match");
}

#[test]
fn test_footnote_definition_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[^1]: This is a footnote\nwith multiple lines.";
    let spans = parser.parse(content);

    let defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();
    assert_eq!(defs.len(), 1, "Should find footnote definition");
    assert_eq!(&content[defs[0].start..defs[0].end], "[^1]:");
}

#[test]
fn test_footnote_mixed() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text[^1] and more[^2].\n\n[^1]: First note.\n[^2]: Second note.";
    let spans = parser.parse(content);

    let refs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteRef).collect();
    let defs: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::FootnoteDef).collect();

    assert_eq!(refs.len(), 2, "Should find two references");
    assert_eq!(defs.len(), 2, "Should find two definitions");
}
