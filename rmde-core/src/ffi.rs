//! FFI bridge for Swift interop via swift-bridge

use crate::{DocumentId, Editor, MarkdownParser};

#[allow(clippy::unnecessary_cast)]
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        // Editor type and methods
        type RMDEEditor;

        #[swift_bridge(init)]
        fn new() -> RMDEEditor;

        fn new_tab(&mut self) -> u64;
        fn close_tab(&mut self, id: u64) -> bool;
        fn switch_tab(&mut self, id: u64) -> bool;
        fn next_tab(&mut self);
        fn prev_tab(&mut self);
        fn tab_count(&self) -> usize;
        fn get_active_tab_id(&self) -> u64;

        fn get_content(&self) -> String;
        fn get_content_length(&self) -> usize;
        fn insert_text(&mut self, text: &str);
        fn delete_backward(&mut self);
        fn delete_forward(&mut self);

        fn set_cursor(&mut self, pos: usize);
        fn add_cursor(&mut self, pos: usize);
        fn move_cursors(&mut self, delta: i64, extend: bool);
        fn select_all(&mut self);
        fn get_cursor_position(&self) -> usize;
        fn get_cursor_line(&self) -> usize;
        fn get_cursor_column(&self) -> usize;
        fn get_line_count(&self) -> usize;

        fn apply_edit(&mut self, pos: usize, delete_len: usize, text: &str);

        fn open_file(&mut self, path: &str) -> String;
        fn save_file(&mut self) -> String;
        fn save_file_as(&mut self, path: &str) -> String;

        fn is_dirty(&self) -> bool;
        fn get_title(&self) -> String;
    }

    extern "Rust" {
        // Parser type and methods
        type RMDEParser;

        #[swift_bridge(init)]
        fn new_parser() -> RMDEParser;

        fn parse(&mut self, content: &str) -> Vec<u64>;
        fn reset_parser(&mut self);
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

    fn apply_edit(&mut self, pos: usize, delete_len: usize, text: &str) {
        let _ = self.inner.apply_edit(pos, delete_len, text);
    }

    fn get_cursor_position(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.primary_selection().head)
            .unwrap_or(0)
    }

    fn get_cursor_line(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.cursor_line_col().0)
            .unwrap_or(1)
    }

    fn get_cursor_column(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.cursor_line_col().1)
            .unwrap_or(1)
    }

    fn get_line_count(&self) -> usize {
        self.inner
            .active()
            .map(|d| d.line_count())
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

/// Wrapper around MarkdownParser for FFI
pub struct RMDEParser {
    inner: MarkdownParser,
}

impl RMDEParser {
    fn new_parser() -> Self {
        Self {
            inner: MarkdownParser::new().expect("Failed to create parser"),
        }
    }

    /// Parse content and return packed spans: [start, end, kind, ...]
    fn parse(&mut self, content: &str) -> Vec<u64> {
        let spans = self.inner.parse(content);
        let mut result = Vec::with_capacity(spans.len() * 3);
        for span in spans {
            result.push(span.start as u64);
            result.push(span.end as u64);
            result.push(span.kind as u64);
        }
        result
    }

    fn reset_parser(&mut self) {
        self.inner.reset();
    }
}
