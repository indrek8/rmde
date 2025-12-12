//! Category 9: Thematic Breaks (Horizontal Rules)
//!
//! Tests for ---, ***, ___ horizontal rule syntax
//! Status: Likely OK
//!
//! Test cases from example_test.md lines 226-249

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_thematic_break_three_dashes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "---";
    let spans = parser.parse(content);

    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks.len(), 1, "Three dashes should create thematic break");
}

#[test]
fn test_thematic_break_three_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "***";
    let spans = parser.parse(content);

    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks.len(), 1, "Three asterisks should create thematic break");
}

#[test]
fn test_thematic_break_three_underscores() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "___";
    let spans = parser.parse(content);

    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks.len(), 1, "Three underscores should create thematic break");
}

#[test]
fn test_thematic_break_with_spaces() {
    let mut parser = MarkdownParser::new().unwrap();

    // Asterisks with spaces
    let content1 = "* * *";
    let spans1 = parser.parse(content1);
    let breaks1: Vec<_> = spans1.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks1.len(), 1, "Asterisks with spaces should create thematic break");

    // Dashes with spaces
    parser.reset();
    let content2 = "- - -";
    let spans2 = parser.parse(content2);
    let breaks2: Vec<_> = spans2.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks2.len(), 1, "Dashes with spaces should create thematic break");
}

#[test]
fn test_thematic_break_many_characters() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "_______________";
    let spans = parser.parse(content);

    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks.len(), 1, "Many underscores should create thematic break");
}

#[test]
fn test_thematic_break_requires_three_characters() {
    let mut parser = MarkdownParser::new().unwrap();

    // Two dashes should not create a thematic break
    let content = "--";
    let spans = parser.parse(content);
    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks.len(), 0, "Two dashes should not create thematic break (minimum is 3)");
}

#[test]
fn test_thematic_break_indented() {
    let mut parser = MarkdownParser::new().unwrap();

    // Up to 3 spaces is allowed
    let content1 = "   ---";
    let spans1 = parser.parse(content1);
    let breaks1: Vec<_> = spans1.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks1.len(), 1, "Thematic break with 3 spaces indent should work");

    // 4 spaces makes it a code block
    parser.reset();
    let content2 = "    ---";
    let spans2 = parser.parse(content2);
    let breaks2: Vec<_> = spans2.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();
    assert_eq!(breaks2.len(), 0, "Thematic break with 4 spaces should not work (becomes code)");
}

#[test]
fn test_thematic_break_vs_setext_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Not a heading\n---";
    let spans = parser.parse(content);

    // This could be either a setext H2 heading or a thematic break
    // depending on context. CommonMark says this is a setext heading.
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let breaks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::HorizontalRule
    ).collect();

    println!("Found {} H2 headings and {} thematic breaks", h2.len(), breaks.len());
}
