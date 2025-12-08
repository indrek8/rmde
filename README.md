# RMDE — Rust Markdown Editor

A minimal, lightning-fast, plain-text editor for Markdown files. Built with a Rust core and native macOS UI for developers and writers who want a clean, local-first editing experience without bloat.

## Why RMDE

- **Rust-powered speed** — sub-100ms startup, <16ms keystroke latency
- **Markdown-native** — open, edit, and save `.md` files directly
- **Zero cloud lock-in** — plain text files only, no proprietary formats
- **Local-first** — your notes stay local unless you explicitly sync them

## Features

- [x] Open and edit Markdown files
- [x] Multiple tabs
- [x] Native macOS UI (SwiftUI + TextKit 2)
- [ ] Syntax highlighting
- [ ] Multi-cursor editing
- [ ] Undo/redo

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

## Project Structure

```
rmde/
├── rmde-core/              # Rust library
│   └── src/
│       ├── lib.rs          # FFI exports
│       ├── document.rs     # Rope buffer
│       ├── editor.rs       # Tab management
│       └── selection.rs    # Multi-cursor
└── RMDE/                   # macOS app
    └── Sources/
        ├── App/            # EditorState
        ├── Views/          # SwiftUI views
        └── Bridge/         # Generated FFI
```

## License

MIT
