import SwiftUI

struct ContentView: View {
    @EnvironmentObject var editorState: EditorState

    var body: some View {
        VStack(spacing: 0) {
            // Tab bar
            TabBarView()

            // Editor
            EditorView()
                .frame(maxWidth: .infinity, maxHeight: .infinity)

            // Status bar
            StatusBarView()
        }
        .frame(minWidth: 600, minHeight: 400)
    }
}

#Preview {
    ContentView()
        .environmentObject(EditorState())
}
