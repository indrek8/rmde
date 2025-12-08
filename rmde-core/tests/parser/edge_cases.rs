//! Edge case tests
//!
//! Tests for parser edge cases, unusual input, and boundary conditions

use rmde_core::{MarkdownParser, SpanKind};

// Empty and minimal input tests

#[test]
fn test_empty_input() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("");
    assert!(spans.is_empty(), "Empty input should produce no spans");
}

#[test]
fn test_whitespace_only() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("   \n\n   \t\t   ");
    assert!(spans.is_empty() || spans.iter().all(|s| s.kind != SpanKind::Bold),
            "Whitespace-only should not produce formatting spans");
}

#[test]
fn test_single_character() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("a");
    assert!(spans.is_empty() || spans.iter().all(|s| s.kind != SpanKind::Bold),
            "Single character should not cause issues");
}

// Delimiter edge cases

#[test]
fn test_many_asterisks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "******text******";
    let spans = parser.parse(content);
    // Just verify it doesn't crash and produces reasonable output
    println!("Many asterisks: found {} spans", spans.len());
}

#[test]
fn test_alternating_delimiters() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*_*_*_text_*_*_*";
    let spans = parser.parse(content);
    // Just verify it doesn't crash
    println!("Alternating delimiters: found {} spans", spans.len());
}

#[test]
fn test_unclosed_delimiters() {
    let mut parser = MarkdownParser::new().unwrap();

    let cases = vec![
        "**unclosed bold",
        "*unclosed italic",
        "`unclosed code",
        "~~unclosed strike",
        "# heading without newline",
    ];

    for content in cases {
        let _spans = parser.parse(content);
        parser.reset();
        // Just verify it doesn't crash
    }
}

// Unicode edge cases

#[test]
fn test_unicode_stress() {
    let mut parser = MarkdownParser::new().unwrap();

    // Various Unicode stress tests
    let cases = vec![
        "emoji **🎉🎊🎁** test",
        "chinese **中文** bold",
        "arabic **العربية** text",
        "zero-width **\u{200B}hidden\u{200B}** chars",
        "combining marks **e\u{0301}** acute",
    ];

    for content in cases {
        let spans = parser.parse(content);
        parser.reset();
        // Verify we found some formatting
        let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
        assert!(!bold.is_empty(), "Should find bold in: {}", content);
    }
}

#[test]
fn test_unicode_boundaries() {
    let mut parser = MarkdownParser::new().unwrap();
    // Test that byte slicing works correctly with multi-byte chars
    let content = "你好 **世界**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    if !bold.is_empty() {
        // Verify the slice is valid UTF-8
        let sliced = &content[bold[0].start..bold[0].end];
        assert!(sliced.contains("世界"), "Should correctly slice Chinese text");
    }
}

// Nesting edge cases

#[test]
fn test_deep_nesting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*a **b `c` b** a*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    // Should find all nested elements
    assert!(!italic.is_empty(), "Should find italic");
    assert!(!bold.is_empty(), "Should find bold");
    assert!(!code.is_empty(), "Should find code");
}

#[test]
fn test_overlapping_delimiters() {
    let mut parser = MarkdownParser::new().unwrap();
    // Intentionally malformed - overlapping delimiters
    let content = "**bold *and italic** end*";
    let spans = parser.parse(content);

    // Just verify it doesn't crash and produces something
    println!("Overlapping delimiters: {} spans", spans.len());
}

// Line ending edge cases

#[test]
fn test_crlf_line_endings() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading\r\n\r\nParagraph with **bold**\r\n";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert!(!h1.is_empty(), "Should find heading with CRLF");
    assert!(!bold.is_empty(), "Should find bold with CRLF");
}

#[test]
fn test_mixed_line_endings() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading\r\n**bold**\n*italic*\r\n";
    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert!(!h1.is_empty(), "Should find heading");
    assert!(!bold.is_empty(), "Should find bold");
    assert!(!italic.is_empty(), "Should find italic");
}

// Large input edge cases

#[test]
fn test_long_line() {
    let mut parser = MarkdownParser::new().unwrap();
    let long_word = "a".repeat(10000);
    let content = format!("**{}**", long_word);
    let spans = parser.parse(&content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!bold.is_empty(), "Should handle long lines");
}

#[test]
fn test_many_lines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = (0..1000).map(|i| format!("Line {} **bold**", i)).collect::<Vec<_>>().join("\n");
    let spans = parser.parse(&content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(bold.len(), 1000, "Should find bold in all 1000 lines");
}

// Special character edge cases

#[test]
fn test_backslash_escapes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\*not bold\* but **this is**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    // At minimum, the non-escaped should work
    println!("Backslash escapes: {} bold spans", bold.len());
}

#[test]
fn test_special_characters_in_content() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**<>&\"'`{}[]()#+-!**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!bold.is_empty(), "Should handle special characters inside formatting");
}

// Parser reset edge cases

#[test]
fn test_parser_reuse() {
    let mut parser = MarkdownParser::new().unwrap();

    // Parse multiple times
    for i in 0..100 {
        let content = format!("Iteration {} with **bold**", i);
        let spans = parser.parse(&content);
        let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
        assert_eq!(bold.len(), 1, "Should find bold on iteration {}", i);
        parser.reset();
    }
}

#[test]
fn test_reset_clears_state() {
    let mut parser = MarkdownParser::new().unwrap();

    // Parse something complex
    let _spans1 = parser.parse("# Heading\n\n**Bold** and *italic*\n\n```code```");
    parser.reset();

    // Parse something simple - should be clean
    let spans2 = parser.parse("**bold**");
    let bold: Vec<_> = spans2.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(bold.len(), 1, "Should find exactly one bold after reset");
}
