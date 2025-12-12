//! Category 29: Complex Nesting
//!
//! Tests for deeply nested and complex markdown structures
//! Status: Testing required
//!
//! Test cases for combinations of lists, quotes, code, and tables

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_list_with_blockquote() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- List item\n  > Quote inside list";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();

    assert!(list_markers.len() >= 1, "Should find list marker");
    assert!(quotes.len() >= 1, "Should find blockquote inside list");
}

#[test]
fn test_list_with_code_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- List item\n\n      code block in list";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    let code: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::CodeBlock | SpanKind::CodeInline)
    ).collect();

    assert!(list_markers.len() >= 1, "Should find list marker");
    assert!(code.len() >= 1, "Should find code block inside list");
}

#[test]
fn test_blockquote_with_nested_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Quote\n> - List in quote\n> - Another item";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(quotes.len() >= 1, "Should find blockquote");
    assert!(list_markers.len() >= 2, "Should find list items inside quote");
}

#[test]
fn test_blockquote_with_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Quote with `code` inside";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    let code: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeInline
    ).collect();

    assert!(quotes.len() >= 1, "Should find blockquote");
    assert!(code.len() >= 1, "Should find code span inside quote");
}

#[test]
fn test_nested_blockquotes_three_levels() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Level 1\n>> Level 2\n>>> Level 3";
    let spans = parser.parse(content);

    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();

    assert!(quotes.len() >= 3, "Should find three levels of nested quotes");
}

#[test]
fn test_list_with_multiple_paragraphs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- First paragraph\n\n  Second paragraph in same item";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(list_markers.len() >= 1, "Should find list with multiple paragraphs");
}

#[test]
fn test_nested_lists_three_levels() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Level 1\n  - Level 2\n    - Level 3";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(list_markers.len() >= 3, "Should find three levels of nested lists");
}

#[test]
fn test_table_with_formatting_in_cells() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| **Bold** | *Italic* | `code` |\n|---|---|---|\n| a | b | c |";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic)
    ).collect();
    let italic: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Italic | SpanKind::BoldItalic)
    ).collect();
    let code: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeInline
    ).collect();

    assert!(bold.len() >= 1, "Should find bold in table");
    assert!(italic.len() >= 1, "Should find italic in table");
    assert!(code.len() >= 1, "Should find code in table");
}

#[test]
fn test_table_with_links() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| [Link](url) | Text |\n|---|---|\n| a | b |";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();

    assert!(links.len() >= 1, "Should find links in table");
}

#[test]
fn test_emphasis_with_nested_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*italic with **bold** inside*";
    let spans = parser.parse(content);

    let formatted: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind,
            SpanKind::Italic | SpanKind::Bold | SpanKind::BoldItalic | SpanKind::Emphasis
        )
    ).collect();

    assert!(formatted.len() >= 2, "Should find nested emphasis and bold");
}

#[test]
fn test_link_with_formatted_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[**bold link**](url)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::Link
    ).collect();
    let bold: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::BoldItalic)
    ).collect();

    assert!(links.len() >= 1, "Should find link");
    assert!(bold.len() >= 1, "Should find bold inside link text");
}

#[test]
fn test_list_with_task_items_and_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Task with **bold**\n- [x] Completed with *italic*";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::TaskMarker | SpanKind::TaskChecked)
    ).collect();
    let formatted: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::Bold | SpanKind::Italic)
    ).collect();

    assert!(task_markers.len() >= 2, "Should find task markers");
    assert!(formatted.len() >= 2, "Should find formatting in task items");
}

#[test]
fn test_deeply_nested_combination() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "> Quote\n> - List item\n>   > Nested quote\n>   - Nested list";
    let spans = parser.parse(content);

    // Complex nesting: quote -> list -> nested quote -> nested list
    let quotes: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::BlockQuote
    ).collect();
    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();

    assert!(quotes.len() >= 1, "Should find blockquotes in complex nesting");
    assert!(list_markers.len() >= 1, "Should find list markers in complex nesting");
}
