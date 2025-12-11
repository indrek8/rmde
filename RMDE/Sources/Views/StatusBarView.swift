import SwiftUI

struct StatusBarView: View {
    @EnvironmentObject var editorState: EditorState

    var body: some View {
        HStack {
            // File type
            Text("Markdown")
                .font(.system(size: 11))
                .foregroundColor(.secondary)

            Divider()
                .frame(height: 12)

            // Ghost mode toggle
            Button(action: {
                editorState.ghostMode.toggle()
            }) {
                HStack(spacing: 4) {
                    Image(systemName: editorState.ghostMode ? "eye.slash" : "eye")
                        .font(.system(size: 10))
                    Text(editorState.ghostMode ? "Ghost" : "Markup")
                        .font(.system(size: 11))
                }
                .foregroundColor(editorState.ghostMode ? .accentColor : .secondary)
            }
            .buttonStyle(.plain)
            .help(editorState.ghostMode ? "Show markdown syntax (⇧⌘G)" : "Hide markdown syntax (⇧⌘G)")

            Divider()
                .frame(height: 12)

            // Theme toggle
            Button(action: {
                editorState.toggleTheme()
            }) {
                HStack(spacing: 4) {
                    Image(systemName: editorState.themeManager.currentMode.iconName)
                        .font(.system(size: 10))
                    Text(editorState.themeManager.currentMode.label)
                        .font(.system(size: 11))
                }
                .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)
            .help("Toggle theme: \(editorState.themeManager.currentMode.label) (⇧⌘T)")

            Spacer()

            // Line:Column
            Text("Ln \(editorState.cursorLine), Col \(editorState.cursorColumn)")
                .font(.system(size: 11, design: .monospaced))
                .foregroundColor(.secondary)

            Divider()
                .frame(height: 12)

            // Line count
            Text("\(editorState.lineCount) lines")
                .font(.system(size: 11, design: .monospaced))
                .foregroundColor(.secondary)

            Divider()
                .frame(height: 12)

            // Character count (from Rust - no string copy)
            Text("\(editorState.contentLength) chars")
                .font(.system(size: 11, design: .monospaced))
                .foregroundColor(.secondary)
        }
        .padding(.horizontal, 12)
        .frame(height: 22)
        .background(Color(nsColor: .windowBackgroundColor))
        .overlay(
            Rectangle()
                .frame(height: 1)
                .foregroundColor(Color(nsColor: .separatorColor)),
            alignment: .top
        )
    }
}

#Preview {
    StatusBarView()
        .environmentObject(EditorState())
}
