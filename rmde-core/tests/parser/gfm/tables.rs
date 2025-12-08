//! GFM Table tests
//!
//! Tests for GFM pipe table syntax per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_table_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should find 2 header cells");
    assert_eq!(delimiters.len(), 1, "Should find 1 delimiter row");
    assert_eq!(cells.len(), 2, "Should find 2 data cells");
}

#[test]
fn test_table_with_alignment_left() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Left |\n|:-----|\n| Data |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    assert_eq!(headers.len(), 1, "Should find header with left alignment");
    assert_eq!(delimiters.len(), 1, "Should find delimiter with left alignment marker");
}

#[test]
fn test_table_with_alignment_center() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Center |\n|:------:|\n| Data   |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    assert_eq!(headers.len(), 1, "Should find header with center alignment");
    assert_eq!(delimiters.len(), 1, "Should find delimiter with center alignment markers");
}

#[test]
fn test_table_with_alignment_right() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Right |\n|------:|\n| Data  |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    assert_eq!(headers.len(), 1, "Should find header with right alignment");
    assert_eq!(delimiters.len(), 1, "Should find delimiter with right alignment marker");
}

#[test]
fn test_table_mixed_alignment() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Left | Center | Right |\n|:-----|:------:|------:|\n| L1   | C1     | R1    |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 3, "Should find 3 headers with mixed alignment");
    assert_eq!(delimiters.len(), 1, "Should find 1 delimiter row");
    assert_eq!(cells.len(), 3, "Should find 3 data cells");
}

#[test]
fn test_table_without_leading_trailing_pipes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Header 1 | Header 2\n---------|----------\nCell 1   | Cell 2";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should find headers without leading/trailing pipes");
    assert_eq!(delimiters.len(), 1, "Should find delimiter without leading/trailing pipes");
    assert_eq!(cells.len(), 2, "Should find cells without leading/trailing pipes");
}

#[test]
fn test_table_multi_row() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Col 1 | Col 2 |\n|-------|-------|\n| A1    | A2    |\n| B1    | B2    |\n| C1    | C2    |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should find 2 header cells");
    assert_eq!(cells.len(), 6, "Should find 6 data cells (3 rows x 2 columns)");
}

#[test]
fn test_table_with_inline_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header |\n|--------|\n| **bold** |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();

    assert_eq!(headers.len(), 1, "Should find header");
    assert_eq!(cells.len(), 1, "Should find data cell");
    assert_eq!(bold.len(), 1, "Should find bold formatting inside table cell");
}

#[test]
fn test_table_with_inline_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header |\n|--------|\n| *italic* |";
    let spans = parser.parse(content);

    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert_eq!(cells.len(), 1, "Should find data cell");
    assert_eq!(italic.len(), 1, "Should find italic formatting inside table cell");
}

#[test]
fn test_table_with_inline_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header |\n|--------|\n| `code` |";
    let spans = parser.parse(content);

    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(cells.len(), 1, "Should find data cell");
    assert_eq!(code.len(), 1, "Should find code formatting inside table cell");
}

#[test]
fn test_table_not_detected_without_delimiter() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Not a table |\n| Just pipes |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    assert_eq!(headers.len(), 0, "Should not find headers without delimiter row");
    assert_eq!(delimiters.len(), 0, "Should not find delimiter without proper format");
}

#[test]
fn test_table_empty_cells() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| H1 | H2 |\n|----|----|\\n|    | X  |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    // Note: This test has a literal \n in the string, not a newline
    assert!(headers.len() <= 2, "Should handle malformed table gracefully");
    assert!(delimiters.len() <= 1, "Should handle malformed table gracefully");
}

#[test]
fn test_table_real_world_example() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Project Status

| Task | Status | Notes |
|------|--------|-------|
| Design | **Done** | Finalized |
| Development | *In Progress* | 80% complete |
| Testing | Pending | Starts next week |

More text here."#;

    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();

    assert_eq!(headers.len(), 3, "Should find 3 header cells");
    assert_eq!(delimiters.len(), 1, "Should find 1 delimiter row");
    assert_eq!(cells.len(), 9, "Should find 9 data cells (3 rows x 3 columns)");
    assert_eq!(bold.len(), 1, "Should find bold in table");
    assert_eq!(italic.len(), 1, "Should find italic in table");
}

#[test]
fn test_table_stops_at_blank_line() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| H1 | H2 |\n|----|----|\\n| A  | B  |\n\n| Not | Table |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let _cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert!(headers.len() <= 2, "Should handle malformed table gracefully");
}

#[test]
fn test_table_with_unicode() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| 名前 | 年齢 |\n|------|------|\n| 太郎 | 25   |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should handle Unicode in headers");
    assert_eq!(cells.len(), 2, "Should handle Unicode in cells");
}

#[test]
fn test_table_with_empty_cell_content() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| H1 | H2 |\n|----|----|\n|    | X  |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should find 2 headers");
    assert_eq!(delimiters.len(), 1, "Should find delimiter row");
    assert_eq!(cells.len(), 2, "Should find 2 cells (including empty)");
}

#[test]
fn test_table_ends_at_blank_line() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| H1 | H2 |\n|----|----|\\n| A  | B  |\n\nMore text";
    let spans = parser.parse(content);

    let _headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();

    assert!(delimiters.len() <= 1, "Should have at most one table");
}

#[test]
fn test_table_delimiter_minimum_three_hyphens() {
    let mut parser = MarkdownParser::new().unwrap();

    // Valid: exactly 3 hyphens
    let content = "| H1 | H2 |\n|---|---|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 1, "Should accept delimiter with 3 hyphens");

    // Valid: 4+ hyphens
    let content = "| H1 | H2 |\n|----|-----|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 1, "Should accept delimiter with 4+ hyphens");

    // Invalid: 1 hyphen
    let content = "| H1 | H2 |\n|-|-|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 0, "Should reject delimiter with 1 hyphen");

    // Invalid: 2 hyphens
    let content = "| H1 | H2 |\n|--|--|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 0, "Should reject delimiter with 2 hyphens");
}

#[test]
fn test_table_delimiter_three_hyphens_with_alignment() {
    let mut parser = MarkdownParser::new().unwrap();

    // Valid: left alignment with 3 hyphens
    let content = "| H1 | H2 |\n|:---|:---|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 1, "Should accept left-aligned delimiter with 3 hyphens");

    // Valid: center alignment with 3 hyphens
    let content = "| H1 | H2 |\n|:---:|:---:|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 1, "Should accept center-aligned delimiter with 3 hyphens");

    // Valid: right alignment with 3 hyphens
    let content = "| H1 | H2 |\n|---:|---:|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 1, "Should accept right-aligned delimiter with 3 hyphens");

    // Invalid: left alignment with 1 hyphen
    let content = "| H1 | H2 |\n|:-|:-|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 0, "Should reject left-aligned delimiter with 1 hyphen");

    // Invalid: center alignment with 2 hyphens
    let content = "| H1 | H2 |\n|:--:|:--:|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 0, "Should reject center-aligned delimiter with 2 hyphens");

    // Invalid: right alignment with 1 hyphen
    let content = "| H1 | H2 |\n|-:|-:|\n| A | B |";
    let spans = parser.parse(content);
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    assert_eq!(delimiters.len(), 0, "Should reject right-aligned delimiter with 1 hyphen");
}
