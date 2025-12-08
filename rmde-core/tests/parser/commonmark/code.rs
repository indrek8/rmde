//! Code span and code block tests
//!
//! Tests for CommonMark code syntax per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_parse_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("Use `code` here");

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert!(!code.is_empty(), "Should find inline code");
}

#[test]
fn test_parse_code_single_backtick() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `code` here";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Should find one code span");
    assert_eq!(code[0].start, 4);
    assert_eq!(code[0].end, 10);
    assert_eq!(&content[code[0].start..code[0].end], "`code`");
}

#[test]
fn test_parse_code_double_backtick() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use ``code`` here";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Should find one code span");
    assert_eq!(code[0].start, 4);
    assert_eq!(code[0].end, 12);
    assert_eq!(&content[code[0].start..code[0].end], "``code``");
}

#[test]
fn test_parse_code_backtick_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `` `inner` `` here";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Should find one code span");
    assert_eq!(code[0].start, 4);
    assert_eq!(code[0].end, 17);
    assert_eq!(&content[code[0].start..code[0].end], "`` `inner` ``");
}

#[test]
fn test_parse_code_triple_backtick_inline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Inline ```code``` works";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Should find one code span");
    assert_eq!(code[0].start, 7);
    assert_eq!(code[0].end, 17);
    assert_eq!(&content[code[0].start..code[0].end], "```code```");
}

#[test]
fn test_parse_code_mismatched_backticks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `code`` here";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 0, "Mismatched backtick counts should not create code span");
}

#[test]
fn test_parse_code_multiple_spans() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `single` and ``double`` code";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 2, "Should find two code spans");
    assert_eq!(&content[code[0].start..code[0].end], "`single`");
    assert_eq!(&content[code[1].start..code[1].end], "``double``");
}

#[test]
fn test_code_empty_code_span() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text `` `` more text";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Empty code span with space should match");
    assert_eq!(&content[code[0].start..code[0].end], "`` ``");
}

#[test]
fn test_code_only_spaces() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text ` ` more";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Code span with only spaces should match");
    assert_eq!(&content[code[0].start..code[0].end], "` `");
}

#[test]
fn test_code_multiple_backticks_inside() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use ``` `` ``` to escape";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Triple backticks should contain double backticks");
    assert_eq!(&content[code[0].start..code[0].end], "``` `` ```");
}

#[test]
fn test_code_unclosed_backticks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text `unclosed";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 0, "Unclosed code span should not match");
}

#[test]
fn test_code_with_newlines_between_backticks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text `code\nwith newline` more";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    println!("Code with newlines found {} spans", code.len());
}

#[test]
fn test_code_four_vs_five_backticks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```` code ````";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1, "Four backticks should match four backticks");
    assert_eq!(&content[code[0].start..code[0].end], "```` code ````");
}

#[test]
fn test_multiple_code_spans_in_sequence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`first` `second` `third`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 3, "Should find three separate code spans");
}

#[test]
fn test_fenced_code_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```rust\nfn main() {}\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeBlock).collect();
    assert!(!code_blocks.is_empty(), "Should find code block");
}

#[test]
fn test_combined_code_inside_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*italic with `code` inside*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(italic.len(), 1, "Should find italic");
    assert_eq!(code.len(), 1, "Should find code inside italic");
}

#[test]
fn test_combined_emphasis_inside_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`**not bold**`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert_eq!(code.len(), 1, "Should find code span");
    assert_eq!(bold.len(), 0, "Bold markers inside code should not be parsed");
}
