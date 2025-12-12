//! Category 10: Tables (GFM)
//!
//! Tests for GitHub Flavored Markdown table syntax
//! Status: ISSUES DETECTED (formatting in cells)
//!
//! Test cases from example_test.md lines 252-290

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_simple_table() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n| Cell 4   | Cell 5   | Cell 6   |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table");
}

#[test]
fn test_aligned_table() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Left | Center | Right |\n|:-----|:------:|------:|\n| L1   | C1     | R1    |\n| L2   | C2     | R2    |\n| L3   | C3     | R3    |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table with alignment markers");
}

#[test]
fn test_table_without_leading_trailing_pipes() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Header 1 | Header 2\n---------|----------\nCell 1   | Cell 2";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table without leading/trailing pipes");
}

#[test]
fn test_table_with_bold() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Feature | Syntax |\n|---------|--------|\n| Bold | **bold** |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table");

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    println!("Found {} bold spans in table cell", bold.len());
}

#[test]
fn test_table_with_italic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Feature | Example |\n|---------|---------|  \n| Italic | *italic* |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table");

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    println!("Found {} italic spans in table cell", italic.len());
}

#[test]
fn test_table_with_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Feature | Syntax |\n|---------|--------|\n| Code | `code` |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table");

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    println!("Found {} code spans in table cell", code.len());
}

#[test]
fn test_table_with_link() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Feature | Example |\n|---------|---------|  \n| Link | [link](url) |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table");

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    println!("Found {} link spans in table cell", links.len());
}

#[test]
fn test_table_with_empty_cells() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| A | B | C |\n|---|---|---|\n| 1 |   | 3 |\n|   | 2 |   |";
    let spans = parser.parse(content);

    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert!(!tables.is_empty(), "Should find table with empty cells");
}

#[test]
fn test_table_requires_header_row() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "|---|---|---|\n| 1 | 2 | 3 |";
    let spans = parser.parse(content);

    // A table must have a header row before the delimiter row
    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    assert_eq!(tables.len(), 0, "Delimiter row without header should not create table");
}

#[test]
fn test_table_column_mismatch() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| A | B |\n|---|---|\n| 1 | 2 | 3 |";
    let spans = parser.parse(content);

    // Tables can have mismatched columns (extra cells are typically ignored)
    let tables: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TableHeader || s.kind == SpanKind::TableDelimiter || s.kind == SpanKind::TableCell
    ).collect();
    println!("Table with mismatched columns found {} table spans", tables.len());
}
