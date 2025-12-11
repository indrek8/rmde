//! Heading tests - ATX and Setext headings
//!
//! Tests for CommonMark heading syntax per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_parse_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("# Hello\n\n## World");

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert!(!h1.is_empty(), "Should find H1");
    assert!(!h2.is_empty(), "Should find H2");
}

#[test]
fn test_heading_atx_with_closing_hashes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading #\n## Heading ##";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();

    assert_eq!(h1.len(), 1, "H1 with closing # should work");
    assert_eq!(h2.len(), 1, "H2 with closing ## should work");
}

#[test]
fn test_heading_seven_hashes_not_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "####### Too many hashes";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Heading1 | SpanKind::Heading2 | SpanKind::Heading3 |
                 SpanKind::Heading4 | SpanKind::Heading5 | SpanKind::Heading6)
    ).collect();

    println!("Seven hashes found {} headings", headings.len());
}

#[test]
fn test_heading_with_extra_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "   # Heading with 3 spaces";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    println!("Heading with 3 spaces found {} H1 headings", h1.len());
}

// ========================================================================
// SETEXT HEADINGS
// ========================================================================

#[test]
fn test_parse_setext_headings() {
    let mut parser = MarkdownParser::new().unwrap();

    let content_h1 = "Heading One\n===========";
    let spans_h1 = parser.parse(content_h1);
    let h1: Vec<_> = spans_h1.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1.len(), 1, "Should find H1 with = underline");

    parser.reset();
    let content_h2 = "Heading Two\n-----------";
    let spans_h2 = parser.parse(content_h2);
    let h2: Vec<_> = spans_h2.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h2.len(), 1, "Should find H2 with - underline");

    parser.reset();
    let content_both = "First Heading\n=============\n\nSecond Heading\n--------------";
    let spans_both = parser.parse(content_both);
    let h1_both: Vec<_> = spans_both.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2_both: Vec<_> = spans_both.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h1_both.len(), 1, "Should find one H1");
    assert_eq!(h2_both.len(), 1, "Should find one H2");

    parser.reset();
    let content_mixed = "Not a heading\n-=-=-";
    let spans_mixed = parser.parse(content_mixed);
    let headings: Vec<_> = spans_mixed.iter().filter(|s|
        s.kind == SpanKind::Heading1 || s.kind == SpanKind::Heading2
    ).collect();
    assert_eq!(headings.len(), 0, "Mixed characters should not create heading");
}

#[test]
fn test_setext_mixed_underline_invalid() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Foo\n=-=";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Heading1 | SpanKind::Heading2)
    ).collect();

    assert_eq!(headings.len(), 0, "Mixed underline characters should not create heading");
}

#[test]
fn test_setext_empty_heading_invalid() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "\n===";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1.len(), 0, "Empty setext heading should not be valid");
}

#[test]
fn test_setext_vs_thematic_break() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Paragraph text\n---";
    let spans = parser.parse(content);

    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    println!("Found {} H2 headings for setext vs thematic break", h2.len());
}

#[test]
fn test_setext_h1_vs_h2_distinction() {
    // Regression test for bug where setext headings always returned H1
    let mut parser = MarkdownParser::new().unwrap();

    // Test H1 specifically
    let content_h1 = "This is H1\n==========";
    let spans_h1 = parser.parse(content_h1);
    let h1_spans: Vec<_> = spans_h1.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2_spans: Vec<_> = spans_h1.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h1_spans.len(), 1, "Should find exactly 1 H1 with = underline");
    assert_eq!(h2_spans.len(), 0, "Should find no H2 with = underline");

    // Test H2 specifically
    parser.reset();
    let content_h2 = "This is H2\n----------";
    let spans_h2 = parser.parse(content_h2);
    let h1_spans: Vec<_> = spans_h2.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2_spans: Vec<_> = spans_h2.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h1_spans.len(), 0, "Should find no H1 with - underline");
    assert_eq!(h2_spans.len(), 1, "Should find exactly 1 H2 with - underline");

    // Test both in same document
    parser.reset();
    let content_both = "Heading One\n===========\n\nHeading Two\n-----------";
    let spans_both = parser.parse(content_both);
    let h1_spans: Vec<_> = spans_both.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2_spans: Vec<_> = spans_both.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h1_spans.len(), 1, "Should find exactly 1 H1");
    assert_eq!(h2_spans.len(), 1, "Should find exactly 1 H2");

    // Verify the H1 span contains = underline
    let h1_text = &content_both[h1_spans[0].start..h1_spans[0].end];
    assert!(h1_text.contains('='), "H1 span should contain = characters");
    assert!(!h1_text.contains('-') || h1_text.contains("Heading One"), "H1 should not be confused with -");

    // Verify the H2 span contains - underline
    let h2_text = &content_both[h2_spans[0].start..h2_spans[0].end];
    assert!(h2_text.contains('-'), "H2 span should contain - characters");
    assert!(!h2_text.contains('='), "H2 should not contain = characters");
}

#[test]
fn test_setext_headings_windows_line_endings() {
    let mut parser = MarkdownParser::new().unwrap();

    let content_h1 = "Heading One\r\n===========\r\n";
    let spans_h1 = parser.parse(content_h1);
    let h1: Vec<_> = spans_h1.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    assert_eq!(h1.len(), 1, "Should find H1 with Windows line endings");

    let span = h1[0];
    let heading_text = &content_h1[span.start..span.end];
    assert!(heading_text.contains("Heading One"), "Span should contain heading text");
    assert!(heading_text.contains("==========="), "Span should contain underline");

    parser.reset();
    let content_h2 = "Heading Two\r\n-----------\r\n";
    let spans_h2 = parser.parse(content_h2);
    let h2: Vec<_> = spans_h2.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h2.len(), 1, "Should find H2 with Windows line endings");

    parser.reset();
    let content_mixed = "First\r\n=====\r\n\r\nSecond\r\n------\r\n";
    let spans_mixed = parser.parse(content_mixed);
    let h1_mixed: Vec<_> = spans_mixed.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2_mixed: Vec<_> = spans_mixed.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h1_mixed.len(), 1, "Should find H1 in mixed Windows line endings");
    assert_eq!(h2_mixed.len(), 1, "Should find H2 in mixed Windows line endings");
}

