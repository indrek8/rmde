//! Task list tests
//!
//! Tests for GFM task list syntax (- [ ] / - [x]) per MARKDOWN-SYNTAX.md

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_task_marker_basic_unchecked() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] task item";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 1, "Should find one unchecked task marker");
    assert_eq!(tasks[0].start, 2);
    assert_eq!(tasks[0].end, 5);
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[ ]");
}

#[test]
fn test_task_marker_basic_checked_lowercase() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [x] completed task";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();
    assert_eq!(tasks.len(), 1, "Should find one checked task marker");
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[x]");
}

#[test]
fn test_task_marker_basic_checked_uppercase() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [X] completed task";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();
    assert_eq!(tasks.len(), 1, "Should find one checked task marker");
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[X]");
}

#[test]
fn test_task_marker_asterisk_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "* [ ] task with asterisk";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 1, "Should find task marker with asterisk");
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[ ]");
}

#[test]
fn test_task_marker_plus_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "+ [ ] task with plus";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 1, "Should find task marker with plus");
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[ ]");
}

#[test]
fn test_task_marker_numbered_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1. [ ] numbered task";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 1, "Should find task marker in numbered list");
    assert_eq!(&content[tasks[0].start..tasks[0].end], "[ ]");
}

#[test]
fn test_task_marker_multiple_tasks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] task one\n- [x] task two\n- [ ] task three";
    let spans = parser.parse(content);

    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();

    assert_eq!(unchecked.len(), 2, "Should find two unchecked tasks");
    assert_eq!(checked.len(), 1, "Should find one checked task");
}

#[test]
fn test_task_marker_nested_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] parent task\n  - [ ] child task\n    - [x] grandchild task";
    let spans = parser.parse(content);

    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();

    assert_eq!(unchecked.len(), 2, "Should find two unchecked tasks in nested list");
    assert_eq!(checked.len(), 1, "Should find one checked task in nested list");
}

#[test]
fn test_task_marker_not_after_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Just some text [ ] with brackets";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(tasks.len(), 0, "[ ] without list marker should not match");
}

#[test]
fn test_task_marker_no_space_after_dash() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "-[ ] invalid";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(tasks.len(), 0, "Task marker without space after dash should not match");
}

#[test]
fn test_task_marker_invalid_checkbox_character() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [y] invalid checkbox";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    assert_eq!(tasks.len(), 0, "Invalid checkbox character should not match");
}

#[test]
fn test_task_marker_with_formatting() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] **bold** task\n- [x] *italic* task\n- [ ] `code` task";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::TaskMarker || s.kind == SpanKind::TaskChecked
    ).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();

    assert_eq!(tasks.len(), 3, "Should find three task markers");
    assert_eq!(bold.len(), 1, "Should find bold in task");
    assert_eq!(italic.len(), 1, "Should find italic in task");
    assert_eq!(code.len(), 1, "Should find code in task");
}

#[test]
fn test_task_marker_real_world_todo_list() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r#"# Project Tasks

## In Progress
- [ ] Implement feature X
- [ ] Write tests
  - [x] Unit tests
  - [ ] Integration tests
- [x] Update documentation

## Completed
- [x] Setup project
- [x] Initial commit
"#;

    let spans = parser.parse(content);

    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();

    assert_eq!(unchecked.len(), 3, "Should find three unchecked tasks");
    assert_eq!(checked.len(), 4, "Should find four checked tasks");
}

#[test]
fn test_task_marker_mixed_list_types() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] dash task\n* [ ] star task\n+ [ ] plus task\n1. [ ] numbered task";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 4, "Should find tasks with all list marker types");
}

#[test]
fn test_task_marker_with_strikethrough() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [x] ~~deprecated task~~\n- [ ] new task";
    let spans = parser.parse(content);

    let checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();
    let unchecked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();

    assert_eq!(checked.len(), 1, "Should find checked task");
    assert_eq!(unchecked.len(), 1, "Should find unchecked task");
    assert_eq!(strike.len(), 1, "Should find strikethrough");
}

#[test]
fn test_task_marker_indented_with_tabs() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "\t- [ ] task with tab indent";
    let spans = parser.parse(content);

    let tasks: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    assert_eq!(tasks.len(), 1, "Should find task with tab indentation");
}
