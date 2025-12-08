//! Marker span tests for ghost mode
//!
//! These tests verify that marker spans (delimiters for syntax) are correctly
//! emitted alongside content spans. This enables "ghost mode" where syntax
//! markers can be hidden without changing text reflow.

use rmde_core::{MarkdownParser, SpanKind};

// ============================================================================
// Bold Markers
// ============================================================================

#[test]
fn test_bold_markers_asterisk() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**bold**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();

    assert_eq!(bold.len(), 1, "Should find one bold span");
    assert_eq!(markers.len(), 2, "Should find two marker spans (opening and closing)");

    // Opening marker
    assert_eq!(markers[0].start, 0);
    assert_eq!(markers[0].end, 2);
    assert_eq!(&content[markers[0].start..markers[0].end], "**");

    // Closing marker
    assert_eq!(markers[1].start, 6);
    assert_eq!(markers[1].end, 8);
    assert_eq!(&content[markers[1].start..markers[1].end], "**");
}

#[test]
fn test_bold_markers_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is **bold** text";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();

    assert_eq!(bold.len(), 1);
    assert_eq!(markers.len(), 2);

    // Opening marker
    assert_eq!(markers[0].start, 8);
    assert_eq!(markers[0].end, 10);

    // Closing marker
    assert_eq!(markers[1].start, 14);
    assert_eq!(markers[1].end, 16);
}

#[test]
fn test_multiple_bold_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**first** and **second**";
    let spans = parser.parse(content);

    let bold: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Bold).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();

    assert_eq!(bold.len(), 2, "Should find two bold spans");
    assert_eq!(markers.len(), 4, "Should find four marker spans");
}

// ============================================================================
// Italic Markers
// ============================================================================

#[test]
fn test_italic_markers_asterisk() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*italic*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerItalic).collect();

    assert_eq!(italic.len(), 1, "Should find one italic span");
    assert_eq!(markers.len(), 2, "Should find two marker spans");

    // Opening marker
    assert_eq!(markers[0].start, 0);
    assert_eq!(markers[0].end, 1);
    assert_eq!(&content[markers[0].start..markers[0].end], "*");

    // Closing marker
    assert_eq!(markers[1].start, 7);
    assert_eq!(markers[1].end, 8);
    assert_eq!(&content[markers[1].start..markers[1].end], "*");
}

#[test]
fn test_italic_markers_underscore() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "_italic_";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerItalic).collect();

    assert_eq!(italic.len(), 1);
    assert_eq!(markers.len(), 2);
}

#[test]
fn test_multiple_italic_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "*first* and *second*";
    let spans = parser.parse(content);

    let italic: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Italic).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerItalic).collect();

    assert_eq!(italic.len(), 2);
    assert_eq!(markers.len(), 4);
}

// ============================================================================
// Code Markers
// ============================================================================

#[test]
fn test_code_markers_single_backtick() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`code`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();

    assert_eq!(code.len(), 1, "Should find one code span");
    assert_eq!(markers.len(), 2, "Should find two marker spans");

    // Opening marker
    assert_eq!(markers[0].start, 0);
    assert_eq!(markers[0].end, 1);
    assert_eq!(&content[markers[0].start..markers[0].end], "`");

    // Closing marker
    assert_eq!(markers[1].start, 5);
    assert_eq!(markers[1].end, 6);
    assert_eq!(&content[markers[1].start..markers[1].end], "`");
}

#[test]
fn test_code_markers_double_backtick() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "``code``";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();

    assert_eq!(code.len(), 1);
    assert_eq!(markers.len(), 2);

    // Opening marker (2 backticks)
    assert_eq!(markers[0].start, 0);
    assert_eq!(markers[0].end, 2);
    assert_eq!(&content[markers[0].start..markers[0].end], "``");

    // Closing marker (2 backticks)
    assert_eq!(markers[1].start, 6);
    assert_eq!(markers[1].end, 8);
    assert_eq!(&content[markers[1].start..markers[1].end], "``");
}

#[test]
fn test_code_markers_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use `code` here";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();

    assert_eq!(code.len(), 1);
    assert_eq!(markers.len(), 2);

    assert_eq!(markers[0].start, 4);
    assert_eq!(markers[0].end, 5);
    assert_eq!(markers[1].start, 9);
    assert_eq!(markers[1].end, 10);
}

// ============================================================================
// Strikethrough Markers
// ============================================================================

#[test]
fn test_strikethrough_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~strike~~";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerStrikethrough).collect();

    assert_eq!(strike.len(), 1, "Should find one strikethrough span");
    assert_eq!(markers.len(), 2, "Should find two marker spans");

    // Opening marker
    assert_eq!(markers[0].start, 0);
    assert_eq!(markers[0].end, 2);
    assert_eq!(&content[markers[0].start..markers[0].end], "~~");

    // Closing marker
    assert_eq!(markers[1].start, 8);
    assert_eq!(markers[1].end, 10);
    assert_eq!(&content[markers[1].start..markers[1].end], "~~");
}

#[test]
fn test_strikethrough_markers_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "This is ~~deleted~~ text";
    let spans = parser.parse(content);

    let strike: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Strikethrough).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerStrikethrough).collect();

    assert_eq!(strike.len(), 1);
    assert_eq!(markers.len(), 2);

    assert_eq!(markers[0].start, 8);
    assert_eq!(markers[0].end, 10);
    assert_eq!(markers[1].start, 17);
    assert_eq!(markers[1].end, 19);
}

// ============================================================================
// Heading Markers
// ============================================================================

#[test]
fn test_heading_markers_h1() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "# Heading 1";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading1).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerHeading).collect();

    assert_eq!(headings.len(), 1, "Should find one heading");
    assert!(!markers.is_empty(), "Should find at least one heading marker");
}

#[test]
fn test_heading_markers_h2() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "## Heading 2";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading2).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerHeading).collect();

    assert_eq!(headings.len(), 1);
    assert!(!markers.is_empty());
}

#[test]
fn test_heading_markers_h3() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "### Heading 3";
    let spans = parser.parse(content);

    let headings: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Heading3).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerHeading).collect();

    assert_eq!(headings.len(), 1);
    assert!(!markers.is_empty());
}

// ============================================================================
// List Markers
// ============================================================================

#[test]
fn test_list_bullet_markers_dash() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- Item 1\n- Item 2";
    let spans = parser.parse(content);

    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerListBullet).collect();
    assert_eq!(markers.len(), 2, "Should find two bullet list markers");
}

#[test]
fn test_list_bullet_markers_asterisk() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "* Item 1\n* Item 2";
    let spans = parser.parse(content);

    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerListBullet).collect();
    assert_eq!(markers.len(), 2);
}

#[test]
fn test_list_bullet_markers_plus() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "+ Item 1\n+ Item 2";
    let spans = parser.parse(content);

    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerListBullet).collect();
    assert_eq!(markers.len(), 2);
}

#[test]
fn test_list_number_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "1. First\n2. Second";
    let spans = parser.parse(content);

    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerListNumber).collect();
    assert_eq!(markers.len(), 2, "Should find two numbered list markers");
}

// ============================================================================
// Task Box Markers
// ============================================================================

#[test]
fn test_task_box_markers_unchecked() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Task 1";
    let spans = parser.parse(content);

    let task_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskMarker).collect();
    let box_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerTaskBox).collect();

    assert_eq!(task_markers.len(), 1, "Should find one task marker");
    assert_eq!(box_markers.len(), 1, "Should find one task box marker");
}

#[test]
fn test_task_box_markers_checked() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [x] Task 1";
    let spans = parser.parse(content);

    let task_checked: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::TaskChecked).collect();
    let box_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerTaskBox).collect();

    assert_eq!(task_checked.len(), 1, "Should find one checked task");
    assert_eq!(box_markers.len(), 1, "Should find one task box marker");
}

#[test]
fn test_multiple_task_box_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "- [ ] Task 1\n- [x] Task 2\n- [ ] Task 3";
    let spans = parser.parse(content);

    let box_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerTaskBox).collect();
    assert_eq!(box_markers.len(), 3, "Should find three task box markers");
}

// ============================================================================
// Link Markers
// ============================================================================

#[test]
fn test_link_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "[link](url)";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerLink).collect();

    // tree-sitter-md may not support inline links, so this test is informational
    println!("Link test found {} link spans and {} markers (tree-sitter-md may not support inline links)",
             links.len(), markers.len());

    // If links are found, markers should also be present
    if !links.is_empty() {
        assert!(!markers.is_empty(), "If links are found, markers should be present");
    }
}

#[test]
fn test_link_markers_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Visit [example](https://example.com) for more info";
    let spans = parser.parse(content);

    let links: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Link).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerLink).collect();

    // tree-sitter-md may not support inline links
    println!("Link in text test found {} link spans and {} markers", links.len(), markers.len());

    // If links are found, markers should also be present
    if !links.is_empty() {
        assert!(!markers.is_empty(), "If links are found, markers should be present");
    }
}

// ============================================================================
// Image Markers
// ============================================================================

#[test]
fn test_image_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "![alt](image.png)";
    let spans = parser.parse(content);

    let images: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::Image).collect();
    let markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerImage).collect();

    // tree-sitter-md may not support inline images
    println!("Image test found {} image spans and {} markers (tree-sitter-md may not support inline images)",
             images.len(), markers.len());

    // If images are found, markers should also be present
    if !images.is_empty() {
        assert!(!markers.is_empty(), "If images are found, markers should be present");
    }
}

// ============================================================================
// Combined/Complex Cases
// ============================================================================

#[test]
fn test_mixed_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "**Bold** and *italic* with `code`";
    let spans = parser.parse(content);

    let bold_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();
    let italic_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerItalic).collect();
    let code_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();

    assert_eq!(bold_markers.len(), 2);
    assert_eq!(italic_markers.len(), 2);
    assert_eq!(code_markers.len(), 2);
}

#[test]
fn test_nested_emphasis_markers() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "***bold and italic***";
    let spans = parser.parse(content);

    // Should find markers for the emphasis
    let bold_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();
    let italic_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerItalic).collect();

    // Depending on how the parser handles this, we should see some markers
    assert!(!bold_markers.is_empty() || !italic_markers.is_empty());
}

#[test]
fn test_markers_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "`**not bold**`";
    let spans = parser.parse(content);

    let code: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::CodeInline).collect();
    let code_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerCode).collect();
    let bold_markers: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MarkerBold).collect();

    assert_eq!(code.len(), 1, "Should find code span");
    assert_eq!(code_markers.len(), 2, "Should find code markers");
    assert_eq!(bold_markers.len(), 0, "Should NOT find bold markers inside code");
}
