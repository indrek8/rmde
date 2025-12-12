//! Category 26: Escaping Special Characters
//!
//! Tests for \* \[ \\ and other escaped characters
//! Status: Testing required
//!
//! Test cases for backslash escaping of special markdown characters

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_escape_asterisk() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\*not emphasis\*";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert_eq!(emphasis.len(), 0, "Escaped asterisks should not create emphasis");
}

#[test]
fn test_escape_underscore() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\_not emphasis\_";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert_eq!(emphasis.len(), 0, "Escaped underscores should not create emphasis");
}

#[test]
fn test_escape_backtick() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\`not code\`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeInline
    ).collect();

    assert_eq!(code.len(), 0, "Escaped backticks should not create code span");
}

#[test]
fn test_escape_bracket() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\[not a link\](url)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    assert_eq!(links.len(), 0, "Escaped brackets should not create link");
}

#[test]
fn test_escape_backslash() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\\literal backslash";
    let spans = parser.parse(content);

    // Escaped backslash should render as a single backslash
    // This just verifies parsing doesn't fail
    assert!(spans.len() > 0, "Escaped backslash should parse correctly");
}

#[test]
fn test_escape_hash() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\# not a heading";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind,
            SpanKind::Heading1 | SpanKind::Heading2 | SpanKind::Heading3 |
            SpanKind::Heading4 | SpanKind::Heading5 | SpanKind::Heading6
        )
    ).collect();

    assert_eq!(headings.len(), 0, "Escaped hash should not create heading");
}

#[test]
fn test_escape_angle_bracket() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\<not an autolink\>";
    let spans = parser.parse(content);

    let autolinks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Autolink
    ).collect();

    assert_eq!(autolinks.len(), 0, "Escaped angle brackets should not create autolink");
}

#[test]
fn test_multiple_escapes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\*\*\*not bold italic\*\*\*";
    let spans = parser.parse(content);

    let formatted: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind,
            SpanKind::Bold | SpanKind::Italic | SpanKind::BoldItalic | SpanKind::Emphasis
        )
    ).collect();

    assert_eq!(formatted.len(), 0, "All escaped asterisks should not create formatting");
}

#[test]
fn test_escape_in_middle_of_word() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"foo\*bar";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert_eq!(emphasis.len(), 0, "Escaped asterisk in word should not create emphasis");
}

#[test]
fn test_backslash_before_non_special() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\a backslash before non-special character";
    let spans = parser.parse(content);

    // Backslash before non-special character should remain as literal backslash
    assert!(spans.len() > 0, "Should parse backslash before non-special character");
}

#[test]
fn test_escape_tilde() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\~\~not strikethrough\~\~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Strikethrough
    ).collect();

    assert_eq!(strike.len(), 0, "Escaped tildes should not create strikethrough");
}

#[test]
fn test_escape_equals() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\=\=not highlight\=\=";
    let spans = parser.parse(content);

    let highlight: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Highlight
    ).collect();

    assert_eq!(highlight.len(), 0, "Escaped equals should not create highlight");
}
