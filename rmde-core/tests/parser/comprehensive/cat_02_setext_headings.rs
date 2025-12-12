//! Category 2: Setext Headings
//!
//! Tests for underline-style headings (= for H1, - for H2)
//! Status: ISSUES DETECTED (underlines showing red in screenshots)
//!
//! Test cases from example_test.md lines 23-34

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_setext_heading_level_1() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Setext Heading Level 1\n======================";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1.len(), 1, "Setext H1 with = underline should parse");
}

#[test]
fn test_setext_heading_level_2() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Setext Heading Level 2\n----------------------";
    let spans = parser.parse(content);

    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h2.len(), 1, "Setext H2 with - underline should parse");
}

#[test]
fn test_multi_line_setext_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Multi-line\nSetext Heading\n--------------";
    let spans = parser.parse(content);

    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h2.len(), 1, "Multi-line setext heading should parse");

    // Verify the heading span includes both lines of text
    if !h2.is_empty() {
        let heading_text = &content[h2[0].start..h2[0].end];
        assert!(heading_text.contains("Multi-line"), "Should include first line");
        assert!(heading_text.contains("Setext Heading"), "Should include second line");
    }
}

#[test]
fn test_setext_underline_length_varies() {
    let mut parser = MarkdownParser::new().unwrap();

    // Short underline (just one character should work)
    let content1 = "Heading\n=";
    parser.reset();
    let spans1 = parser.parse(content1);
    let h1_short: Vec<_> = spans1.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1_short.len(), 1, "Single = character should create H1");

    // Long underline
    let content2 = "Heading\n========================================";
    parser.reset();
    let spans2 = parser.parse(content2);
    let h1_long: Vec<_> = spans2.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1_long.len(), 1, "Long === underline should create H1");
}
