//! Category 17: Images
//!
//! Tests for ![alt](url), reference style images
//! Status: Testing required
//!
//! Test cases from example_test.md lines 401-414

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_image() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![Alt text](https://via.placeholder.com/150)";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    assert!(images.len() >= 1, "Should find image");
}

#[test]
fn test_image_with_title() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"![Image with title](https://via.placeholder.com/150 "Placeholder Image")"#;
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    let titles: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::LinkTitle).collect();

    assert!(images.len() >= 1, "Should find image");
    assert!(titles.len() >= 1, "Should find image title");
}

#[test]
fn test_reference_image() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![Reference image][img-ref]\n\n[img-ref]: https://via.placeholder.com/100";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    assert!(images.len() >= 1, "Should find reference image");
}

#[test]
fn test_image_in_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[![Clickable image](https://via.placeholder.com/100)](https://example.com)";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();

    assert!(images.len() >= 1, "Should find image");
    assert!(links.len() >= 1, "Should find link containing image");
}

#[test]
fn test_image_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![Alt](url)";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerImage).collect();

    assert!(images.len() >= 1, "Should find image");
    // Markers are optional depending on implementation
    if markers.len() > 0 {
        assert!(markers.len() >= 4, "Should find image markers (!, [, ], (, ))");
    }
}

#[test]
fn test_image_url_span() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![Alt](https://example.com/image.png)";
    let spans = parser.parse(content);

    let urls: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::LinkUrl).collect();
    assert!(urls.len() >= 1, "Should find LinkUrl span for image URL");
}
