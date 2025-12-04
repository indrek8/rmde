use std::path::PathBuf;

use ropey::Rope;

use crate::error::{Error, Result};
use crate::selection::Selection;
use crate::DocumentId;

/// A single document with its content and metadata
pub struct Document {
    /// Unique identifier for this document
    pub(crate) id: DocumentId,
    /// The text content as a rope for efficient editing
    content: Rope,
    /// All selections/cursors in this document
    selections: Vec<Selection>,
    /// File path if saved/opened from disk
    path: Option<PathBuf>,
    /// Whether the document has unsaved changes
    dirty: bool,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            id: DocumentId::new(),
            content: Rope::new(),
            selections: vec![Selection::default()],
            path: None,
            dirty: false,
        }
    }

    /// Open a document from a file path
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        let rope = Rope::from_str(&content);

        Ok(Self {
            id: DocumentId::new(),
            content: rope,
            selections: vec![Selection::default()],
            path: Some(path),
            dirty: false,
        })
    }

    /// Save the document to its file path
    pub fn save(&mut self) -> Result<()> {
        let path = self.path.as_ref().ok_or(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No file path set",
        )))?;

        std::fs::write(path, self.content.to_string())?;
        self.dirty = false;
        Ok(())
    }

    /// Save the document to a new path
    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        std::fs::write(&path, self.content.to_string())?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }

    /// Get the document's ID
    pub fn id(&self) -> DocumentId {
        self.id
    }

    /// Get the file path if set
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Check if document has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the document title (filename or "Untitled")
    pub fn title(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_else(|| "Untitled".to_string())
    }

    /// Get the full content as a string
    pub fn content(&self) -> String {
        self.content.to_string()
    }

    /// Get the length in bytes
    pub fn len(&self) -> usize {
        self.content.len_bytes()
    }

    /// Check if document is empty
    pub fn is_empty(&self) -> bool {
        self.content.len_bytes() == 0
    }

    /// Get number of lines
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get a specific line (0-indexed)
    pub fn line(&self, idx: usize) -> Option<String> {
        if idx < self.content.len_lines() {
            Some(self.content.line(idx).to_string())
        } else {
            None
        }
    }

    /// Get all selections
    pub fn selections(&self) -> &[Selection] {
        &self.selections
    }

    /// Get mutable access to selections
    pub fn selections_mut(&mut self) -> &mut Vec<Selection> {
        &mut self.selections
    }

    /// Get the primary selection (first one)
    pub fn primary_selection(&self) -> &Selection {
        &self.selections[0]
    }

    /// Set a single cursor position
    pub fn set_cursor(&mut self, pos: usize) {
        let pos = pos.min(self.len());
        self.selections = vec![Selection::cursor(pos)];
    }

    /// Add a new cursor at position
    pub fn add_cursor(&mut self, pos: usize) {
        let pos = pos.min(self.len());
        // Don't add duplicate cursors
        if !self.selections.iter().any(|s| s.head == pos && s.is_cursor()) {
            self.selections.push(Selection::cursor(pos));
        }
    }

    /// Insert text at all cursor positions
    pub fn insert(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let text_len = text.len();

        // Sort selections by position (descending) to avoid offset issues
        self.selections.sort_by(|a, b| b.start().cmp(&a.start()));

        for sel in &mut self.selections {
            // Delete any selected text first
            if !sel.is_cursor() {
                let start = sel.start();
                let end = sel.end();
                let start_char = self.content.byte_to_char(start);
                let end_char = self.content.byte_to_char(end);
                self.content.remove(start_char..end_char);
                sel.anchor = start;
                sel.head = start;
            }

            // Insert the text
            let char_idx = self.content.byte_to_char(sel.head);
            self.content.insert(char_idx, text);

            // Move cursor after inserted text
            sel.head += text_len;
            sel.anchor = sel.head;
        }

        // Re-sort selections ascending and adjust for insertions
        self.normalize_selections();
        self.dirty = true;
    }

    /// Delete character before cursor (backspace)
    pub fn delete_backward(&mut self) {
        self.selections.sort_by(|a, b| b.start().cmp(&a.start()));

        for sel in &mut self.selections {
            if sel.is_cursor() {
                if sel.head > 0 {
                    let char_idx = self.content.byte_to_char(sel.head);
                    if char_idx > 0 {
                        self.content.remove((char_idx - 1)..char_idx);
                        sel.head = self.content.char_to_byte(char_idx - 1);
                        sel.anchor = sel.head;
                    }
                }
            } else {
                // Delete selection
                let start = sel.start();
                let end = sel.end();
                let start_char = self.content.byte_to_char(start);
                let end_char = self.content.byte_to_char(end);
                self.content.remove(start_char..end_char);
                sel.head = start;
                sel.anchor = start;
            }
        }

        self.normalize_selections();
        self.dirty = true;
    }

    /// Delete character after cursor (delete key)
    pub fn delete_forward(&mut self) {
        self.selections.sort_by(|a, b| b.start().cmp(&a.start()));

        for sel in &mut self.selections {
            if sel.is_cursor() {
                let char_idx = self.content.byte_to_char(sel.head);
                if char_idx < self.content.len_chars() {
                    self.content.remove(char_idx..(char_idx + 1));
                }
            } else {
                // Delete selection
                let start = sel.start();
                let end = sel.end();
                let start_char = self.content.byte_to_char(start);
                let end_char = self.content.byte_to_char(end);
                self.content.remove(start_char..end_char);
                sel.head = start;
                sel.anchor = start;
            }
        }

        self.normalize_selections();
        self.dirty = true;
    }

    /// Move all cursors by delta
    pub fn move_cursors(&mut self, delta: isize, extend: bool) {
        let max = self.len();
        for sel in &mut self.selections {
            sel.move_by(delta, max, extend);
        }
        self.normalize_selections();
    }

    /// Normalize selections: sort, merge overlapping, ensure at least one
    fn normalize_selections(&mut self) {
        if self.selections.is_empty() {
            self.selections.push(Selection::default());
            return;
        }

        // Sort by start position
        self.selections.sort_by_key(|s| s.start());

        // Merge overlapping selections
        let mut merged: Vec<Selection> = Vec::with_capacity(self.selections.len());
        for sel in self.selections.drain(..) {
            if let Some(last) = merged.last_mut() {
                if sel.start() <= last.end() {
                    // Overlapping - merge
                    last.head = last.end().max(sel.end());
                    last.anchor = last.start().min(sel.start());
                    continue;
                }
            }
            merged.push(sel);
        }

        self.selections = merged;
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.selections = vec![Selection::new(0, self.len())];
    }

    /// Find next occurrence of text from current position and add cursor
    pub fn select_next_occurrence(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let content = self.content();
        let start_pos = self.selections.last().map(|s| s.end()).unwrap_or(0);

        // Search from current position
        if let Some(pos) = content[start_pos..].find(text) {
            let abs_pos = start_pos + pos;
            self.selections
                .push(Selection::new(abs_pos, abs_pos + text.len()));
        } else if start_pos > 0 {
            // Wrap around to beginning
            if let Some(pos) = content[..start_pos].find(text) {
                self.selections
                    .push(Selection::new(pos, pos + text.len()));
            }
        }
    }

    /// Get text of primary selection
    pub fn selected_text(&self) -> Option<String> {
        let sel = self.primary_selection();
        if sel.is_cursor() {
            None
        } else {
            let start_char = self.content.byte_to_char(sel.start());
            let end_char = self.content.byte_to_char(sel.end());
            Some(self.content.slice(start_char..end_char).to_string())
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
