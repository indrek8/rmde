use std::path::PathBuf;

use crate::document::Document;
use crate::error::{Error, Result};
use crate::{DocumentId, Selection, TabInfo};

/// The main editor state, managing multiple documents as tabs
pub struct Editor {
    /// All open documents
    documents: Vec<Document>,
    /// Index of the active document
    active_idx: usize,
}

impl Editor {
    /// Create a new editor with an empty document
    pub fn new() -> Self {
        Self {
            documents: vec![Document::new()],
            active_idx: 0,
        }
    }

    /// Create a new empty tab and return its ID
    pub fn new_tab(&mut self) -> DocumentId {
        let doc = Document::new();
        let id = doc.id();
        self.documents.push(doc);
        self.active_idx = self.documents.len() - 1;
        id
    }

    /// Open a file in a new tab
    pub fn open_file(&mut self, path: impl Into<PathBuf>) -> Result<DocumentId> {
        let path = path.into();

        // Check if file is already open
        for (idx, doc) in self.documents.iter().enumerate() {
            if doc.path() == Some(&path) {
                self.active_idx = idx;
                return Ok(doc.id());
            }
        }

        let doc = Document::open(&path)?;
        let id = doc.id();
        self.documents.push(doc);
        self.active_idx = self.documents.len() - 1;
        Ok(id)
    }

    /// Close a tab by ID, returns true if closed
    pub fn close_tab(&mut self, id: DocumentId) -> bool {
        if let Some(idx) = self.find_doc_index(id) {
            // Don't close the last document
            if self.documents.len() == 1 {
                // Replace with new empty document
                self.documents[0] = Document::new();
                return true;
            }

            self.documents.remove(idx);

            // Adjust active index
            if self.active_idx >= self.documents.len() {
                self.active_idx = self.documents.len() - 1;
            } else if self.active_idx > idx {
                self.active_idx -= 1;
            }

            true
        } else {
            false
        }
    }

    /// Switch to a specific tab by ID
    pub fn switch_tab(&mut self, id: DocumentId) -> bool {
        if let Some(idx) = self.find_doc_index(id) {
            self.active_idx = idx;
            true
        } else {
            false
        }
    }

    /// Switch to next tab
    pub fn next_tab(&mut self) {
        if !self.documents.is_empty() {
            self.active_idx = (self.active_idx + 1) % self.documents.len();
        }
    }

    /// Switch to previous tab
    pub fn prev_tab(&mut self) {
        if !self.documents.is_empty() {
            self.active_idx = if self.active_idx == 0 {
                self.documents.len() - 1
            } else {
                self.active_idx - 1
            };
        }
    }

    /// Get information about all tabs
    pub fn tabs(&self) -> Vec<TabInfo> {
        self.documents
            .iter()
            .map(|doc| TabInfo {
                id: doc.id().as_u64(),
                title: doc.title(),
                dirty: doc.is_dirty(),
            })
            .collect()
    }

    /// Get the active document ID
    pub fn active_id(&self) -> Option<DocumentId> {
        self.documents.get(self.active_idx).map(|d| d.id())
    }

    /// Get a reference to the active document
    pub fn active(&self) -> Option<&Document> {
        self.documents.get(self.active_idx)
    }

    /// Get a mutable reference to the active document
    pub fn active_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active_idx)
    }

    /// Get the number of open tabs
    pub fn tab_count(&self) -> usize {
        self.documents.len()
    }

    // --- Convenience methods that delegate to active document ---

    /// Save the active document
    pub fn save(&mut self) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .save()
    }

    /// Save the active document to a path
    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .save_as(path)
    }

    /// Get content of active document
    pub fn content(&self) -> Result<String> {
        Ok(self.active().ok_or(Error::NoActiveDocument)?.content())
    }

    /// Insert text in active document
    pub fn insert(&mut self, text: &str) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .insert(text);
        Ok(())
    }

    /// Delete backward in active document
    pub fn delete_backward(&mut self) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .delete_backward();
        Ok(())
    }

    /// Delete forward in active document
    pub fn delete_forward(&mut self) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .delete_forward();
        Ok(())
    }

    /// Get selections from active document
    pub fn selections(&self) -> Result<Vec<Selection>> {
        Ok(self
            .active()
            .ok_or(Error::NoActiveDocument)?
            .selections()
            .to_vec())
    }

    /// Add cursor to active document
    pub fn add_cursor(&mut self, pos: usize) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .add_cursor(pos);
        Ok(())
    }

    /// Set cursor position in active document
    pub fn set_cursor(&mut self, pos: usize) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .set_cursor(pos);
        Ok(())
    }

    /// Move cursors in active document
    pub fn move_cursors(&mut self, delta: isize, extend: bool) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .move_cursors(delta, extend);
        Ok(())
    }

    /// Select all in active document
    pub fn select_all(&mut self) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .select_all();
        Ok(())
    }

    /// Select next occurrence in active document
    pub fn select_next_occurrence(&mut self, text: &str) -> Result<()> {
        self.active_mut()
            .ok_or(Error::NoActiveDocument)?
            .select_next_occurrence(text);
        Ok(())
    }

    // --- Private helpers ---

    fn find_doc_index(&self, id: DocumentId) -> Option<usize> {
        self.documents.iter().position(|d| d.id() == id)
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
