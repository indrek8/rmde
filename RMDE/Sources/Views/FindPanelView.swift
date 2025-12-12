import SwiftUI

struct FindPanelView: View {
    @EnvironmentObject var editorState: EditorState
    @FocusState private var searchFieldFocused: Bool

    var body: some View {
        VStack(spacing: 0) {
            // Find row
            HStack(spacing: 8) {
                // Search icon
                Image(systemName: "magnifyingglass")
                    .font(.system(size: 12))
                    .foregroundColor(.secondary)

                // Search field
                TextField("Find", text: $editorState.findState.searchText)
                    .textFieldStyle(.plain)
                    .font(.system(size: 12))
                    .focused($searchFieldFocused)
                    .onChange(of: editorState.findState.searchText) { _, newValue in
                        editorState.performFind(newValue)
                    }
                    .onSubmit {
                        editorState.findNext()
                    }

                // Clear button
                if !editorState.findState.searchText.isEmpty {
                    Button(action: {
                        editorState.findState.searchText = ""
                        editorState.findState.matchRanges = []
                    }) {
                        Image(systemName: "xmark.circle.fill")
                            .font(.system(size: 12))
                            .foregroundColor(.secondary)
                    }
                    .buttonStyle(.plain)
                }

                Divider()
                    .frame(height: 16)

                // Previous button
                Button(action: {
                    editorState.findPrevious()
                }) {
                    Image(systemName: "chevron.left")
                        .font(.system(size: 11))
                }
                .buttonStyle(.plain)
                .disabled(editorState.findState.matchCount == 0)
                .help("Previous match (⇧⌘G)")

                // Next button
                Button(action: {
                    editorState.findNext()
                }) {
                    Image(systemName: "chevron.right")
                        .font(.system(size: 11))
                }
                .buttonStyle(.plain)
                .disabled(editorState.findState.matchCount == 0)
                .help("Next match (⌘G)")

                Divider()
                    .frame(height: 16)

                // Match counter
                Text(matchCountText)
                    .font(.system(size: 11, design: .monospaced))
                    .foregroundColor(.secondary)
                    .frame(minWidth: 50)

                Divider()
                    .frame(height: 16)

                // Case sensitive toggle
                Button(action: {
                    editorState.findState.caseSensitive.toggle()
                    editorState.performFind(editorState.findState.searchText)
                }) {
                    Text("Aa")
                        .font(.system(size: 11, weight: editorState.findState.caseSensitive ? .semibold : .regular))
                        .foregroundColor(editorState.findState.caseSensitive ? .accentColor : .secondary)
                }
                .buttonStyle(.plain)
                .help(editorState.findState.caseSensitive ? "Case sensitive" : "Case insensitive")

                Divider()
                    .frame(height: 16)

                // Expand/collapse replace button
                Button(action: {
                    editorState.findState.showReplace.toggle()
                }) {
                    Image(systemName: editorState.findState.showReplace ? "chevron.up" : "chevron.down")
                        .font(.system(size: 11))
                }
                .buttonStyle(.plain)
                .help(editorState.findState.showReplace ? "Hide replace" : "Show replace")

                Divider()
                    .frame(height: 16)

                // Close button
                Button(action: {
                    editorState.findState.isVisible = false
                    editorState.findState.reset()
                }) {
                    Image(systemName: "xmark")
                        .font(.system(size: 11))
                }
                .buttonStyle(.plain)
                .help("Close (Escape)")
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 6)
            .frame(height: 32)

            // Replace row (expandable)
            if editorState.findState.showReplace {
                Divider()

                HStack(spacing: 8) {
                    // Replace icon
                    Image(systemName: "arrow.triangle.2.circlepath")
                        .font(.system(size: 12))
                        .foregroundColor(.secondary)

                    // Replace field
                    TextField("Replace", text: $editorState.findState.replaceText)
                        .textFieldStyle(.plain)
                        .font(.system(size: 12))
                        .onSubmit {
                            editorState.replaceCurrentMatch()
                        }

                    Divider()
                        .frame(height: 16)

                    // Replace button
                    Button("Replace") {
                        editorState.replaceCurrentMatch()
                    }
                    .buttonStyle(.plain)
                    .font(.system(size: 11))
                    .disabled(editorState.findState.matchCount == 0)

                    // Replace All button
                    Button("Replace All") {
                        editorState.replaceAllMatches()
                    }
                    .buttonStyle(.plain)
                    .font(.system(size: 11))
                    .disabled(editorState.findState.matchCount == 0)
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 6)
                .frame(height: 32)
            }
        }
        .background(Color(nsColor: .windowBackgroundColor))
        .overlay(
            Rectangle()
                .frame(height: 1)
                .foregroundColor(Color(nsColor: .separatorColor)),
            alignment: .bottom
        )
        .onAppear {
            searchFieldFocused = true
        }
    }

    private var matchCountText: String {
        let count = editorState.findState.matchCount
        if count == 0 {
            return "No results"
        } else {
            let current = editorState.findState.currentMatchIndex + 1
            return "\(current) of \(count)"
        }
    }
}

#Preview {
    FindPanelView()
        .environmentObject(EditorState())
}
