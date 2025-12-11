//! Category 18: Autolinks
//!
//! Tests for <url>, bare URLs (GFM)
//! Status: Testing required
//!
//! Test cases from example_test.md lines 417-436

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_standard_https_autolink() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<https://example.com>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert!(autolinks.len() >= 1, "Should find HTTPS autolink");
}

#[test]
fn test_standard_http_autolink() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<http://example.com/path?query=value>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert!(autolinks.len() >= 1, "Should find HTTP autolink with path and query");
}

#[test]
fn test_mailto_autolink() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<mailto:email@example.com>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink || s.kind == SpanKind::AutolinkEmail).collect();
    assert!(autolinks.len() >= 1, "Should find mailto autolink");
}

#[test]
fn test_email_autolink() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<email@example.com>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::AutolinkEmail).collect();
    assert!(autolinks.len() >= 1, "Should find email autolink");
}

#[test]
fn test_ssh_autolink() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<ssh://user@host.com>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert!(autolinks.len() >= 1, "Should find SSH autolink");
}

#[test]
fn test_gfm_bare_url_https() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit https://example.com for more info.";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    // GFM extension - bare URLs should be autolinked
    assert!(autolinks.len() >= 1, "Should find bare HTTPS URL (GFM)");
}

#[test]
fn test_gfm_bare_url_www() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Check out www.example.com today.";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    // GFM extension - www URLs should be autolinked
    assert!(autolinks.len() >= 1, "Should find bare www URL (GFM)");
}

#[test]
fn test_gfm_bare_email() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Email us at user@example.com for support.";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::AutolinkEmail).collect();
    // GFM extension - bare emails should be autolinked
    assert!(autolinks.len() >= 1, "Should find bare email address (GFM)");
}

#[test]
fn test_multiple_autolinks_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit <https://example.com> or <http://test.com> for info.";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert!(autolinks.len() >= 2, "Should find multiple autolinks");
}
