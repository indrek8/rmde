//! Integration tests
//!
//! Tests combining multiple GFM features and real-world scenarios

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_integration_strikethrough_in_tables() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Header |\n|--------|\n| ~~deleted~~ |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(headers.len(), 1, "Should find table header");
    assert_eq!(delimiters.len(), 1, "Should find table delimiter");
    assert_eq!(cells.len(), 1, "Should find table cell");
    assert_eq!(strike.len(), 1, "Should find strikethrough inside table cell");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~deleted~~");
}

#[test]
fn test_integration_autolinks_in_tables() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "| Link |\n|------|\n| https://example.com |";
    let spans = parser.parse(content);

    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();
    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();

    assert_eq!(headers.len(), 1, "Should find table header");
    assert_eq!(delimiters.len(), 1, "Should find table delimiter");
    assert_eq!(cells.len(), 1, "Should find table cell");
    assert_eq!(autolinks.len(), 1, "Should find autolink inside table cell");
    assert_eq!(&content[autolinks[0].start..autolinks[0].end], "https://example.com");
}

#[test]
fn test_integration_strikethrough_with_autolinks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit ~~https://old.com~~ and use https://new.com instead";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();

    assert_eq!(strike.len(), 1, "Should find strikethrough");
    assert_eq!(autolinks.len(), 2, "Should find both autolinks (inside and outside strikethrough)");

    // Verify that one autolink is inside strikethrough and one is outside
    let strike_span = strike[0];
    let autolink1 = autolinks[0];
    let autolink2 = autolinks[1];

    // One should be inside strike, one outside
    let inside_strike = (autolink1.start >= strike_span.start && autolink1.end <= strike_span.end) ||
                       (autolink2.start >= strike_span.start && autolink2.end <= strike_span.end);
    let outside_strike = (autolink1.start >= strike_span.end) || (autolink2.start >= strike_span.end);

    assert!(inside_strike, "One autolink should be inside strikethrough");
    assert!(outside_strike, "One autolink should be outside strikethrough");
}

#[test]
fn test_integration_task_lists_with_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Not done\n- [x] ~~Completed and archived~~";
    let spans = parser.parse(content);

    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(unchecked.len(), 1, "Should find one unchecked task");
    assert_eq!(checked.len(), 1, "Should find one checked task");
    assert_eq!(strike.len(), 1, "Should find strikethrough in completed task");
    assert_eq!(&content[strike[0].start..strike[0].end], "~~Completed and archived~~");
}

#[test]
fn test_integration_all_phase2_features_combined() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"- [ ] Check ~~https://old.com~~ and use https://new.com

| Feature | Status |
|---------|--------|
| ~~Old~~ | Deprecated |
| New | https://docs.com |"#;

    let spans = parser.parse(content);

    // Task list markers
    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(unchecked.len(), 1, "Should find one unchecked task");

    // Strikethrough
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    assert_eq!(strike.len(), 2, "Should find two strikethrough spans (in task and in table)");

    // Autolinks
    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    assert_eq!(autolinks.len(), 3, "Should find three autolinks (old, new, docs)");

    // Table elements
    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();
    let delimiters: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableDelimiter).collect();
    let cells: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableCell).collect();

    assert_eq!(headers.len(), 2, "Should find two table headers");
    assert_eq!(delimiters.len(), 1, "Should find one table delimiter");
    assert_eq!(cells.len(), 4, "Should find four table cells (2 rows x 2 columns)");

    // Verify that features don't interfere with each other
    // All spans should be valid and not overlap incorrectly
    for i in 0..spans.len() {
        for j in (i + 1)..spans.len() {
            let s1 = &spans[i];
            let s2 = &spans[j];

            // Skip if they're the same type (nesting allowed) or if one contains the other (nesting)
            if s1.kind == s2.kind {
                continue;
            }

            // Check for proper nesting - either completely separate or one contains the other
            let separate = s1.end <= s2.start || s2.end <= s1.start;
            let s1_contains_s2 = s1.start <= s2.start && s1.end >= s2.end;
            let s2_contains_s1 = s2.start <= s1.start && s2.end >= s1.end;

            assert!(
                separate || s1_contains_s2 || s2_contains_s1,
                "Spans should either be separate or properly nested: {:?} vs {:?}",
                s1, s2
            );
        }
    }
}

// Real-world document tests

#[test]
fn test_real_world_readme() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Project Name

A **bold** statement about this *project*.

## Features

- [x] ~~Old feature~~ - Deprecated
- [x] Core functionality
- [ ] Future enhancement

## Links

Visit https://example.com or <https://docs.example.com> for more info.

| Feature | Status |
|---------|--------|
| Core | **Done** |
| Docs | *In Progress* |
"#;

    let spans = parser.parse(content);

    // Verify all major elements are found
    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let tasks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();
    let headers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TableHeader).collect();

    assert_eq!(h1.len(), 1, "Should find 1 H1");
    assert_eq!(h2.len(), 2, "Should find 2 H2");
    assert!(!bold.is_empty(), "Should find bold");
    assert!(!italic.is_empty(), "Should find italic");
    assert_eq!(tasks.len(), 3, "Should find 3 task markers");
    assert_eq!(strike.len(), 1, "Should find 1 strikethrough");
    assert_eq!(autolinks.len(), 2, "Should find 2 autolinks");
    assert_eq!(headers.len(), 2, "Should find 2 table headers");
}

#[test]
fn test_real_world_changelog() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Changelog

## [1.0.0] - 2024-01-01

### Added
- **Feature A**: New functionality
- **Feature B**: Another feature

### Changed
- ~~Old behavior~~ replaced with new

### Fixed
- Bug in `parse()` function

See https://github.com/example/repo for details.
"#;

    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let h3: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let autolinks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Autolink).collect();

    assert_eq!(h1.len(), 1, "Should find 1 H1");
    assert_eq!(h2.len(), 1, "Should find 1 H2");
    assert_eq!(h3.len(), 3, "Should find 3 H3");
    assert!(!bold.is_empty(), "Should find bold");
    assert!(!code.is_empty(), "Should find code");
    assert!(!strike.is_empty(), "Should find strikethrough");
    assert!(!autolinks.is_empty(), "Should find autolinks");
}

#[test]
fn test_combined_multiple_elements_one_line() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading with **bold** and `code`";
    let spans = parser.parse(content);

    let heading: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert!(!heading.is_empty(), "Should find heading");
    assert_eq!(bold.len(), 1, "Should find bold inside heading");
    assert_eq!(code.len(), 1, "Should find code inside heading");
}

#[test]
fn test_real_world_markdown_snippet() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Overview

This is **important** text with `code` and *emphasis*.

## Details

- Item with `inline code`
- Item with **bold text**
"#;

    let spans = parser.parse(content);

    let h1: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let h2: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(h1.len(), 1, "Should find H1");
    assert_eq!(h2.len(), 1, "Should find H2");
    assert_eq!(bold.len(), 2, "Should find two bold spans");
    assert_eq!(italic.len(), 1, "Should find italic");
    assert_eq!(code.len(), 2, "Should find two code spans");
}
