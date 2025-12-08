//! Document tests

use rmde_core::Document;

#[test]
fn test_new_document() {
    let doc = Document::new();
    assert!(doc.is_empty());
    assert!(!doc.is_dirty());
    assert_eq!(doc.title(), "Untitled");
}

#[test]
fn test_insert() {
    let mut doc = Document::new();
    doc.insert("Hello");
    assert_eq!(doc.content(), "Hello");
    assert!(doc.is_dirty());

    doc.insert(" World");
    assert_eq!(doc.content(), "Hello World");
}

#[test]
fn test_delete_backward() {
    let mut doc = Document::new();
    doc.insert("Hello");
    doc.delete_backward();
    assert_eq!(doc.content(), "Hell");
}

#[test]
fn test_multi_cursor_insert() {
    let mut doc = Document::new();
    doc.insert("ab");
    doc.set_cursor(1); // between a and b
    doc.add_cursor(2); // after b

    doc.insert("X");
    assert_eq!(doc.content(), "aXbX");
}

#[test]
fn test_select_all() {
    let mut doc = Document::new();
    doc.insert("Hello World");
    doc.select_all();
    assert_eq!(doc.selected_text(), Some("Hello World".to_string()));
}
