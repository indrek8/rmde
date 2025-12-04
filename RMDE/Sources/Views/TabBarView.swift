import SwiftUI

struct TabBarView: View {
    @EnvironmentObject var editorState: EditorState

    var body: some View {
        HStack(spacing: 0) {
            // All tabs
            ForEach(editorState.tabs) { tab in
                TabItemView(
                    title: tab.title,
                    isDirty: tab.isDirty,
                    isActive: tab.id == editorState.activeTabId,
                    onSelect: {
                        editorState.switchTab(id: tab.id)
                    },
                    onClose: {
                        editorState.closeTab(id: tab.id)
                    }
                )
            }

            Spacer()

            // New tab button
            Button(action: { editorState.newTab() }) {
                Image(systemName: "plus")
                    .font(.system(size: 12, weight: .medium))
                    .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)
            .padding(.horizontal, 12)
        }
        .frame(height: 32)
        .background(Color(nsColor: .windowBackgroundColor))
        .overlay(
            Rectangle()
                .frame(height: 1)
                .foregroundColor(Color(nsColor: .separatorColor)),
            alignment: .bottom
        )
    }
}

struct TabItemView: View {
    let title: String
    let isDirty: Bool
    let isActive: Bool
    let onSelect: () -> Void
    let onClose: () -> Void

    @State private var isHovering = false

    var body: some View {
        HStack(spacing: 4) {
            // Dirty indicator
            Circle()
                .fill(isDirty ? Color.orange : Color.clear)
                .frame(width: 8, height: 8)

            Text(title)
                .font(.system(size: 12))
                .lineLimit(1)
                .foregroundColor(isActive ? .primary : .secondary)

            // Close button
            Button(action: onClose) {
                Image(systemName: "xmark")
                    .font(.system(size: 9, weight: .medium))
                    .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)
            .opacity(isHovering ? 1 : 0)
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 6)
        .background(
            RoundedRectangle(cornerRadius: 4)
                .fill(isActive ? Color(nsColor: .controlBackgroundColor) : Color.clear)
        )
        .contentShape(Rectangle())
        .onTapGesture {
            onSelect()
        }
        .onHover { hovering in
            isHovering = hovering
        }
    }
}

#Preview {
    TabBarView()
        .environmentObject(EditorState())
}
