//! Category 16: Links
//!
//! Tests for [text](url), reference links, and links with formatting
//! Status: Testing required
//!
//! Test cases from example_test.md lines 364-398

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_inline_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Simple link](https://example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find inline link");
}

#[test]
fn test_link_with_title() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"[Link with title](https://example.com "Example Title")"#;
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let titles: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::LinkTitle).collect();

    assert!(links.len() >= 1, "Should find link");
    assert!(titles.len() >= 1, "Should find link title");
}

#[test]
fn test_link_in_angle_brackets() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Link in angle brackets](<https://example.com>)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find link with angle brackets");
}

#[test]
fn test_relative_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Relative link](/path/to/file)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find relative link");
}

#[test]
fn test_anchor_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Anchor link](#headings)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find anchor link");
}

#[test]
fn test_email_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Email link](mailto:email@example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find email link");
}

#[test]
fn test_full_reference_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Full reference][ref1]\n\n[ref1]: https://example.com";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find reference link");
}

#[test]
fn test_collapsed_reference_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Collapsed reference][]\n\n[collapsed reference]: https://example.com";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    assert!(links.len() >= 1, "Should find collapsed reference link");
}

#[test]
fn test_shortcut_reference_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Shortcut reference]\n\n[shortcut reference]: https://example.com";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    // Shortcut reference may require special handling
    assert!(links.len() >= 1, "Should find shortcut reference link");
}

#[test]
fn test_link_with_bold_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[**Bold link text**](https://example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert!(links.len() >= 1, "Should find link");
    assert!(bold.len() >= 1, "Should find bold text inside link");
}

#[test]
fn test_link_with_italic_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[*Italic link text*](https://example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(links.len() >= 1, "Should find link");
    assert!(italic.len() >= 1, "Should find italic text inside link");
}

#[test]
fn test_link_with_code_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[`Code link text`](https://example.com)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert!(links.len() >= 1, "Should find link");
    assert!(code.len() >= 1, "Should find code text inside link");
}

#[test]
fn test_link_url_span() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[Link](https://example.com)";
    let spans = parser.parse(content);

    let urls: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::LinkUrl).collect();
    assert!(urls.len() >= 1, "Should find LinkUrl span for the URL part");
}
