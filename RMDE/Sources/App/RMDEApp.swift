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

                Button("Toggle Theme (\(editorState.themeManager.currentMode.label))") {
                    editorState.toggleTheme()
                }
                .keyboardShortcut("t", modifiers: [.command, .shift])
            }
        }
    }
}
