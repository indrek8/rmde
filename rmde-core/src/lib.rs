mod document;
mod editor;
mod error;
mod ffi;
mod selection;

pub use document::Document;
pub use editor::Editor;
pub use error::Error;
pub use selection::Selection;

/// Document identifier for tab tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentId(u64);

impl DocumentId {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn from_u64(val: u64) -> Self {
        Self(val)
    }
}

/// Tab information for UI
#[derive(Debug, Clone)]
pub struct TabInfo {
    pub id: u64,
    pub title: String,
    pub dirty: bool,
}

/// Highlight span for syntax highlighting
#[derive(Debug, Clone, Copy)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub kind: HighlightKind,
}

/// Types of syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HighlightKind {
    Heading1 = 1,
    Heading2 = 2,
    Heading3 = 3,
    Heading4 = 4,
    Heading5 = 5,
    Heading6 = 6,
    Bold = 10,
    Italic = 11,
    Code = 20,
    CodeBlock = 21,
    Link = 30,
    LinkUrl = 31,
    ListMarker = 40,
    BlockQuote = 50,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_id_unique() {
        let id1 = DocumentId::new();
        let id2 = DocumentId::new();
        assert_ne!(id1, id2);
    }
}
