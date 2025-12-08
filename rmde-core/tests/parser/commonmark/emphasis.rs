//! Emphasis tests - Bold, Italic, and nested emphasis
//!
//! Tests for CommonMark emphasis syntax with flanking rules per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_parse_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("This is **bold** text");

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!bold.is_empty(), "Should find bold");
    assert_eq!(bold[0].start, 8);
    assert_eq!(bold[0].end, 16);
}

#[test]
fn test_parse_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("This is *italic* text");

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(!italic.is_empty(), "Should find italic");
}

#[test]
fn test_emphasis_underscore_intraword() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo_bar_baz";
    let spans = parser.parse(content);

    let emphasis: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Italic || s.kind == SpanKind::Bold
    ).collect();
    assert_eq!(emphasis.len(), 0, "Underscores mid-word should not create emphasis");
}

#[test]
fn test_emphasis_asterisk_intraword() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "foo*bar*baz";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert_eq!(italic.len(), 1, "Asterisks mid-word should create emphasis");
    assert_eq!(&content[italic[0].start..italic[0].end], "*bar*");
}

#[test]
fn test_emphasis_nested_bold_in_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*foo **bar** baz*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert_eq!(italic.len(), 1, "Should find outer italic");
    assert_eq!(bold.len(), 1, "Should find inner bold");
    assert_eq!(&content[italic[0].start..italic[0].end], "*foo **bar** baz*");
    assert_eq!(&content[bold[0].start..bold[0].end], "**bar**");
}

#[test]
fn test_emphasis_nested_italic_in_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**foo *bar* baz**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert_eq!(bold.len(), 1, "Should find outer bold");
    assert_eq!(italic.len(), 1, "Should find inner italic");
    assert_eq!(&content[bold[0].start..bold[0].end], "**foo *bar* baz**");
    assert_eq!(&content[italic[0].start..italic[0].end], "*bar*");
}

#[test]
fn test_emphasis_triple_asterisk() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "***bold italic***";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(bold.len() >= 1 || italic.len() >= 1,
            "Triple asterisk should create emphasis (found {} bold, {} italic)",
            bold.len(), italic.len());
}

#[test]
fn test_emphasis_simple_cases_still_work() {
    let mut parser = MarkdownParser::new().unwrap();

    parser.reset();
    let bold_content = "This is **bold** text";
    let bold_spans = parser.parse(bold_content);
    let bold: Vec<_> = bold_spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(bold.len(), 1, "Simple bold should work");
    assert_eq!(&bold_content[bold[0].start..bold[0].end], "**bold**");

    parser.reset();
    let italic_content = "This is *italic* text";
    let italic_spans = parser.parse(italic_content);
    let italic: Vec<_> = italic_spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert_eq!(italic.len(), 1, "Simple italic should work");
    assert_eq!(&italic_content[italic[0].start..italic[0].end], "*italic*");

    parser.reset();
    let under_bold = "This is __bold__ text";
    let under_bold_spans = parser.parse(under_bold);
    let under_bold_vec: Vec<_> = under_bold_spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(under_bold_vec.len(), 1, "Underscore bold should work");

    parser.reset();
    let under_italic = "This is _italic_ text";
    let under_italic_spans = parser.parse(under_italic);
    let under_italic_vec: Vec<_> = under_italic_spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert_eq!(under_italic_vec.len(), 1, "Underscore italic should work");
}

#[test]
fn test_emphasis_comprehensive_examples() {
    let mut parser = MarkdownParser::new().unwrap();

    let test_cases = vec![
        ("This is **bold** text", 1, 0, "Simple bold"),
        ("This is *italic* text", 0, 1, "Simple italic"),
        ("**bold** and *italic*", 1, 1, "Both in same line"),
        ("foo_bar_baz", 0, 0, "Underscores mid-word should not create emphasis"),
        ("foo*bar*baz", 0, 1, "Asterisks mid-word should create emphasis"),
        ("This**is**fine", 1, 0, "Bold mid-word with asterisks works"),
    ];

    for (content, expected_bold, expected_italic, desc) in test_cases {
        parser.reset();
        let spans = parser.parse(content);
        let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
        let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

        assert_eq!(bold.len(), expected_bold,
            "{}: expected {} bold, got {}", desc, expected_bold, bold.len());
        assert_eq!(italic.len(), expected_italic,
            "{}: expected {} italic, got {}", desc, expected_italic, italic.len());
    }
}

#[test]
fn test_emphasis_unclosed() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*unclosed emphasis";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert_eq!(italic.len(), 0, "Unclosed emphasis should not match");
}

#[test]
fn test_emphasis_escaped_delimiters() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\*not emphasis\*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    println!("Escaped delimiters found {} italic spans", italic.len());
}

#[test]
fn test_emphasis_adjacent() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**bold1** **bold2**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(bold.len(), 2, "Adjacent bold should create two spans");
    assert_eq!(&content[bold[0].start..bold[0].end], "**bold1**");
    assert_eq!(&content[bold[1].start..bold[1].end], "**bold2**");
}

#[test]
fn test_emphasis_empty_delimiters() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ** ** more";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    println!("Empty delimiters found {} bold spans", bold.len());
}

#[test]
fn test_emphasis_underscores_at_word_boundaries() {
    let mut parser = MarkdownParser::new().unwrap();

    let content = "This is _italic_ text";
    let spans = parser.parse(content);
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert_eq!(italic.len(), 1, "Underscores at word boundaries should work");
}

#[test]
fn test_emphasis_complex_nesting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*foo **bar** baz*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert_eq!(italic.len(), 1, "Should find outer italic");
    assert_eq!(bold.len(), 1, "Should find nested bold");
}

#[test]
fn test_emphasis_whitespace_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text * * more";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    println!("Whitespace-only emphasis found {} spans", italic.len());
}

#[test]
fn test_emphasis_with_unicode() {
    let mut parser = MarkdownParser::new().unwrap();

    let content = "Hello 👋 *world*";
    let spans = parser.parse(content);
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(!italic.is_empty(), "Should find italic after emoji");
    assert_eq!(&content[italic[0].start..italic[0].end], "*world*");

    let content2 = "你好**世界**";
    parser.reset();
    let spans2 = parser.parse(content2);
    let bold: Vec<_> = spans2.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!bold.is_empty(), "Should find bold in Chinese text");
    assert_eq!(&content2[bold[0].start..bold[0].end], "**世界**");

    parser.reset();
    let content3 = "🎉 Party **time** 🎊";
    let spans3 = parser.parse(content3);
    let bold3: Vec<_> = spans3.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!bold3.is_empty(), "Should find bold between emojis");
    assert_eq!(&content3[bold3[0].start..bold3[0].end], "**time**");

    parser.reset();
    let content4 = "Café *résumé* naïve";
    let spans4 = parser.parse(content4);
    let italic4: Vec<_> = spans4.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(!italic4.is_empty(), "Should find italic with accented chars");
    assert_eq!(&content4[italic4[0].start..italic4[0].end], "*résumé*");
}
