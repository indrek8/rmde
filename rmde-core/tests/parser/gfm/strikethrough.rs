//! Strikethrough tests
//!
//! Tests for GFM strikethrough syntax (~~text~~) per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_strikethrough_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ~~strikethrough~~ text";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 1, "Should find one strikethrough span");
    assert_eq!(strike[0].start, 8);
    assert_eq!(strike[0].end, 25);
    assert_eq!(&content[strike[0].start..strike[0].end], "~~strikethrough~~");
}

#[test]
fn test_strikethrough_empty() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ~~~~ more";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 0, "Empty strikethrough ~~~~ should not match");
}

#[test]
fn test_strikethrough_unclosed() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ~~unclosed";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 0, "Unclosed strikethrough should not match");
}

#[test]
fn test_strikethrough_multiple() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ~~one~~ and ~~two~~ more";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 2, "Should find two strikethrough spans");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~one~~");
    assert_eq!(&content[strike[1].start..strike[1].end], "~~two~~");
}

#[test]
fn test_strikethrough_with_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**bold ~~strike~~**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(bold.len(), 1, "Should find bold");
    assert_eq!(strike.len(), 1, "Should find strikethrough inside bold");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~strike~~");
}

#[test]
fn test_strikethrough_inside_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`~~not strikethrough~~`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(code.len(), 1, "Should find code span");
    assert_eq!(strike.len(), 0, "Strikethrough markers inside code should not be parsed");
}

#[test]
fn test_strikethrough_single_tilde() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ~single tilde~ more";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 0, "Single tildes should not create strikethrough");
}

#[test]
fn test_strikethrough_triple_tilde() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ~~~triple~~~ more";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    println!("Triple tilde test found {} strikethrough spans", strike.len());
}

#[test]
fn test_strikethrough_with_italic_and_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*italic with ~~strike~~ inside* and **bold ~~strike~~ too**";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(italic.len(), 1, "Should find italic");
    assert_eq!(bold.len(), 1, "Should find bold");
    assert_eq!(strike.len(), 2, "Should find two strikethrough spans");
}

#[test]
fn test_strikethrough_adjacent() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~first~~~~second~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 2, "Adjacent strikethrough should create two spans");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~first~~");
    assert_eq!(&content[strike[1].start..strike[1].end], "~~second~~");
}

#[test]
fn test_strikethrough_adjacent_short() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~a~~~~b~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 2, "Adjacent short strikethrough should create two spans");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~a~~");
    assert_eq!(&content[strike[1].start..strike[1].end], "~~b~~");
}

#[test]
fn test_strikethrough_with_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~text with spaces~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 1, "Strikethrough should work with spaces");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~text with spaces~~");
}

#[test]
fn test_strikethrough_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~text\nacross lines~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 1, "Strikethrough should work across lines");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~text\nacross lines~~");
}

#[test]
fn test_strikethrough_with_unicode() {
    let mut parser = MarkdownParser::new().unwrap();

    let content = "~~deleted 👋 text~~";
    let spans = parser.parse(content);
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 1, "Should find strikethrough with emoji");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~deleted 👋 text~~");

    parser.reset();
    let content2 = "保留 ~~删除~~ 文本";
    let spans2 = parser.parse(content2);
    let strike2: Vec<_> = spans2.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike2.len(), 1, "Should find strikethrough in Chinese text");
    assert_eq!(&content2[strike2[0].start..strike2[0].end], "~~删除~~");
}

#[test]
fn test_strikethrough_real_world() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Task List

- [x] ~~Completed task~~
- [ ] **Important:** ~~Old approach~~ Use new method
- [ ] Review `code` and ~~remove deprecated~~
"#;

    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 3, "Should find three strikethrough spans in real-world example");
}
