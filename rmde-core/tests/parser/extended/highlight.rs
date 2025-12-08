//! Highlighting tests
//!
//! Tests for highlighting syntax (==text==) per MARKDOWN-SYNTAX.md § Highlighting

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_highlight_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ==highlighted== text.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 1, "Should find one highlight span");
    assert_eq!(hl[0].start, 8);
    assert_eq!(hl[0].end, 23);
    assert_eq!(&content[hl[0].start..hl[0].end], "==highlighted==");
}

#[test]
fn test_highlight_multiple() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==First== and ==second== highlights.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 2, "Should find two highlight spans");
    assert_eq!(&content[hl[0].start..hl[0].end], "==First==");
    assert_eq!(&content[hl[1].start..hl[1].end], "==second==");
}

#[test]
fn test_highlight_empty() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ==== more.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 0, "Empty highlight ==== should not match");
}

#[test]
fn test_highlight_unclosed() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ==unclosed";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 0, "Unclosed highlight should not match");
}

#[test]
fn test_highlight_with_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ==very important text== here.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 1, "Highlight can contain spaces");
    assert_eq!(&content[hl[0].start..hl[0].end], "==very important text==");
}

#[test]
fn test_highlight_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Code `==text==` should not be highlighted.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 0, "Highlight inside code span should not match");
}

#[test]
fn test_highlight_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==This is\na multiline\nhighlight==";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 1, "Highlight can span multiple lines");
    assert_eq!(&content[hl[0].start..hl[0].end], "==This is\na multiline\nhighlight==");
}

#[test]
fn test_highlight_with_punctuation() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==Important: read this!== carefully.";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    assert_eq!(hl.len(), 1, "Highlight can contain punctuation");
}

#[test]
fn test_highlight_nested_not_supported() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==outer ==inner== outer==";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    // This should find the first matching pair: "==outer ==inner=="
    // Nesting is not supported, so we match greedily
    assert!(hl.len() >= 1, "Should find at least one highlight");
}

#[test]
fn test_highlight_adjacent() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "==one====two==";
    let spans = parser.parse(content);

    let hl: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Highlight).collect();
    // Should match "==one==" first, then "==two=="
    assert!(hl.len() >= 1, "Should handle adjacent highlights");
}
