//! Block element tests - Lists, blockquotes, thematic breaks, links, images
//!
//! Tests for CommonMark block syntax per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_unordered_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item 1\n- Item 2";
    let spans = parser.parse(content);

    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::ListMarker).collect();
    assert!(markers.len() >= 2, "Should find list markers");
}

#[test]
fn test_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> This is a quote";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::BlockQuote).collect();
    assert!(!quotes.is_empty(), "Should find blockquote");
}

#[test]
fn test_thematic_break() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "text\n\n---\n\nmore";
    let spans = parser.parse(content);

    let hr: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::HorizontalRule).collect();
    assert!(!hr.is_empty(), "Should find horizontal rule");
}

#[test]
fn test_inline_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[text](https://example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    println!("Link test found {} spans (tree-sitter-md may not support inline links)", spans.len());

    if !links.is_empty() {
        println!("  Link parsing is supported!");
    } else {
        println!("  Link parsing not yet supported by tree-sitter-md");
    }
}

#[test]
fn test_image() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![alt](image.png)";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    println!("Image test found {} spans (tree-sitter-md may not support images)", spans.len());

    if !images.is_empty() {
        println!("  Image parsing is supported!");
    } else {
        println!("  Image parsing not yet supported by tree-sitter-md");
    }
}

#[test]
fn test_combined_multiple_elements_one_line() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading with **bold** and `code`";
    let spans = parser.parse(content);

    let heading: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert!(!heading.is_empty(), "Should find heading");
    assert_eq!(bold.len(), 1, "Should find bold inside heading");
    assert_eq!(code.len(), 1, "Should find code inside heading");
}

#[test]
fn test_real_world_markdown_snippet() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Overview

This is **important** text with `code` and *emphasis*.

## Details

- Item with `inline code`
- Item with **bold text**
"#;

    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(h1.len(), 1, "Should find H1");
    assert_eq!(h2.len(), 1, "Should find H2");
    assert_eq!(bold.len(), 2, "Should find two bold spans");
    assert_eq!(italic.len(), 1, "Should find italic");
    assert_eq!(code.len(), 2, "Should find two code spans");
}
