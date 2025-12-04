# RMDE — Rust Markdown Editor

A minimal, lightning-fast, plain-text editor for Markdown files. Built with a Rust core and native macOS UI for developers and writers who want a clean, local-first editing experience without bloat.

## Why RMDE

- **Rust-powered speed** — sub-100ms startup, <16ms keystroke latency
- **Markdown-native** — open, edit, and save `.md` files directly; what you write is what stays on disk
- **Zero cloud lock-in** — plain text files only; no databases, no proprietary formats
- **Local-first & private** — your notes stay local unless you explicitly sync them

## Features (v0.x)

- [x] Open and edit Markdown files
- [x] Multiple tabs
- [x] Native macOS UI (SwiftUI + TextKit 2)
- [ ] Syntax highlighting (headers, code blocks, emphasis, lists)
- [ ] Multi-cursor editing (Cmd+D, Cmd+Click)
- [ ] Undo/redo
- [ ] Search + quick open

## Architecture

```
┌─────────────────────────────────────┐
│     SwiftUI/AppKit Frontend         │
│     (TextKit 2, native menus)       │
└─────────────────┬───────────────────┘
                  │ swift-bridge FFI
┌─────────────────▼───────────────────┐
│         Rust Core (rmde-core)       │
│  • ropey — O(log n) rope buffer     │
│  • tree-sitter — incremental parse  │
│  • Multi-cursor selections          │
└─────────────────────────────────────┘
```

## Build & Run

**Prerequisites:**
- Rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Xcode 15+
- XcodeGen (`brew install xcodegen`)

```bash
# Clone
git clone git@github.com:indrek8/rmde.git
cd rmde

# Build Rust core + generate Xcode project
source $HOME/.cargo/env
make xcode

# Open in Xcode and run (Cmd+R)
open RMDE/RMDE.xcodeproj
```

## Project Structure

```
rmde/
├── Cargo.toml              # Workspace
├── Makefile                # Build commands
├── rmde-core/              # Rust library
│   └── src/
│       ├── lib.rs          # Public API, FFI exports
│       ├── document.rs     # Rope-based document
│       ├── editor.rs       # Tab management
│       ├── selection.rs    # Multi-cursor
│       └── ffi.rs          # swift-bridge bindings
└── RMDE/                   # macOS app
    └── Sources/
        ├── App/            # RMDEApp, EditorState
        ├── Views/          # SwiftUI views
        └── Bridge/         # Generated FFI
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Startup | < 100ms |
| Keystroke latency | < 16ms |
| Memory (empty) | < 20MB |
| File open (1MB) | < 50ms |

## Roadmap

1. **Phase 1** — Foundation (done)
2. **Phase 2** — Core editing, undo/redo, keyboard shortcuts
3. **Phase 3** — Multi-cursor support
4. **Phase 4** — Syntax highlighting via tree-sitter
5. **Phase 5** — Polish, themes, performance tuning
6. **Phase 6** — AI-assisted writing (future)

## License

MIT
