//! Editor tests

use rmde_core::{Editor, DocumentId};

#[test]
fn test_new_editor() {
    let editor = Editor::new();
    assert_eq!(editor.tab_count(), 1);
    assert!(editor.active().is_some());
}

#[test]
fn test_new_tab() {
    let mut editor = Editor::new();
    let id = editor.new_tab();
    assert_eq!(editor.tab_count(), 2);
    assert_eq!(editor.active_id(), Some(id));
}

#[test]
fn test_close_tab() {
    let mut editor = Editor::new();
    let id1 = editor.active_id().unwrap();
    let id2 = editor.new_tab();

    assert_eq!(editor.tab_count(), 2);
    editor.close_tab(id2);
    assert_eq!(editor.tab_count(), 1);
    assert_eq!(editor.active_id(), Some(id1));
}

#[test]
fn test_close_last_tab() {
    let mut editor = Editor::new();
    let id = editor.active_id().unwrap();

    editor.close_tab(id);
    // Should have a new empty document
    assert_eq!(editor.tab_count(), 1);
    assert!(editor.active().is_some());
}

#[test]
fn test_switch_tab() {
    let mut editor = Editor::new();
    let id1 = editor.active_id().unwrap();
    let id2 = editor.new_tab();

    assert_eq!(editor.active_id(), Some(id2));
    editor.switch_tab(id1);
    assert_eq!(editor.active_id(), Some(id1));
}

#[test]
fn test_next_prev_tab() {
    let mut editor = Editor::new();
    let id1 = editor.active_id().unwrap();
    let id2 = editor.new_tab();
    let _id3 = editor.new_tab();

    editor.switch_tab(id1);
    editor.next_tab();
    assert_eq!(editor.active_id(), Some(id2));

    editor.prev_tab();
    assert_eq!(editor.active_id(), Some(id1));
}

#[test]
fn test_document_id_unique() {
    let id1 = DocumentId::new();
    let id2 = DocumentId::new();
    assert_ne!(id1, id2);
}
