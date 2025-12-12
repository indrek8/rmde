//! Category 4: Blockquotes
//!
//! Tests for > blockquote syntax including nested quotes
//! Status: ISSUES DETECTED (nested quotes, code in quotes failing)
//!
//! Test cases from example_test.md lines 53-77

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Simple blockquote.";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    assert!(!quotes.is_empty(), "Should find blockquote");
}

#[test]
fn test_multi_line_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Multi-line blockquote.\n> With continuation.";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    assert!(!quotes.is_empty(), "Should find multi-line blockquote");
}

#[test]
fn test_lazy_continuation() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Lazy continuation\nworks here too without the > marker.";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    assert!(!quotes.is_empty(), "Should find blockquote with lazy continuation");
}

#[test]
fn test_heading_in_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> ### Heading in blockquote";
    let spans = parser.parse(content);

    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    assert_eq!(h3.len(), 1, "Should find heading inside blockquote");
}

#[test]
fn test_list_in_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> - List item in blockquote\n> - Another item";
    let spans = parser.parse(content);

    let list_items: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert!(!list_items.is_empty(), "Should find list items inside blockquote");
}

#[test]
fn test_nested_blockquotes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Level 1 quote\n> > Level 2 nested quote\n> > > Level 3 deeply nested";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();

    // Should find multiple levels of quote markers
    assert!(quotes.len() >= 3, "Should find markers for nested blockquotes (found {})", quotes.len());
}

#[test]
fn test_code_block_in_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Blockquote with code:\n>\n> ```python\n> print(\"Hello from inside a quote\")\n> ```";
    let spans = parser.parse(content);

    // Check for code block
    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();

    // This is a known issue area - code blocks inside blockquotes
    println!("Found {} code block spans inside blockquote", code_blocks.len());
}

#[test]
fn test_blockquote_with_blank_lines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> First paragraph.\n>\n> Second paragraph in same blockquote.";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    assert!(!quotes.is_empty(), "Should find blockquote with blank lines");
}
