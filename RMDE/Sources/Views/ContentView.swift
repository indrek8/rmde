import SwiftUI

struct ContentView: View {
    @EnvironmentObject var editorState: EditorState

    var body: some View {
        VStack(spacing: 0) {
            // Tab bar
            TabBarView()

            // Find panel (conditionally shown)
            if editorState.findState.isVisible {
                FindPanelView()
            }

            // Editor
            EditorView()
                .frame(maxWidth: .infinity, maxHeight: .infinity)

            // Status bar
            StatusBarView()
        }
        .frame(minWidth: 600, minHeight: 400)
        .onKeyPress(.escape) { _ in
            if editorState.findState.isVisible {
                editorState.findState.isVisible = false
                editorState.findState.reset()
                return .handled
            }
            return .ignored
        }
    }
}

#Preview {
    ContentView()
        .environmentObject(EditorState())
}
