//! Category 7: Task Lists (GFM)
//!
//! Tests for - [ ] and - [x] checkbox syntax
//! Status: PARTIAL ISSUES
//!
//! Test cases from example_test.md lines 148-161

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_unchecked_task() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Unchecked task";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 1, "Should find unchecked task marker");
}

#[test]
fn test_checked_task_lowercase_x() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [x] Checked task";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 1, "Should find checked task marker (lowercase x)");
}

#[test]
fn test_checked_task_uppercase_x() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [X] Also checked (capital X)";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 1, "Should find checked task marker (uppercase X)");
}

#[test]
fn test_task_with_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Task with **bold text**";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 1, "Should find task marker");

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert_eq!(bold.len(), 1, "Should find bold text inside task");
}

#[test]
fn test_nested_task_lists() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [x] Parent task complete\n  - [ ] Subtask 1 incomplete\n  - [x] Subtask 2 complete\n- [ ] Another parent task";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 4, "Should find four task markers in nested list");
}

#[test]
fn test_task_list_must_have_space_after_bracket() {
    let mut parser = MarkdownParser::new().unwrap();

    // Valid: space after bracket
    let valid = "- [ ] Valid task";
    let spans_valid = parser.parse(valid);
    let markers_valid: Vec<_> = spans_valid.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(markers_valid.len(), 1, "Task with space after bracket should be valid");

    // Invalid: no space after bracket
    parser.reset();
    let invalid = "- []Not a task";
    let spans_invalid = parser.parse(invalid);
    let markers_invalid: Vec<_> = spans_invalid.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(markers_invalid.len(), 0, "No space after bracket should not be a task");
}

#[test]
fn test_task_list_mixed_with_regular_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Regular list item\n- [ ] Task item\n- Another regular item\n- [x] Another task";
    let spans = parser.parse(content);

    let list_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::ListMarker
    ).collect();
    assert_eq!(list_markers.len(), 4, "Should find all four list markers");

    let task_markers: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(task_markers.len(), 2, "Should find two task list markers");
}
