/// A selection in the document, represented by anchor and head positions.
/// When anchor == head, this is a cursor (no selection).
/// When anchor != head, the text between them is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// The anchor point (where selection started)
    pub anchor: usize,
    /// The head point (cursor position, where selection ends)
    pub head: usize,
}

impl Selection {
    /// Create a new cursor (no selection) at the given position
    pub fn cursor(pos: usize) -> Self {
        Self {
            anchor: pos,
            head: pos,
        }
    }

    /// Create a selection from anchor to head
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    /// Returns true if this is just a cursor (no selection)
    pub fn is_cursor(&self) -> bool {
        self.anchor == self.head
    }

    /// Get the start of the selection (min of anchor/head)
    pub fn start(&self) -> usize {
        self.anchor.min(self.head)
    }

    /// Get the end of the selection (max of anchor/head)
    pub fn end(&self) -> usize {
        self.anchor.max(self.head)
    }

    /// Get the length of the selection
    pub fn len(&self) -> usize {
        self.end() - self.start()
    }

    /// Check if selection is empty (cursor)
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// Check if a position is within this selection
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start() && pos < self.end()
    }

    /// Move the cursor/selection by a delta, clamping to max
    pub fn move_by(&mut self, delta: isize, max: usize, extend: bool) {
        let new_head = if delta < 0 {
            self.head.saturating_sub((-delta) as usize)
        } else {
            (self.head + delta as usize).min(max)
        };

        self.head = new_head;
        if !extend {
            self.anchor = new_head;
        }
    }

    /// Collapse the selection to a cursor at the head position
    pub fn collapse(&mut self) {
        self.anchor = self.head;
    }

    /// Collapse to the start of the selection
    pub fn collapse_to_start(&mut self) {
        let start = self.start();
        self.anchor = start;
        self.head = start;
    }

    /// Collapse to the end of the selection
    pub fn collapse_to_end(&mut self) {
        let end = self.end();
        self.anchor = end;
        self.head = end;
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::cursor(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
