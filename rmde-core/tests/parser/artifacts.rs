//! Tests for Claude artifact parsing (Phase 6 - Optional)
//!
//! Tests parsing of Claude-specific XML-style tags:
//! - <antThinking>...</antThinking>
//! - <antMeta>...</antMeta>

use rmde_core::{MarkdownParser, SpanKind};

fn has_span(spans: &[rmde_core::Span], kind: SpanKind) -> bool {
    spans.iter().any(|s| s.kind == kind)
}

fn get_span_text<'a>(content: &'a str, spans: &[rmde_core::Span], kind: SpanKind) -> Option<&'a str> {
    spans
        .iter()
        .find(|s| s.kind == kind)
        .map(|s| &content[s.start..s.end])
}

#[test]
fn test_artifact_thinking_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antThinking>This is my internal reasoning</antThinking>";
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect antThinking");
    let text = get_span_text(content, &spans, SpanKind::ArtifactThinking).unwrap();
    assert_eq!(text, content);
}

#[test]
fn test_artifact_meta_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antMeta>metadata here</antMeta>";
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactMeta), "Should detect antMeta");
    let text = get_span_text(content, &spans, SpanKind::ArtifactMeta).unwrap();
    assert_eq!(text, content);
}

#[test]
fn test_artifact_thinking_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"<antThinking>
Let me think about this step by step:
1. First consideration
2. Second consideration
3. Conclusion
</antThinking>"#;
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect multiline antThinking");
}

#[test]
fn test_artifact_in_context() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Analysis

Here's my response.

<antThinking>
I should consider these points carefully.
</antThinking>

The answer is 42."#;
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect antThinking in context");
    assert!(has_span(&spans, SpanKind::Heading1), "Should also parse other markdown");
}

#[test]
fn test_artifact_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"```xml
<antThinking>This is code, not an artifact</antThinking>
```"#;
    let spans = parser.parse(content);

    // Should not detect artifact inside code block
    assert!(!has_span(&spans, SpanKind::ArtifactThinking), "Should not detect antThinking in code block");
    assert!(has_span(&spans, SpanKind::CodeBlock), "Should detect code block");
}

#[test]
fn test_artifact_not_in_inline_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `<antThinking>` tags in XML";
    let spans = parser.parse(content);

    // Should not detect artifact inside inline code
    assert!(!has_span(&spans, SpanKind::ArtifactThinking), "Should not detect antThinking in inline code");
    assert!(has_span(&spans, SpanKind::CodeInline), "Should detect inline code");
}

#[test]
fn test_multiple_artifacts() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"<antThinking>First thought</antThinking>

Some text here.

<antThinking>Second thought</antThinking>"#;
    let spans = parser.parse(content);

    let thinking_spans: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::ArtifactThinking).collect();
    assert_eq!(thinking_spans.len(), 2, "Should detect multiple antThinking blocks");
}

#[test]
fn test_both_artifact_types() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"<antMeta>metadata</antMeta>

<antThinking>reasoning</antThinking>"#;
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactMeta), "Should detect antMeta");
    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect antThinking");
}

#[test]
fn test_incomplete_artifact() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antThinking>Incomplete thinking block";
    let spans = parser.parse(content);

    // Should not detect incomplete artifact
    assert!(!has_span(&spans, SpanKind::ArtifactThinking), "Should not detect incomplete artifact");
}

#[test]
fn test_nested_tags_in_artifact() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antThinking>This has <em>nested</em> HTML</antThinking>";
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect artifact with nested tags");
}

#[test]
fn test_artifact_with_special_chars() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"<antThinking>Special chars: & < > " '</antThinking>"#;
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should handle special characters");
}

#[test]
fn test_artifact_empty() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<antThinking></antThinking>";
    let spans = parser.parse(content);

    assert!(has_span(&spans, SpanKind::ArtifactThinking), "Should detect empty artifact");
}

#[test]
fn test_artifact_case_sensitive() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "<ANTTHINKING>uppercase</ANTTHINKING>";
    let spans = parser.parse(content);

    // Should NOT detect uppercase version (case sensitive)
    assert!(!has_span(&spans, SpanKind::ArtifactThinking), "Should be case sensitive");
}

#[test]
fn test_artifact_with_attributes() {
    let mut parser = MarkdownParser::new().unwrap();
    // Our simple parser doesn't support attributes, should not match
    let content = r#"<antThinking type="analysis">text</antThinking>"#;
    let spans = parser.parse(content);

    // Won't match because we look for exact <antThinking> without attributes
    assert!(!has_span(&spans, SpanKind::ArtifactThinking), "Should not match tags with attributes");
}
