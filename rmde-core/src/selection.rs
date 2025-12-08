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
