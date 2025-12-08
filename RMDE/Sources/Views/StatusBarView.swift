import SwiftUI

struct StatusBarView: View {
    @EnvironmentObject var editorState: EditorState

    var body: some View {
        HStack {
            // File type
            Text("Markdown")
                .font(.system(size: 11))
                .foregroundColor(.secondary)

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
