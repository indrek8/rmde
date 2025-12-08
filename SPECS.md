# SPECS.md

Detailed technical specifications for planned features.

---

## Architecture Decision: Hybrid Approach

**NSTextView handles:**
- Text editing
- Undo/redo (native NSUndoManager)
- Cursor and selection
- Keyboard navigation

**Rust handles:**
- File I/O (open, save)
- Tab/document state
- tree-sitter parsing (syntax highlighting)
- Large file storage (rope)
- Future AI inference

This means:
- No custom undo/redo in Rust
- No multi-cursor in v1 (requires Rust to control editing)
- Simpler sync: Swift sends content to Rust for storage/parsing

---

## Phase 2: Native Editing

### Enable Native Undo

```swift
// EditorView.swift
textView.allowsUndo = true  // Enable NSUndoManager
```

Remove custom undo menu commands - let macOS handle Cmd+Z/Cmd+Shift+Z natively.

### Simplified Sync

Swift → Rust: Send full content on save or when parsing needed
Rust → Swift: Send content on file open

```swift
// EditorState.swift
func syncContentToRust() {
    editor.set_content(textView.string)
}

func loadContentFromRust() {
    textView.string = editor.get_content()
}
```

---

## Phase 3: Syntax Highlighting

### Dependencies

```toml
tree-sitter = "0.24"
tree-sitter-md = "0.3"
```

### Highlight Spans

```rust
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub kind: HighlightKind,
}

pub enum HighlightKind {
    Heading1, Heading2, Heading3, Heading4, Heading5, Heading6,
    Bold,
    Italic,
    Strikethrough,
    Code,
    CodeBlock,
    Link,
    LinkUrl,
    ListMarker,
    Blockquote,
}
```

### Parsing Flow

1. Swift: `textDidChange` → send content to Rust
2. Rust: Parse with tree-sitter, return highlight spans
3. Swift: Apply spans as NSAttributedString attributes

```swift
func updateHighlighting() {
    editor.set_content(textView.string)
    let spans = editor.get_highlights()
    applyHighlights(spans)
}
```

### Incremental Parsing

tree-sitter tracks edits for efficient re-parsing:

```rust
fn update_content(&mut self, new_content: &str) {
    // For now: full reparse (simple)
    // Future: track edit positions for incremental update
    self.content = Rope::from_str(new_content);
    self.tree = self.parser.parse(new_content, None);
}
```

---

## Phase 4: Show/Hide Markdown

"Ghost mode" — toggle syntax visibility without text reflow.

### Concept

- Syntax characters (`#`, `*`, `[`, etc.) always in buffer
- Show ON: dimmed gray
- Show OFF: foreground = background (invisible)
- Monospace font ensures no position shift

### Syntax Marker Spans

| Markdown | Kind | Hidden |
|----------|------|--------|
| `# ` | HeadingMark | `#` + space |
| `**` | BoldMark | Both sides |
| `*` | ItalicMark | Both sides |
| `` ` `` | CodeMark | Backticks |
| `[]()` | LinkMark | Brackets, parens |
| `- ` | ListMark | `-` + space |

### Implementation

```swift
func applyGhostStyling(visible: Bool) {
    let spans = editor.get_marker_spans()
    for span in spans {
        let range = NSRange(location: span.start, length: span.end - span.start)
        let color = visible ? NSColor.gray : textView.backgroundColor
        storage.addAttribute(.foregroundColor, value: color, range: range)
    }
}
```

---

## Phase 5: Polish

### Theme System

```swift
struct EditorTheme: Codable {
    let name: String
    let background: ColorHex
    let foreground: ColorHex
    let selection: ColorHex
    let syntax: SyntaxColors
}
```

### Find & Replace

| Shortcut | Action |
|----------|--------|
| Cmd+F | Open find bar |
| Cmd+G | Find next |
| Cmd+Shift+G | Find previous |
| Cmd+Option+F | Find and replace |

### Window State Persistence

```swift
struct WindowState: Codable {
    let frame: CGRect
    let tabs: [TabState]
    let activeTabIndex: Int
    let showMarkdown: Bool
}
```

---

## Phase 6: AI Integration

### Provider Abstraction

```rust
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn complete(&self, prompt: &str, context: &str) -> Result<String>;
    fn stream_complete(&self, prompt: &str, context: &str) -> impl Stream<Item = String>;
}
```

### Inline Suggestions

1. Trigger: 500ms pause in typing
2. Display: Gray ghost text after cursor
3. Accept: Tab
4. Dismiss: Esc or any keystroke

---

## Future: Multi-Cursor

**Deferred to post-v1.**

Multi-cursor requires Rust to control editing (intercept all keystrokes, route through Rust). Current architecture uses NSTextView for editing.

If needed later, would require:
1. Intercept `insertText()` in NSTextView
2. Route all edits through Rust
3. Update NSTextView from Rust state
4. Custom undo system in Rust

This is significant architectural work - only pursue if multi-cursor is essential.
