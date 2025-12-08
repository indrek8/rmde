//! Autolink tests
//!
//! Tests for GFM autolink syntax per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_autolink_http() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit <http://example.com> for more";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find one HTTP autolink");
    assert_eq!(autolinks[0].start, 6, "Autolink should start at correct position");
    assert_eq!(autolinks[0].end, 26, "Autolink should end at correct position");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<http://example.com>", "Should capture full autolink with angle brackets");
}

#[test]
fn test_autolink_https() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit <https://example.com> for more";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find one HTTPS autolink");
    assert_eq!(autolinks[0].start, 6, "Autolink should start at correct position");
    assert_eq!(autolinks[0].end, 27, "Autolink should end at correct position");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<https://example.com>", "Should capture full HTTPS autolink");
}

#[test]
fn test_autolink_email() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Contact me at <email@example.com> please";
    let spans = parser.parse(content);

    let emails: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::AutolinkEmail).collect();
    assert_eq!(emails.len(), 1, "Should find one email autolink");
    assert_eq!(emails[0].start, 14, "Email autolink should start at correct position");
    assert_eq!(emails[0].end, 33, "Email autolink should end at correct position");

    let email_text = &content[emails[0].start..emails[0].end];
    assert_eq!(email_text, "<email@example.com>", "Should capture full email autolink");
}

#[test]
fn test_autolink_gfm_bare_https() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit https://example.com for more";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find one bare HTTPS URL");
    assert_eq!(autolinks[0].start, 6, "Bare URL should start at correct position");
    assert_eq!(autolinks[0].end, 25, "Bare URL should end at correct position");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "https://example.com", "Should capture bare URL without angle brackets");
}

#[test]
fn test_autolink_gfm_www() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit www.example.com for more";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find one www URL");
    assert_eq!(autolinks[0].start, 6, "www URL should start at correct position");
    assert_eq!(autolinks[0].end, 21, "www URL should end at correct position");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "www.example.com", "Should capture www URL");
}

#[test]
fn test_autolink_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Don't parse `<https://example.com>` in code";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(code.len(), 1, "Should find code span");
    assert_eq!(autolinks.len(), 0, "Should not find autolinks inside code spans");
}

#[test]
fn test_autolink_multiple() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit <https://example.com> or email <user@example.org> or go to www.test.com";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    let emails: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::AutolinkEmail).collect();

    assert_eq!(autolinks.len(), 2, "Should find two autolinks (HTTPS and www)");
    assert_eq!(emails.len(), 1, "Should find one email autolink");
}

#[test]
fn test_autolink_with_path() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "See <https://example.com/path/to/page?query=1#section> for details";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find autolink with path and query");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<https://example.com/path/to/page?query=1#section>", "Should capture full URL");
}

#[test]
fn test_autolink_trailing_punctuation() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit https://example.com. Next sentence.";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "https://example.com", "Should not include trailing period");
}

#[test]
fn test_autolink_invalid() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Not a link: <not a url> or <incomplete";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    let emails: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::AutolinkEmail).collect();

    assert_eq!(autolinks.len(), 0, "Should not find invalid autolinks");
    assert_eq!(emails.len(), 0, "Should not find invalid email autolinks");
}

#[test]
fn test_autolink_ftp() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Download from <ftp://ftp.example.com/file.txt> now";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find FTP autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<ftp://ftp.example.com/file.txt>", "Should capture FTP URL");
}

#[test]
fn test_autolink_mailto() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Email us at <mailto:user@example.com> for help";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find mailto autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<mailto:user@example.com>", "Should capture mailto URL");
}

#[test]
fn test_autolink_tel() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Call us at <tel:+1-555-555-5555> today";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find tel autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<tel:+1-555-555-5555>", "Should capture tel URL");
}

#[test]
fn test_autolink_ssh() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Connect via <ssh://user@host.com> for access";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find SSH autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<ssh://user@host.com>", "Should capture SSH URL");
}

#[test]
fn test_autolink_file() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Open <file:///path/to/file> locally";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 1, "Should find file autolink");

    let link_text = &content[autolinks[0].start..autolinks[0].end];
    assert_eq!(link_text, "<file:///path/to/file>", "Should capture file URL");
}

#[test]
fn test_autolink_multiple_schemes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Try <ftp://files.com>, <mailto:help@example.com>, or <tel:555-1234>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 3, "Should find all three autolinks with different schemes");
}
