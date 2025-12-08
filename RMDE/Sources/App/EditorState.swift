import SwiftUI
import AppKit

/// Tab information for UI
struct Tab: Identifiable, Equatable {
    let id: UInt64
    var title: String
    var isDirty: Bool
    var hasPath: Bool  // true if file has been saved to disk
    var filePath: URL?
}

/// Highlight span from Rust parser
struct HighlightSpan {
    let start: Int
    let end: Int
    let kind: UInt64

    // SpanKind values from Rust parser (must match rmde-core/src/parser.rs)

    // Headings
    static let heading1: UInt64 = 1
    static let heading2: UInt64 = 2
    static let heading3: UInt64 = 3
    static let heading4: UInt64 = 4
    static let heading5: UInt64 = 5
    static let heading6: UInt64 = 6
    static let headingMarker: UInt64 = 7

    // Emphasis
    static let bold: UInt64 = 10
    static let italic: UInt64 = 11
    static let boldItalic: UInt64 = 12
    static let strikethrough: UInt64 = 13

    // Code
    static let codeInline: UInt64 = 20
    static let codeBlock: UInt64 = 21
    static let codeFence: UInt64 = 22
    static let codeLanguage: UInt64 = 23

    // Links and Images
    static let link: UInt64 = 30
    static let linkUrl: UInt64 = 31
    static let linkTitle: UInt64 = 32
    static let image: UInt64 = 33
    static let autolink: UInt64 = 34
    static let autolinkEmail: UInt64 = 35

    // Lists
    static let listMarker: UInt64 = 40
    static let taskMarker: UInt64 = 41
    static let taskChecked: UInt64 = 42

    // Blocks
    static let blockQuote: UInt64 = 50
    static let horizontalRule: UInt64 = 51

    // Tables (GFM)
    static let tableHeader: UInt64 = 60
    static let tableDelimiter: UInt64 = 61
    static let tableCell: UInt64 = 62

    // Generic emphasis (70)
    static let emphasis: UInt64 = 70

    // Extended Syntax (Phase 4)
    static let footnoteRef: UInt64 = 80
    static let footnoteDef: UInt64 = 81
    static let mathInline: UInt64 = 82
    static let mathBlock: UInt64 = 83
    static let highlight: UInt64 = 84

    // LLM Artifacts (Phase 6)
    static let artifactThinking: UInt64 = 90
    static let artifactMeta: UInt64 = 91

    // Markers for ghost mode (100+)
    static let markerHeading: UInt64 = 100
    static let markerBold: UInt64 = 101
    static let markerItalic: UInt64 = 102
    static let markerStrikethrough: UInt64 = 104
    static let markerCode: UInt64 = 105
    static let markerLink: UInt64 = 107
    static let markerImage: UInt64 = 108
    static let markerListBullet: UInt64 = 109
    static let markerListNumber: UInt64 = 110
    static let markerTaskBox: UInt64 = 112
}

/// Observable state wrapper around the Rust editor core
@MainActor
final class EditorState: ObservableObject {
    private var editor: RMDEEditor
    private var parser: RMDEParser
    private var parseTimer: Timer?
    private let parseDebounceInterval: TimeInterval = 0.3  // 300ms debounce

    @Published var contentVersion: Int = 0  // Increments on file open, tab switch
    @Published var highlightVersion: Int = 0  // Increments when highlights change
    @Published var cursorPosition: UInt = 0
    @Published var cursorLine: UInt = 1
    @Published var cursorColumn: UInt = 1
    @Published var lineCount: UInt = 0
    @Published var isDirty: Bool = false
    @Published var title: String = "Untitled"
    @Published var tabs: [Tab] = []
    @Published var activeTabId: UInt64 = 0

    // Highlight spans - not @Published to avoid excessive updates
    private(set) var highlightSpans: [HighlightSpan] = []

    init() {
        editor = RMDEEditor()
        parser = RMDEParser()
        syncFromRust()
    }

    // MARK: - Tab Management

    func newTab() {
        let newId = editor.new_tab()
        tabs.append(Tab(id: newId, title: "Untitled", isDirty: false, hasPath: false, filePath: nil))
        syncFromRust()
    }

    func closeTab(id: UInt64) {
        if editor.close_tab(id) {
            tabs.removeAll { $0.id == id }
            // If all tabs closed, Rust creates a new one - we need to track it
            if tabs.isEmpty {
                let newId = editor.get_active_tab_id()
                tabs.append(Tab(id: newId, title: "Untitled", isDirty: false, hasPath: false, filePath: nil))
            }
        }
        syncFromRust()
    }

    func switchTab(id: UInt64) {
        if editor.switch_tab(id) {
            resetParser()  // Clear cached tree before switching
            syncFromRust()
        }
    }

    func nextTab() {
        editor.next_tab()
        syncFromRust()
    }

    func prevTab() {
        editor.prev_tab()
        syncFromRust()
    }

    // MARK: - File Operations

    func openFile() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText, .text]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false

        if panel.runModal() == .OK, let url = panel.url {
            let error = editor.open_file(url.path)
            let errorStr = error.toString()
            if !errorStr.isEmpty {
                // TODO: Show error alert
                print("Error opening file: \(errorStr)")
            } else {
                // Add new tab for opened file
                let newId = editor.get_active_tab_id()
                let fileName = url.lastPathComponent
                // Only add if not already in tabs (file might already be open)
                if !tabs.contains(where: { $0.id == newId }) {
                    tabs.append(Tab(id: newId, title: fileName, isDirty: false, hasPath: true, filePath: url))
                }
            }
            syncFromRust()
        }
    }

    func save() {
        let error = editor.save_file()
        let errorStr = error.toString()
        if !errorStr.isEmpty {
            if errorStr.contains("No file path") {
                saveAs()
            } else {
                // TODO: Show error alert
                print("Error saving: \(error)")
            }
        } else {
            syncFromRust()
        }
    }

    func saveAs() {
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.plainText]
        panel.nameFieldStringValue = title

        if panel.runModal() == .OK, let url = panel.url {
            let error = editor.save_file_as(url.path)
            if !error.toString().isEmpty {
                // TODO: Show error alert
                print("Error saving: \(error)")
            } else {
                // Update tab with new path
                if let idx = tabs.firstIndex(where: { $0.id == activeTabId }) {
                    tabs[idx].hasPath = true
                    tabs[idx].filePath = url
                    tabs[idx].title = url.lastPathComponent
                }
            }
            syncFromRust()
        }
    }

    func renameCurrentFile(to newName: String) {
        guard let idx = tabs.firstIndex(where: { $0.id == activeTabId }),
              let oldPath = tabs[idx].filePath else { return }

        let newURL = oldPath.deletingLastPathComponent().appendingPathComponent(newName)

        do {
            try FileManager.default.moveItem(at: oldPath, to: newURL)
            // Update Rust's path by saving to new location
            let error = editor.save_file_as(newURL.path)
            if error.toString().isEmpty {
                tabs[idx].filePath = newURL
                tabs[idx].title = newName
            }
            syncFromRust()
        } catch {
            print("Error renaming file: \(error)")
        }
    }

    // MARK: - Text Editing

    func insertText(_ text: String) {
        editor.insert_text(text)
        syncFromRust()
    }

    func deleteBackward() {
        editor.delete_backward()
        syncFromRust()
    }

    func deleteForward() {
        editor.delete_forward()
        syncFromRust()
    }

    func setCursor(_ pos: UInt) {
        editor.set_cursor(pos)
        syncFromRust()
    }

    func addCursor(_ pos: UInt) {
        editor.add_cursor(pos)
        syncFromRust()
    }

    func moveCursors(delta: Int64, extend: Bool) {
        editor.move_cursors(delta, extend)
        syncFromRust()
    }

    func selectAll() {
        editor.select_all()
        syncFromRust()
    }

    // MARK: - Sync with Rust

    /// Apply an incremental edit from NSTextView
    /// Called by EditorView delegate on each text change
    func applyEdit(pos: Int, deleteLen: Int, text: String) {
        editor.apply_edit(UInt(pos), UInt(deleteLen), text)
        syncMetadata()  // Fast - only metadata, not content
        scheduleHighlightUpdate()  // Debounced syntax highlighting
    }

    /// Full content sync - used for recovery when incremental sync gets out of sync
    func fullSync(_ content: String) {
        editor.select_all()
        editor.insert_text(content)
        syncMetadata()  // Fast - only metadata, not content
    }

    /// Get content length for sync verification
    var contentLength: Int {
        Int(editor.get_content_length())
    }

    /// Lightweight sync - only metadata, not content (fast, for every edit)
    private func syncMetadata() {
        cursorPosition = editor.get_cursor_position()
        cursorLine = UInt(editor.get_cursor_line())
        cursorColumn = UInt(editor.get_cursor_column())
        lineCount = UInt(editor.get_line_count())
        isDirty = editor.is_dirty()
        activeTabId = editor.get_active_tab_id()

        // Update active tab's dirty state
        if let idx = tabs.firstIndex(where: { $0.id == activeTabId }) {
            tabs[idx].isDirty = isDirty
        }
    }

    /// Full sync - increments version to trigger NSTextView reload
    private func syncFromRust() {
        contentVersion += 1  // Trigger NSTextView to reload
        title = editor.get_title().toString()
        activeTabId = editor.get_active_tab_id()
        syncMetadata()

        // Initialize tabs if empty (first run)
        if tabs.isEmpty {
            tabs.append(Tab(id: activeTabId, title: title, isDirty: isDirty, hasPath: false, filePath: nil))
        }

        // Update active tab's info
        if let idx = tabs.firstIndex(where: { $0.id == activeTabId }) {
            tabs[idx].title = title
            tabs[idx].isDirty = isDirty
        }

        // Parse immediately on file open/tab switch
        parseContent()
    }

    /// Get content from Rust - only call when actually needed (e.g., loading into NSTextView)
    func getContent() -> String {
        editor.get_content().toString()
    }

    // MARK: - Syntax Highlighting

    /// Schedule debounced parsing after text changes
    func scheduleHighlightUpdate() {
        parseTimer?.invalidate()
        parseTimer = Timer.scheduledTimer(withTimeInterval: parseDebounceInterval, repeats: false) { [weak self] _ in
            Task { @MainActor in
                self?.parseContent()
            }
        }
    }

    /// Parse content immediately (called after debounce or on file open)
    func parseContent() {
        let content = getContent()

        // Skip parsing for very large files (> 1MB) to prevent UI freeze
        guard content.utf8.count < 1_000_000 else {
            highlightSpans = []
            highlightVersion += 1
            return
        }

        // Get packed spans from Rust: [start, end, kind, start, end, kind, ...]
        let packed = parser.parse(content)
        var spans: [HighlightSpan] = []
        spans.reserveCapacity(Int(packed.len()) / 3)

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
        highlightVersion += 1
    }

    /// Reset parser state (call on tab switch)
    func resetParser() {
        parser.reset_parser()
        highlightSpans = []
    }

}
