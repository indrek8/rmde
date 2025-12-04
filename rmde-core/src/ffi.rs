//! FFI bridge for Swift interop via swift-bridge

use crate::{DocumentId, Editor};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RMDEEditor;

        // Constructor
        #[swift_bridge(init)]
        fn new() -> RMDEEditor;

        // Tab management
        fn new_tab(&mut self) -> u64;
        fn close_tab(&mut self, id: u64) -> bool;
        fn switch_tab(&mut self, id: u64) -> bool;
        fn next_tab(&mut self);
        fn prev_tab(&mut self);
        fn tab_count(&self) -> usize;
        fn get_active_tab_id(&self) -> u64;

        // Document content
        fn get_content(&self) -> String;
        fn get_content_length(&self) -> usize;
        fn insert_text(&mut self, text: &str);
        fn delete_backward(&mut self);
        fn delete_forward(&mut self);

        // Cursor/selection
        fn set_cursor(&mut self, pos: usize);
        fn add_cursor(&mut self, pos: usize);
        fn move_cursors(&mut self, delta: i64, extend: bool);
        fn select_all(&mut self);
        fn get_cursor_position(&self) -> usize;

        // File operations - returns empty string on success, error message on failure
        fn open_file(&mut self, path: &str) -> String;
        fn save_file(&mut self) -> String;
        fn save_file_as(&mut self, path: &str) -> String;

        // Document info
        fn is_dirty(&self) -> bool;
        fn get_title(&self) -> String;
    }
}

/// Wrapper around Editor for FFI
pub struct RMDEEditor {
    inner: Editor,
}

impl RMDEEditor {
    fn new() -> Self {
        Self {
            inner: Editor::new(),
        }
    }

    fn new_tab(&mut self) -> u64 {
        self.inner.new_tab().as_u64()
    }

    fn close_tab(&mut self, id: u64) -> bool {
        self.inner.close_tab(DocumentId::from_u64(id))
    }

    fn switch_tab(&mut self, id: u64) -> bool {
        self.inner.switch_tab(DocumentId::from_u64(id))
    }

    fn next_tab(&mut self) {
        self.inner.next_tab();
    }

    fn prev_tab(&mut self) {
        self.inner.prev_tab();
    }

    fn tab_count(&self) -> usize {
        self.inner.tab_count()
    }

    fn get_active_tab_id(&self) -> u64 {
        self.inner.active_id().map(|id| id.as_u64()).unwrap_or(0)
    }

    fn get_content(&self) -> String {
        self.inner.content().unwrap_or_default()
    }

    fn get_content_length(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.len())
            .unwrap_or(0)
    }

    fn insert_text(&mut self, text: &str) {
        let _ = self.inner.insert(text);
    }

    fn delete_backward(&mut self) {
        let _ = self.inner.delete_backward();
    }

    fn delete_forward(&mut self) {
        let _ = self.inner.delete_forward();
    }

    fn set_cursor(&mut self, pos: usize) {
        let _ = self.inner.set_cursor(pos);
    }

    fn add_cursor(&mut self, pos: usize) {
        let _ = self.inner.add_cursor(pos);
    }

    fn move_cursors(&mut self, delta: i64, extend: bool) {
        let _ = self.inner.move_cursors(delta as isize, extend);
    }

    fn select_all(&mut self) {
        let _ = self.inner.select_all();
    }

    fn get_cursor_position(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.primary_selection().head)
            .unwrap_or(0)
    }

    fn open_file(&mut self, path: &str) -> String {
        match self.inner.open_file(path) {
            Ok(_) => String::new(),
            Err(e) => e.to_string(),
        }
    }

    fn save_file(&mut self) -> String {
        match self.inner.save() {
            Ok(_) => String::new(),
            Err(e) => e.to_string(),
        }
    }

    fn save_file_as(&mut self, path: &str) -> String {
        match self.inner.save_as(path) {
            Ok(_) => String::new(),
            Err(e) => e.to_string(),
        }
    }

    fn is_dirty(&self) -> bool {
        self.inner.active().map(|d| d.is_dirty()).unwrap_or(false)
    }

    fn get_title(&self) -> String {
        self.inner.active().map(|d| d.title()).unwrap_or_default()
    }
}
