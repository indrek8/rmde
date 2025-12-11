import SwiftUI
import AppKit

/// Theme mode options for the application
enum ThemeMode: String, CaseIterable {
    case system = "system"
    case light = "light"
    case dark = "dark"

    /// User-facing label for menu items
    var label: String {
        switch self {
        case .system: return "System"
        case .light: return "Light"
        case .dark: return "Dark"
        }
    }

    /// SF Symbol icon name for status bar
    var iconName: String {
        switch self {
        case .system: return "circle.lefthalf.filled"
        case .light: return "sun.max"
        case .dark: return "moon"
        }
    }

    /// Apply this theme to the application
    func apply() {
        switch self {
        case .system:
            NSApplication.shared.appearance = nil
        case .light:
            NSApplication.shared.appearance = NSAppearance(named: .aqua)
        case .dark:
            NSApplication.shared.appearance = NSAppearance(named: .darkAqua)
        }
    }

    /// Get the next theme mode in the cycle (System -> Light -> Dark -> System)
    func next() -> ThemeMode {
        switch self {
        case .system: return .light
        case .light: return .dark
        case .dark: return .system
        }
    }
}

/// Manager for persisting and applying theme settings
@MainActor
class ThemeManager: ObservableObject {
    @AppStorage("themeMode") private var savedThemeMode: String = ThemeMode.system.rawValue

    @Published var currentMode: ThemeMode {
        didSet {
            savedThemeMode = currentMode.rawValue
            currentMode.apply()
        }
    }

    init() {
        // Load saved theme mode or default to system
        if let mode = ThemeMode(rawValue: savedThemeMode) {
            currentMode = mode
        } else {
            currentMode = .system
        }
        // Apply the theme on initialization
        currentMode.apply()
    }

    /// Cycle to the next theme mode
    func toggleTheme() {
        currentMode = currentMode.next()
    }
}
