import SwiftUI

struct TabBarView: View {
    @EnvironmentObject var editorState: EditorState
    @State private var renamingTabId: UInt64? = nil
    @State private var renameText: String = ""

    var body: some View {
        HStack(spacing: 0) {
            // All tabs
            ForEach(editorState.tabs) { tab in
                TabItemView(
                    tab: tab,
                    isActive: tab.id == editorState.activeTabId,
                    isRenaming: renamingTabId == tab.id,
                    renameText: $renameText,
                    onSelect: {
                        editorState.switchTab(id: tab.id)
                    },
                    onClose: {
                        editorState.closeTab(id: tab.id)
                    },
                    onDoubleClick: {
                        handleDoubleClick(tab: tab)
                    },
                    onRenameSubmit: {
                        submitRename(tab: tab)
                    },
                    onRenameCancel: {
                        renamingTabId = nil
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

    private func handleDoubleClick(tab: Tab) {
        editorState.switchTab(id: tab.id)

        if tab.hasPath {
            // File is saved — start inline rename
            renameText = tab.title
            renamingTabId = tab.id
        } else {
            // File not saved — trigger Save As
            editorState.saveAs()
        }
    }

    private func submitRename(tab: Tab) {
        let newName = renameText.trimmingCharacters(in: .whitespacesAndNewlines)
        if !newName.isEmpty && newName != tab.title {
            editorState.renameCurrentFile(to: newName)
        }
        renamingTabId = nil
    }
}

struct TabItemView: View {
    let tab: Tab
    let isActive: Bool
    let isRenaming: Bool
    @Binding var renameText: String
    let onSelect: () -> Void
    let onClose: () -> Void
    let onDoubleClick: () -> Void
    let onRenameSubmit: () -> Void
    let onRenameCancel: () -> Void

    @State private var isHovering = false
    @FocusState private var isRenameFocused: Bool

    var body: some View {
        HStack(spacing: 4) {
            // Dirty indicator
            Circle()
                .fill(tab.isDirty ? Color.orange : Color.clear)
                .frame(width: 8, height: 8)

            if isRenaming {
                // Inline rename text field
                TextField("", text: $renameText)
                    .textFieldStyle(.plain)
                    .font(.system(size: 12))
                    .frame(minWidth: 60, maxWidth: 150)
                    .focused($isRenameFocused)
                    .onSubmit {
                        onRenameSubmit()
                    }
                    .onExitCommand {
                        onRenameCancel()
                    }
                    .onAppear {
                        isRenameFocused = true
                    }
            } else {
                Text(tab.title)
                    .font(.system(size: 12))
                    .lineLimit(1)
                    .foregroundColor(isActive ? .primary : .secondary)
            }

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
        .onTapGesture(count: 2) {
            onDoubleClick()
        }
        .onTapGesture(count: 1) {
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
