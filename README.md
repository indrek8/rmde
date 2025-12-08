# RMDE — Rust Markdown Editor

A minimal, lightning-fast, plain-text editor for Markdown files. Built with a Rust core and native macOS UI for developers and writers who want a clean, local-first editing experience without bloat.

## Why RMDE

- **Rust-powered speed** — sub-100ms startup, <16ms keystroke latency
- **Markdown-native** — open, edit, and save `.md` files directly
- **Zero cloud lock-in** — plain text files only, no proprietary formats
- **Local-first** — your notes stay local unless you explicitly sync them

## Features

- [x] Open and edit Markdown files
- [x] Multiple tabs with dirty indicators
- [x] Native macOS UI (SwiftUI + TextKit 2)
- [x] Native undo/redo (NSUndoManager)
- [x] Syntax highlighting (tree-sitter)
  - ATX and Setext headings
  - Bold, italic, strikethrough
  - Inline and fenced code blocks
  - Links, images, autolinks
  - Lists (bullet, numbered, task lists)
  - Block quotes, horizontal rules
  - GFM tables
  - Extended syntax (highlights, math, footnotes)
- [x] Status bar with line/column display
- [ ] Ghost mode (hide/show markdown syntax)
- [ ] Theme system (light/dark)
- [ ] Find & replace

## Build & Run

**Prerequisites:**
- Rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Xcode 15+
- XcodeGen (`brew install xcodegen`)

```bash
git clone git@github.com:indrek8/rmde.git
cd rmde

# Build Rust core + generate Xcode project
source $HOME/.cargo/env
make xcode

# Open in Xcode and run (Cmd+R)
open RMDE/RMDE.xcodeproj
```

## Architecture

Hybrid approach: NSTextView handles editing natively, Rust handles what it's best at.

```
┌─────────────────────────────────────────┐
│         NSTextView (TextKit 2)          │
│  • Native text editing                  │
│  • Undo/redo (NSUndoManager)            │
│  • Cursor & selection                   │
│  • macOS integration (services, etc.)   │
└──────────────────┬──────────────────────┘
                   │ swift-bridge FFI
┌──────────────────▼──────────────────────┐
│            Rust Core (rmde-core)        │
│  • tree-sitter syntax highlighting      │
│  • Large file handling (rope)           │
│  • Tab/document state management        │
│  • File I/O                             │
└─────────────────────────────────────────┘
```

## Project Structure

```
rmde/
├── rmde-core/              # Rust library
│   └── src/
│       ├── lib.rs          # Public exports
│       ├── ffi.rs          # swift-bridge FFI
│       ├── document.rs     # Content storage, file I/O
│       ├── editor.rs       # Tab management
│       ├── parser.rs       # tree-sitter markdown parsing
│       └── selection.rs    # Selection utilities
├── RMDE/                   # macOS app
│   └── Sources/
│       ├── App/            # EditorState (syncs with Rust)
│       ├── Views/          # SwiftUI views
│       └── Bridge/         # Generated FFI (swift-bridge)
├── CLAUDE.md               # AI assistant context
├── SPECS.md                # Technical specifications
├── MARKDOWN-SYNTAX.md      # Markdown syntax reference
└── PLAN.md                 # Implementation roadmap
```

## Documentation

- **[CLAUDE.md](CLAUDE.md)** — Context for AI assistants
- **[SPECS.md](SPECS.md)** — Technical specifications
- **[MARKDOWN-SYNTAX.md](MARKDOWN-SYNTAX.md)** — Comprehensive Markdown syntax reference
- **[PLAN.md](PLAN.md)** — Implementation plan and roadmap

## License

MIT
