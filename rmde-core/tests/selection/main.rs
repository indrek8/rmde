//! Selection tests

use rmde_core::Selection;

#[test]
fn test_cursor() {
    let sel = Selection::cursor(5);
    assert!(sel.is_cursor());
    assert_eq!(sel.start(), 5);
    assert_eq!(sel.end(), 5);
    assert_eq!(sel.len(), 0);
}

#[test]
fn test_selection() {
    let sel = Selection::new(5, 10);
    assert!(!sel.is_cursor());
    assert_eq!(sel.start(), 5);
    assert_eq!(sel.end(), 10);
    assert_eq!(sel.len(), 5);
}

#[test]
fn test_reverse_selection() {
    let sel = Selection::new(10, 5);
    assert_eq!(sel.start(), 5);
    assert_eq!(sel.end(), 10);
    assert_eq!(sel.len(), 5);
}

#[test]
fn test_contains() {
    let sel = Selection::new(5, 10);
    assert!(!sel.contains(4));
    assert!(sel.contains(5));
    assert!(sel.contains(7));
    assert!(!sel.contains(10));
}

#[test]
fn test_move_by() {
    let mut sel = Selection::cursor(5);
    sel.move_by(3, 100, false);
    assert_eq!(sel.head, 8);
    assert_eq!(sel.anchor, 8);

    sel.move_by(-2, 100, true);
    assert_eq!(sel.head, 6);
    assert_eq!(sel.anchor, 8);
}
