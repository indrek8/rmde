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

            // Cursor position
            Text("Pos \(editorState.cursorPosition)")
                .font(.system(size: 11, design: .monospaced))
                .foregroundColor(.secondary)

            Divider()
                .frame(height: 12)

            // Character count
            Text("\(editorState.content.count) chars")
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
