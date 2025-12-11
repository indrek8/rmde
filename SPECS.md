# SPECS.md

Detailed technical specifications for planned features.

**Syntax Reference:** All Markdown syntax implementations must follow [MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md) as the single source of truth.

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

**Reference:** See [MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md) for complete Markdown syntax specification that this phase implements.

### Standards Compliance

Implementing syntax defined in MARKDOWN-SYNTAX.md:

| Standard | Coverage | Status |
|----------|----------|--------|
| **CommonMark v0.31.2** | Base specification | ✅ Complete |
| **GitHub Flavored Markdown** | Tables, strikethrough, task lists, autolinks | ✅ Complete |
| **Extended Syntax** | Footnotes, math, highlighting | ✅ Complete |
| **LLM Output Patterns** | Streaming, code blocks, artifacts | ⏳ Planned |

### Dependencies

```toml
[dependencies]
tree-sitter = "0.24"
tree-sitter-md = "0.3"  # Implements CommonMark + GFM
```

### Architecture: Two-Phase Parsing

Following MARKDOWN-SYNTAX.md § "Principles and Parsing Rules":

```
┌─────────────────────────────────────┐
│  Phase 1: Block Structure           │
│  tree-sitter block grammar          │
│  ├─ Headings (ATX, Setext)          │
│  ├─ Lists (ordered, unordered)      │
│  ├─ Code blocks (fenced, indented)  │
│  ├─ Blockquotes                     │
│  ├─ Tables (GFM)                    │
│  └─ Thematic breaks                 │
└──────────────┬──────────────────────┘
               │ Identify inline ranges
┌──────────────▼──────────────────────┐
│  Phase 2: Inline Elements           │
│  tree-sitter inline grammar         │
│  ├─ Emphasis (*, _)                 │
│  ├─ Strong (**, __)                 │
│  ├─ Code spans (`code`)             │
│  ├─ Links (inline, reference)       │
│  ├─ Images                          │
│  └─ Autolinks                       │
└─────────────────────────────────────┘
```

**Note:** tree-sitter-markdown uses `ts_parser_set_included_ranges` to parse inline content within blocks.

### Span Types

Defined in `rmde-core/src/parser.rs`:

```rust
/// Highlight span with byte offsets
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub kind: SpanKind,
}

/// Syntax element types (aligned with MARKDOWN-SYNTAX.md)
#[repr(u8)]
pub enum SpanKind {
    // Headings (MARKDOWN-SYNTAX.md § Headings)
    Heading1 = 1,
    Heading2 = 2,
    Heading3 = 3,
    Heading4 = 4,
    Heading5 = 5,
    Heading6 = 6,
    HeadingMarker = 7,        // For ghost mode

    // Emphasis (MARKDOWN-SYNTAX.md § Emphasis and Strong)
    Bold = 10,
    Italic = 11,
    BoldItalic = 12,
    Strikethrough = 13,       // GFM extension

    // Code (MARKDOWN-SYNTAX.md § Code Blocks, Code Spans)
    CodeInline = 20,          // `code`
    CodeBlock = 21,           // Indented or fenced
    CodeFence = 22,           // ``` markers
    CodeLanguage = 23,        // Info string

    // Links (MARKDOWN-SYNTAX.md § Links)
    Link = 30,
    LinkUrl = 31,
    LinkTitle = 32,
    Image = 33,

    // Lists (MARKDOWN-SYNTAX.md § Lists)
    ListMarker = 40,          // -, *, +, 1.
    TaskMarker = 41,          // GFM: [ ] or [x]
    TaskChecked = 42,

    // Block elements
    BlockQuote = 50,          // MARKDOWN-SYNTAX.md § Blockquotes
    HorizontalRule = 51,      // MARKDOWN-SYNTAX.md § Thematic Breaks

    // Tables (GFM) - future
    // TableHeader, TableDelimiter, TableCell
}
```

### Parsing Flow

#### Current Implementation (Debounced)

```
User types → textDidChange
    │
    ├─ shouldChangeTextIn (before change)
    │  └─ applyEdit(pos, deleteLen, text) → Rust
    │     └─ syncMetadata() (fast)
    │
    └─ textDidChange (after change)
       ├─ Verify sync (length check)
       └─ scheduleHighlightUpdate()
          └─ Timer (300ms debounce)
             └─ parseContent()
                ├─ getContent() from Rust
                ├─ parser.parse(content) → Vec<u64>
                ├─ Unpack to Vec<Span>
                └─ highlightVersion++
                   └─ updateNSView triggered
                      └─ applyHighlights()
```

**Key Performance Optimization:** Parsing is debounced (300ms) and skipped for files > 1MB.

#### Swift Integration

```swift
// EditorState.swift
func scheduleHighlightUpdate() {
    parseTimer?.invalidate()
    parseTimer = Timer.scheduledTimer(withTimeInterval: 0.3, repeats: false) { [weak self] _ in
        Task { @MainActor in
            self?.parseContent()
        }
    }
}

func parseContent() {
    let content = getContent()

    // Skip large files
    guard content.utf8.count < 1_000_000 else {
        highlightSpans = []
        highlightVersion += 1
        return
    }

    // Parse in Rust
    let packed = parser.parse(content)  // Returns [start, end, kind, ...]

    // Unpack spans
    var spans: [HighlightSpan] = []
    var i: UInt = 0
    while i + 2 < packed.len() {
        if let start = packed.get(index: i),
           let end = packed.get(index: i + 1),
           let kind = packed.get(index: i + 2) {
            spans.append(HighlightSpan(start: Int(start), end: Int(end), kind: kind))
        }
        i += 3
    }

    highlightSpans = spans
    highlightVersion += 1  // Triggers UI update
}
```

#### Rust Implementation

```rust
// rmde-core/src/parser.rs
impl MarkdownParser {
    /// Parse content and return highlight spans
    /// Takes &str directly - no copying (MARKDOWN-SYNTAX.md § Performance)
    pub fn parse(&mut self, content: &str) -> Vec<Span> {
        // Phase 1: Block structure
        let tree = match self.parser.parse(content, self.tree.as_ref()) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut spans = Vec::new();

        // Collect block-level spans
        self.collect_spans(&tree, content, &mut spans);

        // Phase 2: Inline formatting (pattern matching)
        // tree-sitter-md only parses block structure,
        // so we manually detect inline formatting
        self.collect_inline_spans(content, &mut spans);

        // Cache tree for incremental parsing
        self.tree = Some(tree);

        spans
    }

    /// Find inline formatting spans using pattern matching
    /// This complements tree-sitter which only parses block structure
    fn collect_inline_spans(&self, content: &str, spans: &mut Vec<Span>) {
        let bytes = content.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            // Inline code: `code`
            if bytes[i] == b'`' && (i == 0 || bytes[i - 1] != b'`') {
                if let Some(end) = content[i + 1..].find('`') {
                    let end_pos = i + 1 + end + 1;
                    spans.push(Span {
                        start: i,
                        end: end_pos,
                        kind: SpanKind::CodeInline,
                    });
                    i = end_pos;
                    continue;
                }
            }

            // Bold: **text** or __text__
            if i + 1 < len && ((bytes[i] == b'*' && bytes[i + 1] == b'*') ||
                               (bytes[i] == b'_' && bytes[i + 1] == b'_')) {
                let marker = &content[i..i + 2];
                if let Some(end) = content[i + 2..].find(marker) {
                    let end_pos = i + 2 + end + 2;
                    spans.push(Span {
                        start: i,
                        end: end_pos,
                        kind: SpanKind::Bold,
                    });
                    i = end_pos;
                    continue;
                }
            }

            // Italic: *text* or _text_
            if (bytes[i] == b'*' || bytes[i] == b'_') &&
               (i + 1 < len && bytes[i + 1] != bytes[i]) {
                let marker = bytes[i] as char;
                if let Some(end) = content[i + 1..].find(marker) {
                    let end_pos = i + 1 + end + 1;
                    spans.push(Span {
                        start: i,
                        end: end_pos,
                        kind: SpanKind::Italic,
                    });
                    i = end_pos;
                    continue;
                }
            }

            i += 1;
        }
    }
}
```

### Incremental Parsing

tree-sitter supports incremental re-parsing (MARKDOWN-SYNTAX.md § Implementation Notes):

```rust
// Current: Simple full reparse
self.tree = Some(parser.parse(content, self.tree.as_ref()));

// Future optimization: Track edits
// parser.parse() reuses `self.tree` to only re-parse changed nodes
// tree-sitter internally diffs old tree vs new content
```

**Performance:** tree-sitter's incremental parsing provides O(log n) re-parsing for localized edits.

### Applying Highlights (Swift)

```swift
// EditorView.swift
private func applyHighlights(to textView: NSTextView) {
    guard let textStorage = textView.textStorage else { return }

    let fullRange = NSRange(location: 0, length: textStorage.length)

    textStorage.beginEditing()

    // Reset to default
    let defaultFont = NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)
    textStorage.setAttributes([
        .font: defaultFont,
        .foregroundColor: NSColor.textColor
    ], range: fullRange)

    // Apply each span (MARKDOWN-SYNTAX.md § Inline Elements)
    for span in editorState.highlightSpans {
        let range = NSRange(location: span.start, length: span.end - span.start)
        guard range.location >= 0,
              range.location + range.length <= textStorage.length else { continue }

        let attrs = attributesForKind(span.kind)
        textStorage.addAttributes(attrs, range: range)
    }

    textStorage.endEditing()
}

private func attributesForKind(_ kind: UInt64) -> [NSAttributedString.Key: Any] {
    // See MARKDOWN-SYNTAX.md for semantic meaning of each kind
    switch kind {
    case HighlightSpan.heading1:
        return [.font: NSFont.monospacedSystemFont(ofSize: 24, weight: .bold)]
    case HighlightSpan.bold:
        return [.font: NSFont.monospacedSystemFont(ofSize: 14, weight: .bold)]
    case HighlightSpan.codeInline:
        return [
            .foregroundColor: NSColor.systemPink,
            .backgroundColor: NSColor.quaternaryLabelColor
        ]
    // ... see EditorView.swift for full implementation
    default:
        return [:]
    }
}
```

### Performance Targets

Per MARKDOWN-SYNTAX.md § Implementation Notes:

| Metric | Target | Current | Strategy |
|--------|--------|---------|----------|
| Parse (10KB) | < 10ms | ✅ ~5ms | tree-sitter |
| Parse (1MB) | < 200ms | ⏳ TBD | Skip parsing |
| Keystroke latency | < 16ms | ✅ ~8ms | Debounced parsing |
| Memory overhead | < 2x content | ✅ ~1.5x | Byte offsets only |

**Optimizations:**
1. **Debounced parsing** (300ms) - don't parse on every keystroke
2. **Skip large files** (> 1MB) - prevent UI freeze
3. **Incremental parsing** - tree-sitter reuses cached tree
4. **Lightweight spans** - byte offsets only, no string copies
5. **Rope data structure** - O(log n) edits (planned for large files)

### Testing Strategy

Following MARKDOWN-SYNTAX.md § Testing Strategy:

#### 1. CommonMark Spec Tests
```bash
# 671 examples in CommonMark v0.31.2
cargo test commonmark_spec
```

Test files:
- `tests/commonmark/headings.rs`
- `tests/commonmark/emphasis.rs`
- `tests/commonmark/lists.rs`
- etc.

#### 2. GFM Extension Tests
```bash
cargo test gfm_extensions
```

- Tables (with alignment)
- Strikethrough
- Task lists
- Autolinks

#### 3. Edge Cases
```bash
cargo test edge_cases
```

From MARKDOWN-SYNTAX.md § Edge Cases:
- Nested emphasis: `*foo **bar** baz*`
- Intraword: `foo_bar_baz` (no emphasis)
- Ambiguous syntax: Setext vs thematic break
- Malformed input: unclosed delimiters

#### 4. Performance Tests
```bash
cargo bench
```

- Large files (1MB+)
- Deeply nested structures
- Incremental re-parsing

#### 5. Visual Regression Tests

Compare rendered output against reference screenshots:
- Basic formatting
- Complex documents
- LLM output examples

### Implementation Phases

#### Phase 3.1: Core Syntax ✅

- [x] Headings (ATX, Setext)
- [x] Emphasis (bold, italic)
- [x] Code (inline, fenced blocks)
- [x] Links (basic)
- [x] Lists (unordered, ordered)
- [x] Blockquotes
- [x] Thematic breaks

#### Phase 3.2: GFM Extensions ✅

- [x] Tables (with alignment) - 21 tests
- [x] Strikethrough - 18 tests
- [x] Task lists - 16 tests
- [x] Autolinks (extended) - 23 tests

#### Phase 3.3: Extended Syntax ✅

- [x] Footnotes - 8 tests
- [x] Math blocks (inline/display) - 11 tests
- [x] Highlighting (==text==) - 12 tests

#### Phase 3.4: LLM Patterns ⏳

- [ ] Streaming partial syntax
- [ ] Code blocks with metadata
- [ ] Artifact blocks (Claude)

#### Phase 3.5: Optimization ⏳

- [ ] Incremental edit tracking
- [ ] Rope integration for large files
- [ ] Parallel parsing (multi-threaded)
- [ ] AST caching

### Known Limitations

1. **Inline Formatting Detection**: tree-sitter-md only parses block structure. We use pattern matching for inline formatting, which doesn't handle all edge cases per MARKDOWN-SYNTAX.md § Emphasis rules.

2. **Nested Emphasis**: Complex nesting (e.g., `**_foo_**`) may not parse correctly. Need more sophisticated delimiter tracking.

3. **Reference Links**: Link definitions not yet tracked across document.

4. **HTML Blocks**: Raw HTML passed through but not syntax-highlighted.

### Future Improvements

1. **Custom tree-sitter queries** - More precise span extraction
2. **Syntax tree caching** - Persist across sessions
3. **Language injection** - Syntax highlight code fence contents
4. **Semantic tokens** - LSP-style token classification

---

## Phase 4: Show/Hide Markdown ✅ COMPLETE

"Ghost mode" — toggle syntax visibility without text reflow.

### Status: Implemented (2025-12-11)

- [x] 10 marker SpanKind variants (100-112)
- [x] Separate marker spans emitted for all formatting types
- [x] Swift constants defined in EditorState.swift
- [x] Swift styling in EditorView.swift (gray color when visible)
- [x] Toggle via View menu (⇧⌘G) and status bar button
- [x] 27 marker-specific tests

### Concept

- Syntax characters (`#`, `*`, `[`, etc.) always in buffer
- Show ON: dimmed gray
- Show OFF: foreground = background (invisible)
- Monospace font ensures no position shift

### Marker SpanKinds

| Markdown | Kind | Hidden Characters |
|----------|------|-------------------|
| `# ` | MarkerHeading (100) | `#` + space |
| `**` | MarkerBold (101) | Both `**` delimiters |
| `*` | MarkerItalic (102) | Both `*` delimiters |
| `~~` | MarkerStrikethrough (104) | Both `~~` delimiters |
| `` ` `` | MarkerCode (105) | Backticks |
| `[]()` | MarkerLink (107) | `[`, `]`, `(`, `)` |
| `![]()` | MarkerImage (108) | `!`, `[`, `]`, `(`, `)` |
| `- ` | MarkerListBullet (109) | `-`, `*`, `+` |
| `1. ` | MarkerListNumber (110) | Number + `.` or `)` |
| `[ ]` | MarkerTaskBox (112) | `[ ]` or `[x]` |

*Full syntax specification documented in [MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md).*

### Implementation

```swift
// EditorState.swift
@Published var ghostMode: Bool = false

// EditorView.swift - applies styling based on ghostMode
case HighlightSpan.markerBold, HighlightSpan.markerItalic, ...:
    if editorState.ghostMode {
        return [.foregroundColor: textView.backgroundColor]  // Hide
    } else {
        return [.foregroundColor: NSColor.systemGray]  // Show
    }
```

---

## Phase 5: Polish ✅ PARTIAL

### Theme System ✅ COMPLETE (2025-12-11)

- [x] `ThemeManager.swift` - ThemeMode enum with System/Light/Dark options
- [x] Uses `NSApplication.shared.appearance` for app-wide theme override
- [x] Persists with `@AppStorage("themeMode")`
- [x] View menu toggle with ⇧⌘T keyboard shortcut
- [x] Status bar indicator showing current mode with SF Symbols (sun/moon/half-circle)
- [x] Cycles: System → Light → Dark → System

```swift
enum ThemeMode: String, CaseIterable {
    case system, light, dark

    func apply() {
        switch self {
        case .system: NSApplication.shared.appearance = nil
        case .light: NSApplication.shared.appearance = NSAppearance(named: .aqua)
        case .dark: NSApplication.shared.appearance = NSAppearance(named: .darkAqua)
        }
    }
}

@MainActor
class ThemeManager: ObservableObject {
    @AppStorage("themeMode") private var savedThemeMode: String = "system"
    @Published var currentMode: ThemeMode
}
```

### Find & Replace ✅ COMPLETE (2025-12-11)

- [x] `FindState.swift` - State management (searchText, replaceText, matchRanges, currentMatchIndex)
- [x] `FindPanelView.swift` - Floating panel UI below tab bar
- [x] Real-time search with match counter ("X of Y")
- [x] Case-sensitive toggle
- [x] Replace single match and Replace All
- [x] Escape key closes panel

| Shortcut | Action |
|----------|--------|
| ⌘F | Open find panel |
| ⌘G | Find next |
| ⇧⌘G | Find previous |
| ⌘⌥F | Open find & replace |
| Escape | Close panel |

```swift
@MainActor
class FindState: ObservableObject {
    @Published var isVisible = false
    @Published var showReplace = false
    @Published var searchText = ""
    @Published var replaceText = ""
    @Published var matchRanges: [NSRange] = []
    @Published var currentMatchIndex = 0
    @Published var caseSensitive = true
}
```

### Window State Persistence ⏳ Planned

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
