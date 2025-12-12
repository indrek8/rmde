import SwiftUI
import AppKit

/// SwiftUI wrapper for NSTextView using TextKit 2
struct EditorView: NSViewRepresentable {
    @EnvironmentObject var editorState: EditorState

    func makeNSView(context: Context) -> NSScrollView {
        let scrollView = NSScrollView()
        let textView = RMDETextView()

        // Configure scroll view
        scrollView.hasVerticalScroller = true
        scrollView.hasHorizontalScroller = false
        scrollView.autohidesScrollers = true
        scrollView.borderType = .noBorder

        // Configure text view
        textView.minSize = NSSize(width: 0, height: 0)
        textView.maxSize = NSSize(width: CGFloat.greatestFiniteMagnitude, height: CGFloat.greatestFiniteMagnitude)
        textView.isVerticallyResizable = true
        textView.isHorizontallyResizable = false
        textView.autoresizingMask = [.width]
        textView.textContainer?.containerSize = NSSize(
            width: scrollView.contentSize.width,
            height: CGFloat.greatestFiniteMagnitude
        )
        textView.textContainer?.widthTracksTextView = true

        // Editor appearance
        textView.backgroundColor = NSColor.textBackgroundColor
        textView.isEditable = true
        textView.isSelectable = true
        textView.allowsUndo = true  // Native NSUndoManager handles undo/redo
        textView.isRichText = false
        textView.font = NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)
        textView.textColor = NSColor.textColor

        // Line spacing
        let paragraphStyle = NSMutableParagraphStyle()
        paragraphStyle.lineSpacing = 4
        textView.defaultParagraphStyle = paragraphStyle

        // Store reference for delegate
        textView.editorState = editorState
        textView.delegate = context.coordinator

        scrollView.documentView = textView

        // Store textView reference in editorState for find/replace
        editorState.textView = textView

        return scrollView
    }

    func updateNSView(_ scrollView: NSScrollView, context: Context) {
        guard let textView = scrollView.documentView as? RMDETextView else { return }

        // Only reload content when version changes (file open, tab switch)
        if context.coordinator.loadedVersion != editorState.contentVersion {
            context.coordinator.loadedVersion = editorState.contentVersion
            context.coordinator.highlightVersion = -1  // Force highlight refresh
            let content = editorState.getContent()  // Single copy, only when needed
            textView.string = content
            textView.setSelectedRange(NSRange(location: 0, length: 0))
        }

        // Apply highlights when highlight version changes OR ghost mode changes
        if context.coordinator.highlightVersion != editorState.highlightVersion ||
           context.coordinator.ghostMode != editorState.ghostMode {
            context.coordinator.highlightVersion = editorState.highlightVersion
            context.coordinator.ghostMode = editorState.ghostMode
            applyHighlights(to: textView)
        }
    }

    private func applyHighlights(to textView: NSTextView) {
        guard let textStorage = textView.textStorage else { return }

        let fullRange = NSRange(location: 0, length: textStorage.length)

        // Disable layout updates during attribute changes
        textView.layoutManager?.ensureLayout(for: textView.textContainer!)

        // Begin editing
        textStorage.beginEditing()

        // Reset to default style
        let defaultFont = NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)
        textStorage.setAttributes([
            .font: defaultFont,
            .foregroundColor: NSColor.textColor
        ], range: fullRange)

        // Apply each highlight span
        for span in editorState.highlightSpans {
            let range = NSRange(location: span.start, length: span.end - span.start)
            guard range.location >= 0, range.location + range.length <= textStorage.length else { continue }

            let attrs = attributesForKind(span.kind, ghostMode: editorState.ghostMode)
            textStorage.addAttributes(attrs, range: range)
        }

        textStorage.endEditing()
    }

    private func attributesForKind(_ kind: UInt64, ghostMode: Bool) -> [NSAttributedString.Key: Any] {
        let baseFont = NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)

        switch kind {
        // Headings
        case HighlightSpan.heading1:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 24, weight: .bold),
                .foregroundColor: NSColor.labelColor
            ]
        case HighlightSpan.heading2:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 20, weight: .bold),
                .foregroundColor: NSColor.labelColor
            ]
        case HighlightSpan.heading3:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 18, weight: .semibold),
                .foregroundColor: NSColor.labelColor
            ]
        case HighlightSpan.heading4, HighlightSpan.heading5, HighlightSpan.heading6:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 16, weight: .semibold),
                .foregroundColor: NSColor.labelColor
            ]
        case HighlightSpan.headingMarker:
            return [
                .foregroundColor: NSColor.secondaryLabelColor
            ]

        // Emphasis
        case HighlightSpan.bold:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 14, weight: .bold)
            ]
        case HighlightSpan.italic:
            let descriptor = baseFont.fontDescriptor.withSymbolicTraits(.italic)
            let italicFont = NSFont(descriptor: descriptor, size: 14) ?? baseFont
            return [.font: italicFont]
        case HighlightSpan.boldItalic:
            let descriptor = baseFont.fontDescriptor.withSymbolicTraits([.italic, .bold])
            let boldItalicFont = NSFont(descriptor: descriptor, size: 14) ?? baseFont
            return [.font: boldItalicFont]
        case HighlightSpan.strikethrough:
            return [
                .strikethroughStyle: NSUnderlineStyle.single.rawValue,
                .strikethroughColor: NSColor.secondaryLabelColor
            ]

        // Code
        case HighlightSpan.codeInline:
            return [
                .font: baseFont,
                .foregroundColor: NSColor.systemPink,
                .backgroundColor: NSColor.quaternaryLabelColor
            ]
        case HighlightSpan.codeBlock, HighlightSpan.codeFence:
            return [
                .font: baseFont,
                .foregroundColor: NSColor.systemGreen,
                .backgroundColor: NSColor.quaternaryLabelColor
            ]
        case HighlightSpan.codeLanguage:
            return [
                .foregroundColor: NSColor.systemOrange
            ]

        // Links and Images
        case HighlightSpan.link:
            return [
                .foregroundColor: NSColor.linkColor,
                .underlineStyle: NSUnderlineStyle.single.rawValue
            ]
        case HighlightSpan.linkUrl:
            return [
                .foregroundColor: NSColor.secondaryLabelColor
            ]
        case HighlightSpan.linkTitle:
            return [
                .foregroundColor: NSColor.tertiaryLabelColor
            ]
        case HighlightSpan.image:
            return [
                .foregroundColor: NSColor.systemPurple
            ]
        case HighlightSpan.autolink, HighlightSpan.autolinkEmail:
            return [
                .foregroundColor: NSColor.linkColor,
                .underlineStyle: NSUnderlineStyle.single.rawValue
            ]

        // Lists
        case HighlightSpan.listMarker:
            return [
                .foregroundColor: NSColor.systemBlue
            ]
        case HighlightSpan.taskMarker:
            return [
                .foregroundColor: NSColor.systemGray
            ]
        case HighlightSpan.taskChecked:
            return [
                .foregroundColor: NSColor.systemGreen,
                .font: NSFont.monospacedSystemFont(ofSize: 14, weight: .semibold)
            ]

        // Blocks
        case HighlightSpan.blockQuote:
            return [
                .foregroundColor: NSColor.systemGray
            ]
        case HighlightSpan.horizontalRule:
            return [
                .foregroundColor: NSColor.separatorColor
            ]

        // Tables (GFM)
        case HighlightSpan.tableHeader:
            return [
                .font: NSFont.monospacedSystemFont(ofSize: 14, weight: .semibold),
                .foregroundColor: NSColor.systemBlue
            ]
        case HighlightSpan.tableDelimiter:
            return [
                .foregroundColor: NSColor.systemGray
            ]
        case HighlightSpan.tableCell:
            return [
                .foregroundColor: NSColor.labelColor
            ]

        // Generic emphasis (70)
        case HighlightSpan.emphasis:
            let descriptor = baseFont.fontDescriptor.withSymbolicTraits(.italic)
            let italicFont = NSFont(descriptor: descriptor, size: 14) ?? baseFont
            return [.font: italicFont]

        // Extended Syntax (Phase 4)
        case HighlightSpan.footnoteRef, HighlightSpan.footnoteDef:
            return [
                .foregroundColor: NSColor.systemIndigo,
                .font: NSFont.monospacedSystemFont(ofSize: 12, weight: .regular)
            ]
        case HighlightSpan.mathInline, HighlightSpan.mathBlock:
            return [
                .foregroundColor: NSColor.systemPurple,
                .font: NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)
            ]
        case HighlightSpan.highlight:
            return [
                .backgroundColor: NSColor.systemYellow.withAlphaComponent(0.3)
            ]

        // LLM Artifacts (Phase 6)
        case HighlightSpan.artifactThinking:
            return [
                .foregroundColor: NSColor.systemTeal.withAlphaComponent(0.7),
                .font: NSFont.monospacedSystemFont(ofSize: 13, weight: .light)
            ]
        case HighlightSpan.artifactMeta:
            return [
                .foregroundColor: NSColor.systemBrown.withAlphaComponent(0.7),
                .font: NSFont.monospacedSystemFont(ofSize: 13, weight: .light)
            ]

        // Markers for ghost mode (100+)
        // When ghostMode is enabled, markers are hidden (clear color)
        // When ghostMode is disabled, markers are shown in gray
        case HighlightSpan.markerHeading,
             HighlightSpan.markerBold,
             HighlightSpan.markerItalic,
             HighlightSpan.markerStrikethrough,
             HighlightSpan.markerCode,
             HighlightSpan.markerLink,
             HighlightSpan.markerImage,
             HighlightSpan.markerListBullet,
             HighlightSpan.markerListNumber,
             HighlightSpan.markerTaskBox:
            if ghostMode {
                return [.foregroundColor: NSColor.clear]  // Hide markers in ghost mode
            } else {
                return [.foregroundColor: NSColor.systemGray]  // Show markers normally
            }

        default:
            return [:]
        }
    }

    func makeCoordinator() -> Coordinator {
        Coordinator()
    }

    class Coordinator: NSObject, NSTextViewDelegate {
        var loadedVersion: Int = -1  // Track which content version is loaded
        var highlightVersion: Int = -1  // Track which highlight version is applied
        var ghostMode: Bool = false  // Track ghost mode state

        func textView(_ textView: NSTextView, shouldChangeTextIn range: NSRange, replacementString text: String?) -> Bool {
            guard let rmdeTextView = textView as? RMDETextView,
                  let editorState = rmdeTextView.editorState else { return true }

            // Send incremental edit to Rust before the change happens
            editorState.applyEdit(
                pos: range.location,
                deleteLen: range.length,
                text: text ?? ""
            )
            return true
        }

        func textDidChange(_ notification: Notification) {
            guard let textView = notification.object as? RMDETextView,
                  let editorState = textView.editorState else { return }

            // Verify sync - if lengths don't match, do full resync
            let swiftLen = textView.string.utf8.count
            let rustLen = editorState.contentLength

            if swiftLen != rustLen {
                editorState.fullSync(textView.string)
            }
        }
    }
}

/// Custom NSTextView subclass for RMDE
class RMDETextView: NSTextView {
    weak var editorState: EditorState?

    override func keyDown(with event: NSEvent) {
        // Handle special key combinations
        if event.modifierFlags.contains(.command) {
            switch event.charactersIgnoringModifiers {
            case "d":
                // Cmd+D: Select next occurrence (future feature)
                return
            default:
                break
            }
        }

        super.keyDown(with: event)
    }

    override func mouseDown(with event: NSEvent) {
        if event.modifierFlags.contains(.command) {
            // Cmd+Click: Add cursor (future feature)
            let point = convert(event.locationInWindow, from: nil)
            let charIndex = characterIndexForInsertion(at: point)
            editorState?.addCursor(UInt(charIndex))
            return
        }

        super.mouseDown(with: event)
    }
}
