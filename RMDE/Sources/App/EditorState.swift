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

/// Observable state wrapper around the Rust editor core
@MainActor
final class EditorState: ObservableObject {
    private var editor: RMDEEditor

    @Published var content: String = ""
    @Published var cursorPosition: UInt = 0
    @Published var isDirty: Bool = false
    @Published var title: String = "Untitled"
    @Published var tabs: [Tab] = []
    @Published var activeTabId: UInt64 = 0

    init() {
        editor = RMDEEditor()
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

    /// Sync Swift state from Rust editor
    private func syncFromRust() {
        content = editor.get_content().toString()
        cursorPosition = editor.get_cursor_position()
        isDirty = editor.is_dirty()
        title = editor.get_title().toString()
        activeTabId = editor.get_active_tab_id()

        // Initialize tabs if empty (first run)
        if tabs.isEmpty {
            tabs.append(Tab(id: activeTabId, title: title, isDirty: isDirty, hasPath: false, filePath: nil))
        }

        // Update active tab's info
        if let idx = tabs.firstIndex(where: { $0.id == activeTabId }) {
            tabs[idx].title = title
            tabs[idx].isDirty = isDirty
        }
    }

    /// Update content from external source (e.g., NSTextView)
    func updateContent(_ newContent: String, cursorPos: UInt) {
        // Calculate diff and apply to Rust editor
        // For now, simple approach: select all and replace
        editor.select_all()
        editor.insert_text(newContent)
        editor.set_cursor(cursorPos)
        syncFromRust()
    }
}
