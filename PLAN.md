# RMDE Parser Implementation Plan

**Goal:** Implement complete Markdown syntax highlighting per [MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md)

**Current State:** Basic implementation with headings, bold, italic, code, links, lists, blockquotes

---

## Gap Analysis

### Current Implementation vs MARKDOWN-SYNTAX.md

| Feature | MARKDOWN-SYNTAX.md | Current | Gap |
|---------|-------------------|---------|-----|
| **Block Elements** ||||
| ATX Headings (#) | Full spec | ✅ Basic | Missing: closing #, edge cases |
| Setext Headings (===) | Full spec | ⚠️ Partial | Always returns H1, no H2 detection |
| Paragraphs | Full spec | ❌ None | Not tracked as spans |
| Blockquotes (>) | Full spec | ✅ Basic | Missing: nested, lazy continuation |
| Unordered Lists | Full spec | ✅ Markers only | Missing: nesting, tight/loose |
| Ordered Lists | Full spec | ✅ Markers only | Missing: start number, nesting |
| Task Lists (GFM) | Full spec | ⚠️ SpanKind exists | Not implemented |
| Indented Code | Full spec | ✅ Basic | Grouped with fenced |
| Fenced Code | Full spec | ✅ Basic | Missing: fence markers, tilde |
| Thematic Breaks | Full spec | ✅ Basic | Works |
| Tables (GFM) | Full spec | ❌ None | Not implemented |
| HTML Blocks | Full spec | ❌ None | Not implemented |
| **Inline Elements** ||||
| Emphasis (*/_) | Full flanking rules | ⚠️ Simple pattern | Missing: flanking rules, intraword |
| Strong (**/__) | Full flanking rules | ⚠️ Simple pattern | Missing: flanking rules |
| Bold+Italic (***) | Full spec | ⚠️ SpanKind exists | Not detected |
| Strikethrough (~~) | GFM spec | ⚠️ SpanKind exists | Not implemented |
| Code Spans (`) | Full spec | ⚠️ Simple pattern | Missing: multi-backtick, spaces |
| Links [text](url) | Full spec | ✅ Basic | Missing: titles, reference links |
| Images ![alt](url) | Full spec | ✅ Basic | Missing: titles |
| Autolinks <url> | CommonMark | ❌ None | Not implemented |
| Extended Autolinks | GFM | ❌ None | Not implemented |
| Hard Line Breaks | Full spec | ❌ None | Not tracked |
| **Extended Syntax** ||||
| Footnotes | PHP MD Extra | ❌ None | Not implemented |
| Definition Lists | PHP MD Extra | ❌ None | Not implemented |
| Abbreviations | PHP MD Extra | ❌ None | Not implemented |
| Math ($...$) | Extended | ❌ None | Not implemented |
| Highlighting (==) | Extended | ❌ None | Not implemented |
| Sub/Superscript | Extended | ❌ None | Not implemented |
| **Markers (Ghost Mode)** ||||
| Heading markers | # chars | ✅ HeadingMarker | Works |
| Emphasis markers | * _ chars | ❌ None | Need separate marker spans |
| Code markers | ` chars | ❌ None | Need separate marker spans |
| Link markers | []() | ❌ None | Need separate marker spans |

---

## Implementation Phases

### Phase 1: Fix Current Implementation Bugs

**Priority: HIGH** | **Effort: 2-3 days**

#### 1.1 Fix Setext Headings

Current bug: Always returns `Heading1` regardless of underline character.

```rust
// Current (broken):
"setext_heading" => Some(SpanKind::Heading1),

// Fix: Check underline character
"setext_heading" => {
    // Get the underline line
    let text = &content[start..end.min(content.len())];
    if text.contains('=') {
        Some(SpanKind::Heading1)
    } else {
        Some(SpanKind::Heading2)  // '-' underline
    }
}
```

#### 1.2 Fix Inline Code with Multiple Backticks

Per MARKDOWN-SYNTAX.md § Code Spans:
- `` `code` `` - single backtick
- ``` ``code with `backtick` `` ``` - double backticks

```rust
// Current (broken): Only handles single backticks
if bytes[i] == b'`' && (i == 0 || bytes[i - 1] != b'`') {
    if let Some(end) = content[i + 1..].find('`') { ... }
}

// Fix: Count opening backticks, find matching closing
fn find_code_span(&self, content: &str, start: usize) -> Option<(usize, usize)> {
    let bytes = content.as_bytes();
    let mut backtick_count = 0;
    let mut i = start;

    // Count opening backticks
    while i < bytes.len() && bytes[i] == b'`' {
        backtick_count += 1;
        i += 1;
    }

    if backtick_count == 0 { return None; }

    // Find matching closing backticks
    let pattern = "`".repeat(backtick_count);
    if let Some(end_offset) = content[i..].find(&pattern) {
        // Verify it's exactly backtick_count backticks (not more)
        let end = i + end_offset;
        if end + backtick_count >= bytes.len() || bytes[end + backtick_count] != b'`' {
            return Some((start, end + backtick_count));
        }
    }
    None
}
```

#### 1.3 Fix Bold/Italic Detection Order

Current bug: Bold (**) checked before italic (*), but `*foo*` might be parsed incorrectly if within `**bar**`.

```rust
// Fix: Implement proper delimiter stack per CommonMark spec
// See MARKDOWN-SYNTAX.md § "Emphasis and Strong" for flanking rules
struct DelimiterRun {
    start: usize,
    len: usize,
    char: char,
    can_open: bool,
    can_close: bool,
}

fn parse_emphasis(&self, content: &str) -> Vec<Span> {
    let mut delimiters: Vec<DelimiterRun> = Vec::new();
    let mut spans = Vec::new();

    // Phase 1: Find all delimiter runs
    // Phase 2: Match openers with closers per CommonMark algorithm
    // Phase 3: Generate spans

    spans
}
```

#### 1.4 Add Tests for Edge Cases

From MARKDOWN-SYNTAX.md § Edge Cases:

```rust
#[test]
fn test_intraword_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();

    // Underscore in word should NOT be emphasis
    let spans = parser.parse("foo_bar_baz");
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(italic.is_empty(), "foo_bar_baz should not have emphasis");

    // Asterisk in word SHOULD be emphasis
    let spans = parser.parse("foo*bar*baz");
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    assert!(!italic.is_empty(), "foo*bar*baz should have emphasis");
}

#[test]
fn test_nested_emphasis() {
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse("*foo **bar** baz*");

    // Should have: outer italic, inner bold
    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    assert!(!italic.is_empty());
    assert!(!bold.is_empty());
}

#[test]
fn test_code_with_backticks() {
    let mut parser = MarkdownParser::new().unwrap();

    // Double backticks containing single backtick
    let spans = parser.parse("`` `code` ``");
    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    assert_eq!(code.len(), 1);
    assert_eq!(code[0].start, 0);
    assert_eq!(code[0].end, 12);
}
```

---

### Phase 2: GFM Extensions

**Priority: HIGH** | **Effort: 3-4 days**

#### 2.1 Strikethrough (~~text~~)

```rust
// Add to SpanKind (already exists)
Strikethrough = 13,

// Add to collect_inline_spans
fn parse_strikethrough(&self, content: &str, spans: &mut Vec<Span>) {
    let bytes = content.as_bytes();
    let mut i = 0;

    while i + 1 < bytes.len() {
        if bytes[i] == b'~' && bytes[i + 1] == b'~' {
            // Find closing ~~
            if let Some(end_offset) = content[i + 2..].find("~~") {
                let end = i + 2 + end_offset + 2;
                spans.push(Span {
                    start: i,
                    end,
                    kind: SpanKind::Strikethrough,
                });
                i = end;
                continue;
            }
        }
        i += 1;
    }
}
```

#### 2.2 Task Lists

```rust
// Add to SpanKind (already exists)
TaskMarker = 41,    // [ ]
TaskChecked = 42,   // [x] or [X]

// Add to map_node_kind - check if tree-sitter-md exposes this
// If not, add to inline parsing:
fn parse_task_markers(&self, content: &str, spans: &mut Vec<Span>) {
    // Pattern: list marker followed by [ ] or [x]
    // - [ ] unchecked
    // - [x] checked
    let re = regex::Regex::new(r"(?m)^[\s]*[-*+]\s+\[([ xX])\]").unwrap();
    for cap in re.captures_iter(content) {
        let m = cap.get(0).unwrap();
        let checkbox = cap.get(1).unwrap();
        let kind = if checkbox.as_str() == " " {
            SpanKind::TaskMarker
        } else {
            SpanKind::TaskChecked
        };
        spans.push(Span {
            start: m.start(),
            end: m.end(),
            kind,
        });
    }
}
```

#### 2.3 Tables

Add new SpanKind variants:

```rust
// Add to SpanKind
TableHeader = 70,
TableDelimiter = 71,
TableCell = 72,
TableAlignLeft = 73,
TableAlignCenter = 74,
TableAlignRight = 75,

// Check tree-sitter-md for table nodes
// Node types: "pipe_table", "pipe_table_header", "pipe_table_delimiter_row", etc.
"pipe_table" => None,  // Container, don't highlight
"pipe_table_header" => Some(SpanKind::TableHeader),
"pipe_table_delimiter_row" => Some(SpanKind::TableDelimiter),
"pipe_table_cell" => Some(SpanKind::TableCell),
```

#### 2.4 Autolinks

```rust
// Add to SpanKind
Autolink = 34,
AutolinkEmail = 35,

// tree-sitter should handle <url> and <email>
"uri_autolink" => Some(SpanKind::Autolink),
"email_autolink" => Some(SpanKind::AutolinkEmail),

// GFM extended autolinks (bare URLs) need pattern matching
fn parse_extended_autolinks(&self, content: &str, spans: &mut Vec<Span>) {
    // Match http://, https://, www.
    let url_re = regex::Regex::new(
        r"(?i)(https?://[^\s<>\[\]]+|www\.[^\s<>\[\]]+)"
    ).unwrap();

    for m in url_re.find_iter(content) {
        spans.push(Span {
            start: m.start(),
            end: m.end(),
            kind: SpanKind::Autolink,
        });
    }
}
```

---

### Phase 3: Marker Spans for Ghost Mode

**Priority: MEDIUM** | **Effort: 2-3 days**

For Show/Hide Markdown (Phase 4 in roadmap), we need to track syntax markers separately.

#### 3.1 Add Marker SpanKind Variants

```rust
// New marker types for ghost mode
pub enum SpanKind {
    // ... existing ...

    // Markers (for ghost mode - these are the hidden characters)
    MarkerHeading = 100,      // # characters
    MarkerBold = 101,         // ** or __
    MarkerItalic = 102,       // * or _
    MarkerBoldItalic = 103,   // *** or ___
    MarkerStrikethrough = 104, // ~~
    MarkerCode = 105,         // ` characters
    MarkerCodeFence = 106,    // ``` or ~~~
    MarkerLink = 107,         // [ ] ( )
    MarkerImage = 108,        // ! [ ] ( )
    MarkerListBullet = 109,   // - * +
    MarkerListNumber = 110,   // 1. 2) etc
    MarkerBlockquote = 111,   // >
    MarkerTaskBox = 112,      // [ ] or [x]
}
```

#### 3.2 Emit Separate Marker Spans

When parsing emphasis, emit two spans:
1. Content span (for styling the text)
2. Marker span (for ghost mode hiding)

```rust
fn parse_bold_with_markers(&self, content: &str, start: usize, end: usize, spans: &mut Vec<Span>) {
    // Marker span for opening **
    spans.push(Span {
        start,
        end: start + 2,
        kind: SpanKind::MarkerBold,
    });

    // Content span (entire bold text including markers)
    spans.push(Span {
        start,
        end,
        kind: SpanKind::Bold,
    });

    // Marker span for closing **
    spans.push(Span {
        start: end - 2,
        end,
        kind: SpanKind::MarkerBold,
    });
}
```

---

### Phase 4: Extended Syntax

**Priority: LOW** | **Effort: 4-5 days**

#### 4.1 Footnotes

Per MARKDOWN-SYNTAX.md § Footnotes:

```rust
// SpanKind
FootnoteRef = 80,      // [^1]
FootnoteDef = 81,      // [^1]: definition

// Pattern matching (tree-sitter-md may not support)
fn parse_footnotes(&self, content: &str, spans: &mut Vec<Span>) {
    // Reference: [^id]
    let ref_re = regex::Regex::new(r"\[\^([^\]]+)\]").unwrap();
    for m in ref_re.find_iter(content) {
        // Check if it's a definition (followed by :)
        let is_def = content[m.end()..].starts_with(':');
        spans.push(Span {
            start: m.start(),
            end: m.end() + if is_def { 1 } else { 0 },
            kind: if is_def { SpanKind::FootnoteDef } else { SpanKind::FootnoteRef },
        });
    }
}
```

#### 4.2 Math Blocks

```rust
// SpanKind
MathInline = 82,    // $...$
MathBlock = 83,     // $$...$$

fn parse_math(&self, content: &str, spans: &mut Vec<Span>) {
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'$' {
            // Check for block math $$
            if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                if let Some(end) = content[i + 2..].find("$$") {
                    spans.push(Span {
                        start: i,
                        end: i + 2 + end + 2,
                        kind: SpanKind::MathBlock,
                    });
                    i = i + 2 + end + 2;
                    continue;
                }
            }
            // Inline math $
            else if let Some(end) = content[i + 1..].find('$') {
                spans.push(Span {
                    start: i,
                    end: i + 1 + end + 1,
                    kind: SpanKind::MathInline,
                });
                i = i + 1 + end + 1;
                continue;
            }
        }
        i += 1;
    }
}
```

#### 4.3 Highlighting (==text==)

```rust
// SpanKind
Highlight = 84,

fn parse_highlight(&self, content: &str, spans: &mut Vec<Span>) {
    let mut i = 0;
    let bytes = content.as_bytes();

    while i + 1 < bytes.len() {
        if bytes[i] == b'=' && bytes[i + 1] == b'=' {
            if let Some(end) = content[i + 2..].find("==") {
                spans.push(Span {
                    start: i,
                    end: i + 2 + end + 2,
                    kind: SpanKind::Highlight,
                });
                i = i + 2 + end + 2;
                continue;
            }
        }
        i += 1;
    }
}
```

---

### Phase 5: Performance Optimization

**Priority: MEDIUM** | **Effort: 3-4 days**

#### 5.1 Avoid Code Block Interior Parsing

Don't parse inline formatting inside code blocks/spans:

```rust
fn collect_inline_spans(&self, content: &str, spans: &mut Vec<Span>) {
    // First pass: identify code regions to skip
    let mut skip_regions: Vec<(usize, usize)> = Vec::new();
    for span in spans.iter() {
        match span.kind {
            SpanKind::CodeBlock | SpanKind::CodeInline => {
                skip_regions.push((span.start, span.end));
            }
            _ => {}
        }
    }

    // Parse inline, skipping code regions
    let mut i = 0;
    while i < content.len() {
        // Skip if inside code region
        if skip_regions.iter().any(|(s, e)| i >= *s && i < *e) {
            i += 1;
            continue;
        }
        // ... parse emphasis, etc.
    }
}
```

#### 5.2 Incremental Parsing

Leverage tree-sitter's incremental parsing properly:

```rust
pub struct MarkdownParser {
    parser: Parser,
    tree: Option<Tree>,
    last_content_hash: u64,  // Quick change detection
}

impl MarkdownParser {
    pub fn parse_incremental(&mut self, content: &str, edit: Option<InputEdit>) -> Vec<Span> {
        // If we have an edit, update the tree incrementally
        if let (Some(tree), Some(edit)) = (&mut self.tree, edit) {
            tree.edit(&edit);
        }

        // Parse with old tree for incremental speedup
        let new_tree = self.parser.parse(content, self.tree.as_ref());
        // ...
    }
}

// InputEdit from Swift:
pub struct EditInfo {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
    pub start_row: usize,
    pub start_col: usize,
    pub old_end_row: usize,
    pub old_end_col: usize,
    pub new_end_row: usize,
    pub new_end_col: usize,
}
```

#### 5.3 Parallel Parsing for Large Files

```rust
use rayon::prelude::*;

fn parse_large_file(&mut self, content: &str) -> Vec<Span> {
    const CHUNK_SIZE: usize = 64 * 1024;  // 64KB chunks

    if content.len() < CHUNK_SIZE * 2 {
        return self.parse(content);
    }

    // Split at paragraph boundaries
    let chunks = split_at_paragraphs(content, CHUNK_SIZE);

    // Parse chunks in parallel
    let chunk_spans: Vec<Vec<Span>> = chunks
        .par_iter()
        .map(|(offset, chunk)| {
            let mut parser = MarkdownParser::new().unwrap();
            let mut spans = parser.parse(chunk);
            // Adjust offsets
            for span in &mut spans {
                span.start += offset;
                span.end += offset;
            }
            spans
        })
        .collect();

    // Merge results
    chunk_spans.into_iter().flatten().collect()
}
```

---

### Phase 6: LLM Output Patterns

**Priority: LOW** | **Effort: 2-3 days**

#### 6.1 Streaming Partial Syntax

Handle incomplete markdown during LLM streaming:

```rust
pub struct StreamingParser {
    parser: MarkdownParser,
    buffer: String,
    pending_spans: Vec<Span>,
}

impl StreamingParser {
    pub fn append(&mut self, chunk: &str) -> Vec<Span> {
        self.buffer.push_str(chunk);

        // Parse current buffer
        let spans = self.parser.parse(&self.buffer);

        // Filter out spans that might be incomplete (at buffer end)
        let safe_spans: Vec<Span> = spans
            .into_iter()
            .filter(|s| {
                // Keep spans that are clearly complete
                s.end < self.buffer.len() - 10 ||  // 10 char buffer
                self.is_complete_span(s)
            })
            .collect();

        safe_spans
    }

    fn is_complete_span(&self, span: &Span) -> bool {
        match span.kind {
            SpanKind::Bold => {
                // Check closing ** exists
                let text = &self.buffer[span.start..span.end];
                text.ends_with("**") || text.ends_with("__")
            }
            // ... other checks
            _ => true
        }
    }
}
```

#### 6.2 Claude Artifact Blocks

```rust
// SpanKind
ArtifactThinking = 90,  // <antThinking>...</antThinking>
ArtifactMeta = 91,      // <antMeta>...</antMeta>

fn parse_artifacts(&self, content: &str, spans: &mut Vec<Span>) {
    // <antThinking>
    let thinking_re = regex::Regex::new(r"<antThinking>[\s\S]*?</antThinking>").unwrap();
    for m in thinking_re.find_iter(content) {
        spans.push(Span {
            start: m.start(),
            end: m.end(),
            kind: SpanKind::ArtifactThinking,
        });
    }

    // Similar for other artifact types
}
```

---

## Testing Plan

### Unit Tests by Feature

```
tests/
├── commonmark/
│   ├── headings.rs          # ATX, Setext, edge cases
│   ├── emphasis.rs          # Bold, italic, nested, intraword
│   ├── code.rs              # Inline, fenced, indented
│   ├── links.rs             # Inline, reference, autolinks
│   ├── lists.rs             # Ordered, unordered, nesting
│   ├── blockquotes.rs       # Basic, nested, lazy
│   └── thematic_breaks.rs
├── gfm/
│   ├── tables.rs
│   ├── strikethrough.rs
│   ├── task_lists.rs
│   └── autolinks.rs
├── extended/
│   ├── footnotes.rs
│   ├── math.rs
│   └── highlight.rs
├── edge_cases/
│   ├── nested.rs            # Deep nesting
│   ├── malformed.rs         # Unclosed delimiters
│   └── unicode.rs           # Non-ASCII content
└── performance/
    ├── large_files.rs
    └── incremental.rs
```

### Integration Tests

```rust
// tests/integration/commonmark_spec.rs
// Run against official CommonMark spec examples (671 tests)

#[test]
fn commonmark_spec_example_32() {
    // Example 32: Headings
    let input = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo";
    let mut parser = MarkdownParser::new().unwrap();
    let spans = parser.parse(input);

    assert!(spans.iter().any(|s| s.kind == SpanKind::Heading1));
    assert!(spans.iter().any(|s| s.kind == SpanKind::Heading2));
    // ...
}
```

### Benchmarks

```rust
// benches/parsing.rs
use criterion::{criterion_group, Criterion};

fn bench_small_file(c: &mut Criterion) {
    let content = include_str!("../fixtures/small.md");  // ~10KB
    c.bench_function("parse_small", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        b.iter(|| parser.parse(content))
    });
}

fn bench_large_file(c: &mut Criterion) {
    let content = include_str!("../fixtures/large.md");  // ~1MB
    c.bench_function("parse_large", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        b.iter(|| parser.parse(content))
    });
}
```

---

## Implementation Order

### Week 1: Foundation Fixes
1. [ ] Fix Setext heading detection
2. [ ] Fix multi-backtick code spans
3. [ ] Implement proper emphasis parsing (delimiter stack)
4. [ ] Add edge case tests

### Week 2: GFM Extensions
5. [ ] Implement strikethrough
6. [ ] Implement task list markers
7. [ ] Implement tables (if tree-sitter supports)
8. [ ] Implement extended autolinks

### Week 3: Markers & Ghost Mode
9. [ ] Add marker SpanKind variants
10. [ ] Emit separate marker spans
11. [ ] Update Swift highlighting for markers
12. [ ] Implement ghost mode toggle

### Week 4: Extended & Optimization
13. [ ] Implement footnotes
14. [ ] Implement math blocks
15. [ ] Skip code regions in inline parsing
16. [ ] Add benchmarks

### Week 5: Polish
17. [ ] Complete test coverage
18. [ ] Performance optimization
19. [ ] Documentation
20. [ ] LLM streaming patterns

---

## Dependencies to Add

```toml
# Cargo.toml
[dependencies]
tree-sitter = "0.24"
tree-sitter-md = "0.3"
regex = "1.10"          # For pattern matching fallbacks
once_cell = "1.19"      # Lazy static regex compilation

[dev-dependencies]
criterion = "0.5"       # Benchmarking
```

---

## FFI Updates Required

For each new SpanKind, Swift needs corresponding constants:

```swift
// EditorState.swift - HighlightSpan constants
extension HighlightSpan {
    // Existing
    static let heading1: UInt64 = 1
    // ...

    // New (GFM)
    static let strikethrough: UInt64 = 13
    static let tableHeader: UInt64 = 70
    static let tableCell: UInt64 = 72

    // New (Markers)
    static let markerBold: UInt64 = 101
    static let markerItalic: UInt64 = 102
    // ...

    // New (Extended)
    static let footnoteRef: UInt64 = 80
    static let mathInline: UInt64 = 82
    static let highlight: UInt64 = 84
}
```

---

## Success Criteria

### Phase 1 Complete When:
- [ ] All CommonMark heading tests pass
- [ ] Multi-backtick code spans work
- [ ] Emphasis edge cases (intraword, nested) work
- [ ] No regressions in existing functionality

### Phase 2 Complete When:
- [ ] Strikethrough renders correctly
- [ ] Task lists show checkboxes
- [ ] Tables highlight headers/cells
- [ ] Extended autolinks work

### Phase 3 Complete When:
- [ ] Ghost mode can hide/show markers
- [ ] All marker types have separate spans
- [ ] Swift UI toggle works

### Phase 4 Complete When:
- [ ] Footnotes highlight
- [ ] Math blocks highlight
- [ ] All extended syntax works

### Phase 5 Complete When:
- [ ] Parse 10KB < 10ms
- [ ] Parse 1MB < 500ms
- [ ] Incremental parsing works
- [ ] No UI jank during editing

---

## References

- [MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md) - Single source of truth
- [CommonMark Spec](https://spec.commonmark.org/0.31.2/)
- [GFM Spec](https://github.github.com/gfm/)
- [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- [SPECS.md](SPECS.md) - Technical specifications
