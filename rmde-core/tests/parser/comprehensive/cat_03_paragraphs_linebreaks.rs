//! Category 3: Paragraphs and Line Breaks
//!
//! Tests for paragraph text and hard/soft line breaks
//! Status: Likely OK
//!
//! Test cases from example_test.md lines 37-50

use rmde_core::MarkdownParser;

#[test]
fn test_soft_line_breaks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is a paragraph with\nsoft line breaks that will\nbecome spaces in output.";
    let spans = parser.parse(content);

    // Soft line breaks should be part of paragraph text
    // Parser doesn't track plain paragraph text separately - it's between markers
    println!("Soft line breaks test - found {} spans total", spans.len());
    assert!(!spans.is_empty(), "Should have some spans even for plain paragraphs");
}

#[test]
fn test_paragraph_separation() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is a paragraph with\nsoft line breaks that will\nbecome spaces in output.\n\nThis is another paragraph separated by a blank line.";
    let spans = parser.parse(content);

    // Parser tracks markers, not paragraph text itself
    // Separate paragraphs should still parse without errors
    println!("Paragraph separation test - found {} spans total", spans.len());
    assert!(!spans.is_empty(), "Should have spans for content");
}

#[test]
fn test_hard_line_break_with_two_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Hard line break with two spaces:  \nThis line starts after a hard break.";
    let spans = parser.parse(content);

    // The parser tracks markdown markers, not hard breaks explicitly
    // This test documents that hard breaks (two trailing spaces) parse without error
    println!("Hard line break with two spaces - found {} spans total", spans.len());
    assert!(!spans.is_empty(), "Content should parse");
}

#[test]
fn test_hard_line_break_with_backslash() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Hard line break with backslash:\\\nThis line also starts after a hard break.";
    let spans = parser.parse(content);

    // The parser tracks markdown markers, not hard breaks explicitly
    // This test documents that hard breaks (backslash) parse without error
    println!("Hard line break with backslash - found {} spans total", spans.len());
    assert!(!spans.is_empty(), "Content should parse");
}

#[test]
fn test_multiple_paragraphs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
    let spans = parser.parse(content);

    // Parser tracks markers, not paragraph text itself
    // Multiple paragraphs should parse without errors
    println!("Multiple paragraphs test - found {} spans total", spans.len());
    assert!(!spans.is_empty(), "Should have spans for content");
}
