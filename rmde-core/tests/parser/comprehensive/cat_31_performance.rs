//! Category 31: Performance Test
//!
//! Tests for performance and stress testing with complex markdown
//! Status: Testing required
//!
//! Tests combining all formatting types to verify parser handles complexity

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_paragraph_with_all_inline_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This has **bold** and *italic* and `code` and [link](url) and ![image](url) and ~~strikethrough~~ and ==highlight== and $math$ all together.";
    let spans = parser.parse(content);

    // Should handle all formatting types in one paragraph
    assert!(spans.len() > 10, "Should produce many spans for complex paragraph");

    let has_bold = spans.iter().any(|s| matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic));
    let has_italic = spans.iter().any(|s| matches!(s.kind, SpanKind::Italic | SpanKind::BoldItalic));
    let has_code = spans.iter().any(|s| s.kind == SpanKind::CodeInline);
    let has_link = spans.iter().any(|s| s.kind == SpanKind::Link);

    assert!(has_bold, "Should find bold in complex paragraph");
    assert!(has_italic, "Should find italic in complex paragraph");
    assert!(has_code, "Should find code in complex paragraph");
    assert!(has_link, "Should find link in complex paragraph");
}

#[test]
fn test_list_with_all_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item with **bold**\n- Item with *italic*\n- Item with `code`\n- Item with [link](url)\n- Item with ~~strike~~";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(list_markers.len() >= 5, "Should find all list items");
    assert!(spans.len() > 15, "Should find all formatting in list items");
}

#[test]
fn test_table_with_all_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| **Bold** | *Italic* | `Code` | [Link](url) | ~~Strike~~ |\n|---|---|---|---|---|\n| a | b | c | d | e |";
    let spans = parser.parse(content);

    // Table with all formatting types in header
    let has_bold = spans.iter().any(|s| matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic));
    let has_italic = spans.iter().any(|s| matches!(s.kind, SpanKind::Italic | SpanKind::BoldItalic));
    let has_code = spans.iter().any(|s| s.kind == SpanKind::CodeInline);
    let has_link = spans.iter().any(|s| s.kind == SpanKind::Link);
    let has_strike = spans.iter().any(|s| s.kind == SpanKind::Strikethrough);

    assert!(has_bold, "Should find bold in table");
    assert!(has_italic, "Should find italic in table");
    assert!(has_code, "Should find code in table");
    assert!(has_link, "Should find link in table");
    assert!(has_strike, "Should find strikethrough in table");
}

#[test]
fn test_deeply_nested_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "***~~==bold italic strike highlight==~~***";
    let spans = parser.parse(content);

    // Deeply nested formatting
    let formatted: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind,
            SpanKind::Bold | SpanKind::Italic | SpanKind::BoldItalic |
            SpanKind::Strikethrough | SpanKind::Highlight
        )
    ).collect();

    assert!(formatted.len() >= 1, "Should handle deeply nested formatting");
}

#[test]
fn test_long_document_structure() {
    let mut parser = MarkdownParser::new().unwrap();
    let mut content = String::new();

    // Generate a long document with various elements
    for i in 1..=10 {
        content.push_str(&format!("# Heading {}\n\n", i));
        content.push_str("This is a paragraph with **bold** and *italic*.\n\n");
        content.push_str("- List item 1\n");
        content.push_str("- List item 2\n\n");
        content.push_str("> Blockquote\n\n");
        content.push_str("```\ncode block\n```\n\n");
    }

    let spans = parser.parse(&content);

    // Should handle long document efficiently
    assert!(spans.len() > 100, "Should parse long document with many spans");
}

#[test]
fn test_many_links_in_paragraph() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[link1](url1) [link2](url2) [link3](url3) [link4](url4) [link5](url5) [link6](url6) [link7](url7) [link8](url8) [link9](url9) [link10](url10)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    assert!(links.len() >= 10, "Should handle many links in one paragraph");
}

#[test]
fn test_alternating_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*a* b *c* d *e* f *g* h *i* j *k* l *m* n *o* p *q* r *s* t *u* v *w* x *y* z";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(emphasis.len() >= 10, "Should handle alternating emphasis");
}

#[test]
fn test_complex_table_with_many_rows() {
    let mut parser = MarkdownParser::new().unwrap();
    let mut content = String::from("| Col1 | Col2 | Col3 |\n|---|---|---|\n");

    for i in 1..=20 {
        content.push_str(&format!("| **Row {}** | *data* | `code` |\n", i));
    }

    let spans = parser.parse(&content);

    // Should handle large table with formatting
    assert!(spans.len() > 50, "Should parse large table with many formatted cells");
}

#[test]
fn test_nested_lists_deep() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Level 1\n  - Level 2\n    - Level 3\n      - Level 4\n        - Level 5";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(list_markers.len() >= 5, "Should handle deeply nested lists");
}

#[test]
fn test_mixed_content_stress() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"
# Heading 1
Paragraph with **bold**, *italic*, `code`, [link](url), and ==highlight==.

## Heading 2
- List item with **bold**
  - Nested with *italic*
    - Deep nested with `code`

> Blockquote with [link](url) and ~~strikethrough~~

```rust
fn main() {
    println!("code block");
}
```

| Header 1 | Header 2 |
|---|---|
| **Bold** | *Italic* |

### Heading 3
Final paragraph with $math$ and more.
"#;

    let spans = parser.parse(content);

    // Should handle complex mixed content
    assert!(spans.len() > 40, "Should parse complex mixed content document");
}

#[test]
fn test_repeated_parsing_same_content() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading\n\nParagraph with **bold** and *italic*.";

    // Parse the same content multiple times
    let spans1 = parser.parse(content);
    let spans2 = parser.parse(content);
    let spans3 = parser.parse(content);

    // Results should be consistent
    assert_eq!(spans1.len(), spans2.len(), "Repeated parsing should be consistent");
    assert_eq!(spans2.len(), spans3.len(), "Repeated parsing should be consistent");
}
