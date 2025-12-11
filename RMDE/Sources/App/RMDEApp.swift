import SwiftUI

@main
struct RMDEApp: App {
    @StateObject private var editorState = EditorState()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(editorState)
        }
        .commands {
            CommandGroup(replacing: .newItem) {
                Button("New Tab") {
                    editorState.newTab()
                }
                .keyboardShortcut("t", modifiers: .command)

                Button("Open...") {
                    editorState.openFile()
                }
                .keyboardShortcut("o", modifiers: .command)
            }

            CommandGroup(replacing: .saveItem) {
                Button("Save") {
                    editorState.save()
                }
                .keyboardShortcut("s", modifiers: .command)

                Button("Save As...") {
                    editorState.saveAs()
                }
                .keyboardShortcut("s", modifiers: [.command, .shift])
            }

            // Note: Undo/Redo handled natively by NSTextView (NSUndoManager)

            CommandGroup(after: .textEditing) {
                Divider()

                Button("Find...") {
                    editorState.findState.isVisible = true
                    editorState.findState.showReplace = false
                }
                .keyboardShortcut("f", modifiers: .command)

                Button("Find Next") {
                    if !editorState.findState.isVisible {
                        editorState.findState.isVisible = true
                    }
                    editorState.findNext()
                }
                .keyboardShortcut("g", modifiers: .command)

                Button("Find Previous") {
                    if !editorState.findState.isVisible {
                        editorState.findState.isVisible = true
                    }
                    editorState.findPrevious()
                }
                .keyboardShortcut("g", modifiers: [.command, .shift])

                Button("Find and Replace...") {
                    editorState.findState.isVisible = true
                    editorState.findState.showReplace = true
                }
                .keyboardShortcut("f", modifiers: [.command, .option])
            }

            CommandGroup(after: .windowArrangement) {
                Button("Next Tab") {
                    editorState.nextTab()
                }
                .keyboardShortcut("]", modifiers: [.command, .shift])

                Button("Previous Tab") {
                    editorState.prevTab()
                }
                .keyboardShortcut("[", modifiers: [.command, .shift])
            }

            CommandGroup(after: .toolbar) {
                Toggle("Ghost Mode", isOn: $editorState.ghostMode)
                    .keyboardShortcut("g", modifiers: [.command, .shift])
            }
        }
    }
}
