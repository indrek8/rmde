import SwiftUI

/// State management for find & replace functionality
@MainActor
class FindState: ObservableObject {
    @Published var isVisible = false
    @Published var showReplace = false
    @Published var searchText = ""
    @Published var replaceText = ""
    @Published var matchRanges: [NSRange] = []
    @Published var currentMatchIndex = 0
    @Published var caseSensitive = true

    var matchCount: Int { matchRanges.count }

    /// Current match range, or nil if no matches
    var currentMatch: NSRange? {
        guard currentMatchIndex >= 0 && currentMatchIndex < matchRanges.count else {
            return nil
        }
        return matchRanges[currentMatchIndex]
    }

    /// Reset state when closing the panel
    func reset() {
        searchText = ""
        replaceText = ""
        matchRanges = []
        currentMatchIndex = 0
        showReplace = false
    }

    /// Move to next match
    func nextMatch() {
        guard matchCount > 0 else { return }
        currentMatchIndex = (currentMatchIndex + 1) % matchCount
    }

    /// Move to previous match
    func previousMatch() {
        guard matchCount > 0 else { return }
        currentMatchIndex = (currentMatchIndex - 1 + matchCount) % matchCount
    }
}
