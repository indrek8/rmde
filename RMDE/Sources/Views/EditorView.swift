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
        textView.allowsUndo = false  // We handle undo in Rust
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

        return scrollView
    }

    func updateNSView(_ scrollView: NSScrollView, context: Context) {
        guard let textView = scrollView.documentView as? RMDETextView else { return }

        // Only update if content changed externally (e.g., file opened)
        if textView.string != editorState.content {
            let selectedRange = textView.selectedRange()
            textView.string = editorState.content
            textView.setSelectedRange(selectedRange)
        }
    }

    func makeCoordinator() -> Coordinator {
        Coordinator()
    }

    class Coordinator: NSObject, NSTextViewDelegate {
        func textDidChange(_ notification: Notification) {
            guard let textView = notification.object as? RMDETextView,
                  let editorState = textView.editorState else { return }

            let cursorPos = UInt(textView.selectedRange().location)
            editorState.updateContent(textView.string, cursorPos: cursorPos)
        }

        func textView(_ textView: NSTextView, shouldChangeTextIn range: NSRange, replacementString text: String?) -> Bool {
            // Let the change happen - we'll sync after
            return true
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
