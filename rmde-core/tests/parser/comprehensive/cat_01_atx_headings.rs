//! Category 1: ATX Headings
//!
//! Tests for # through ###### heading syntax
//! Status: Likely OK
//!
//! Test cases from example_test.md lines 7-20

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_all_six_heading_levels() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading Level 1\n## Heading Level 2\n### Heading Level 3\n#### Heading Level 4\n##### Heading Level 5\n###### Heading Level 6";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    let h4: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading4).collect();
    let h5: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading5).collect();
    let h6: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading6).collect();

    assert_eq!(h1.len(), 1, "Should find exactly one H1");
    assert_eq!(h2.len(), 1, "Should find exactly one H2");
    assert_eq!(h3.len(), 1, "Should find exactly one H3");
    assert_eq!(h4.len(), 1, "Should find exactly one H4");
    assert_eq!(h5.len(), 1, "Should find exactly one H5");
    assert_eq!(h6.len(), 1, "Should find exactly one H6");
}

#[test]
fn test_heading_with_closing_hashes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "### With Closing Hashes ###";
    let spans = parser.parse(content);

    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    assert_eq!(h3.len(), 1, "Heading with closing hashes should parse correctly");
}

#[test]
fn test_heading_with_many_closing_hashes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "## Heading ##########";
    let spans = parser.parse(content);

    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    assert_eq!(h2.len(), 1, "Heading with many closing hashes should parse correctly");
}

#[test]
fn test_heading_with_three_space_indent() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "   ### Heading with 3 spaces indent";
    let spans = parser.parse(content);

    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    assert_eq!(h3.len(), 1, "Heading with up to 3 spaces indent should be valid");
}

#[test]
fn test_heading_with_four_space_indent_not_heading() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "    ### Not a heading (4 spaces = code block)";
    let spans = parser.parse(content);

    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    assert_eq!(h3.len(), 0, "Heading with 4+ spaces should not be valid (becomes code block)");
}
