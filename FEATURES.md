# FEATURES.md

Feature tracker for RMDE — Rust Markdown Editor.

## Implemented

### Core Editor
- [x] Rope-based text buffer (ropey) — O(log n) edits
- [x] Multi-cursor selection model (anchor/head)
- [x] Insert text at cursor(s)
- [x] Delete backward (backspace)
- [x] Delete forward (delete key)
- [x] Select all

### Tabs
- [x] Multiple open documents
- [x] New tab (Cmd+T)
- [x] Close tab (click X)
- [x] Switch tabs (click)
- [x] Next/previous tab (Cmd+Shift+]/[)
- [x] Dirty indicator (orange dot)
- [x] Tab title from filename
- [x] Double-click unsaved tab → Save As dialog
- [x] Double-click saved tab → inline file rename

### File Operations
- [x] Open file (Cmd+O)
- [x] Save file (Cmd+S)
- [x] Save as (Cmd+Shift+S)
- [x] Detect already-open files

### UI
- [x] Native macOS app (SwiftUI)
- [x] TextKit 2 text view
- [x] Tab bar
- [x] Status bar (position, character count)
- [x] Monospace font

---

## In Progress

### Phase 2: Core Editing
- [ ] Undo/redo system
- [ ] Cursor navigation (arrows, Cmd+arrows)
- [ ] Proper text sync between NSTextView ↔ Rust
- [ ] Line/column display in status bar

---

## Planned

### Phase 3: Multi-Cursor
- [ ] Cmd+D — select next occurrence
- [ ] Cmd+Click — add cursor
- [ ] Alt+Click+Drag — column selection
- [ ] Synchronized editing across cursors

### Phase 4: Syntax Highlighting
- [ ] tree-sitter markdown parsing
- [ ] Headings (H1-H6)
- [ ] Bold, italic, strikethrough
- [ ] Code spans and code blocks
- [ ] Links and images
- [ ] Lists and blockquotes
- [ ] Incremental re-parsing on edit

### Phase 5: Polish
- [ ] Theme system (light/dark)
- [ ] Customizable font/size
- [ ] Line numbers (optional)
- [ ] Word wrap toggle
- [ ] Find & replace (Cmd+F)
- [ ] Go to line (Cmd+G)
- [ ] Recent files
- [ ] Window state persistence

### Phase 6: AI Integration (Future)
- [ ] Plugin architecture
- [ ] AI provider abstraction
- [ ] Inline completions
- [ ] Chat sidebar
- [ ] Streaming responses

---

## Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Startup time | < 100ms | ✓ |
| Keystroke latency | < 16ms | ✓ |
| Memory (empty) | < 20MB | TBD |
| Memory (1MB file) | < 50MB | TBD |
| File open (1MB) | < 50ms | TBD |

---

## Non-Goals (v1)

- WYSIWYG editing
- Live preview pane
- Vim/Emacs keybindings
- Plugin system
- Mobile/web versions
- Cloud sync
- Collaboration
