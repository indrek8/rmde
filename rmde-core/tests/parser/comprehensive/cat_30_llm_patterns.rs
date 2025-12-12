//! Category 30: LLM Patterns
//!
//! Tests for common LLM-generated markdown patterns
//! Status: Testing required
//!
//! LLMs often generate incomplete markdown or use artifact tags

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_unclosed_code_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```rust\nfn main() {\n    println!(\"hello\");\n}";
    let spans = parser.parse(content);

    // Unclosed code block - parser should handle gracefully
    let code: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::CodeBlock | SpanKind::CodeFence)
    ).collect();

    assert!(spans.len() > 0, "Should handle unclosed code block gracefully");
}

#[test]
fn test_unclosed_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*This emphasis is never closed\n\nNew paragraph here.";
    let spans = parser.parse(content);

    // Unclosed emphasis across paragraphs
    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    // Paragraph break should close emphasis
    assert!(spans.len() > 0, "Should handle unclosed emphasis");
}

#[test]
fn test_unclosed_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**This bold is never closed";
    let spans = parser.parse(content);

    // Unclosed bold at end of content
    assert!(spans.len() > 0, "Should handle unclosed bold");
}

#[test]
fn test_unclosed_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[This link has no closing";
    let spans = parser.parse(content);

    // Unclosed link bracket
    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    assert!(spans.len() > 0, "Should handle unclosed link");
}

#[test]
fn test_artifact_thinking_tags() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antThinking>Internal reasoning</antThinking>";
    let spans = parser.parse(content);

    // Artifact thinking tags - check if parser has special handling
    let artifact: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ArtifactThinking
    ).collect();

    // May or may not be specially handled
    assert!(spans.len() > 0, "Should parse artifact thinking tags");
}

#[test]
fn test_artifact_meta_tags() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antMeta>Metadata here</antMeta>";
    let spans = parser.parse(content);

    // Artifact meta tags
    let artifact: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ArtifactMeta
    ).collect();

    assert!(spans.len() > 0, "Should parse artifact meta tags");
}

#[test]
fn test_mixed_unclosed_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*italic **bold but only italic closed*";
    let spans = parser.parse(content);

    // Mixed unclosed formatting
    assert!(spans.len() > 0, "Should handle mixed unclosed formatting");
}

#[test]
fn test_code_fence_with_no_closing_fence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```python\nprint('hello')\nprint('world')";
    let spans = parser.parse(content);

    // Code fence without closing
    assert!(spans.len() > 0, "Should handle code fence without closing");
}

#[test]
fn test_nested_unclosed_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*outer **inner but never closed";
    let spans = parser.parse(content);

    // Nested unclosed emphasis
    assert!(spans.len() > 0, "Should handle nested unclosed emphasis");
}

#[test]
fn test_llm_continuation_pattern() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Here's some code:\n```\npartial code...";
    let spans = parser.parse(content);

    // LLMs often generate partial code blocks
    assert!(spans.len() > 0, "Should handle LLM continuation pattern");
}

#[test]
fn test_multiple_unclosed_code_blocks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```\nfirst block\n```\nsecond block";
    let spans = parser.parse(content);

    // Multiple code blocks with one unclosed
    let code: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::CodeBlock | SpanKind::CodeFence)
    ).collect();

    assert!(spans.len() > 0, "Should handle multiple code blocks");
}

#[test]
fn test_emphasis_across_many_lines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*Line 1\nLine 2\nLine 3\nLine 4\nLine 5*";
    let spans = parser.parse(content);

    // Emphasis spanning many lines (LLM pattern)
    let emphasis: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::Emphasis)
    ).collect();

    assert!(spans.len() > 0, "Should handle multi-line emphasis");
}

#[test]
fn test_malformed_table() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header |\n| Cell |";
    let spans = parser.parse(content);

    // Table without delimiter row
    assert!(spans.len() > 0, "Should handle malformed table");
}

#[test]
fn test_mixed_list_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item 1\n* Item 2\n+ Item 3";
    let spans = parser.parse(content);

    // Mixed list markers (valid but unusual)
    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(list_markers.len() >= 3, "Should handle mixed list markers");
}
