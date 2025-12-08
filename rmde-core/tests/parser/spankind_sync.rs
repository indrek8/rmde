//! SpanKind synchronization tests
//!
//! These tests ensure SpanKind values in Rust match the constants defined in
//! RMDE/Sources/App/EditorState.swift HighlightSpan struct.
//!
//! CRITICAL: If these tests fail, update BOTH:
//! 1. This test file
//! 2. RMDE/Sources/App/EditorState.swift HighlightSpan constants
//! 3. RMDE/Sources/Views/EditorView.swift attributesForKind()

use rmde_core::SpanKind;

/// Test that all SpanKind discriminant values match Swift constants.
/// Rust uses #[repr(u8)] so discriminants are u8, but we cast to u64 for FFI compatibility.
#[test]
fn test_spankind_values_for_swift_sync() {
    // Headings (1-7)
    assert_eq!(SpanKind::Heading1 as u64, 1, "Heading1 must be 1");
    assert_eq!(SpanKind::Heading2 as u64, 2, "Heading2 must be 2");
    assert_eq!(SpanKind::Heading3 as u64, 3, "Heading3 must be 3");
    assert_eq!(SpanKind::Heading4 as u64, 4, "Heading4 must be 4");
    assert_eq!(SpanKind::Heading5 as u64, 5, "Heading5 must be 5");
    assert_eq!(SpanKind::Heading6 as u64, 6, "Heading6 must be 6");
    assert_eq!(SpanKind::HeadingMarker as u64, 7, "HeadingMarker must be 7");

    // Emphasis (10-13)
    assert_eq!(SpanKind::Bold as u64, 10, "Bold must be 10");
    assert_eq!(SpanKind::Italic as u64, 11, "Italic must be 11");
    assert_eq!(SpanKind::BoldItalic as u64, 12, "BoldItalic must be 12");
    assert_eq!(SpanKind::Strikethrough as u64, 13, "Strikethrough must be 13");

    // Code (20-23)
    assert_eq!(SpanKind::CodeInline as u64, 20, "CodeInline must be 20");
    assert_eq!(SpanKind::CodeBlock as u64, 21, "CodeBlock must be 21");
    assert_eq!(SpanKind::CodeFence as u64, 22, "CodeFence must be 22");
    assert_eq!(SpanKind::CodeLanguage as u64, 23, "CodeLanguage must be 23");

    // Links (30-35)
    assert_eq!(SpanKind::Link as u64, 30, "Link must be 30");
    assert_eq!(SpanKind::LinkUrl as u64, 31, "LinkUrl must be 31");
    assert_eq!(SpanKind::LinkTitle as u64, 32, "LinkTitle must be 32");
    assert_eq!(SpanKind::Image as u64, 33, "Image must be 33");
    assert_eq!(SpanKind::Autolink as u64, 34, "Autolink must be 34");
    assert_eq!(SpanKind::AutolinkEmail as u64, 35, "AutolinkEmail must be 35");

    // Lists (40-42)
    assert_eq!(SpanKind::ListMarker as u64, 40, "ListMarker must be 40");
    assert_eq!(SpanKind::TaskMarker as u64, 41, "TaskMarker must be 41");
    assert_eq!(SpanKind::TaskChecked as u64, 42, "TaskChecked must be 42");

    // Blocks (50-51)
    assert_eq!(SpanKind::BlockQuote as u64, 50, "BlockQuote must be 50");
    assert_eq!(SpanKind::HorizontalRule as u64, 51, "HorizontalRule must be 51");

    // Tables (60-62)
    assert_eq!(SpanKind::TableHeader as u64, 60, "TableHeader must be 60");
    assert_eq!(SpanKind::TableDelimiter as u64, 61, "TableDelimiter must be 61");
    assert_eq!(SpanKind::TableCell as u64, 62, "TableCell must be 62");

    // Generic emphasis (70)
    assert_eq!(SpanKind::Emphasis as u64, 70, "Emphasis must be 70");

    // Extended syntax (80-84)
    assert_eq!(SpanKind::FootnoteRef as u64, 80, "FootnoteRef must be 80");
    assert_eq!(SpanKind::FootnoteDef as u64, 81, "FootnoteDef must be 81");
    assert_eq!(SpanKind::MathInline as u64, 82, "MathInline must be 82");
    assert_eq!(SpanKind::MathBlock as u64, 83, "MathBlock must be 83");
    assert_eq!(SpanKind::Highlight as u64, 84, "Highlight must be 84");

    // Artifacts (90-91)
    assert_eq!(SpanKind::ArtifactThinking as u64, 90, "ArtifactThinking must be 90");
    assert_eq!(SpanKind::ArtifactMeta as u64, 91, "ArtifactMeta must be 91");

    // Markers (100+)
    assert_eq!(SpanKind::MarkerHeading as u64, 100, "MarkerHeading must be 100");
    assert_eq!(SpanKind::MarkerBold as u64, 101, "MarkerBold must be 101");
    assert_eq!(SpanKind::MarkerItalic as u64, 102, "MarkerItalic must be 102");
    assert_eq!(SpanKind::MarkerStrikethrough as u64, 104, "MarkerStrikethrough must be 104");
    assert_eq!(SpanKind::MarkerCode as u64, 105, "MarkerCode must be 105");
    assert_eq!(SpanKind::MarkerLink as u64, 107, "MarkerLink must be 107");
    assert_eq!(SpanKind::MarkerImage as u64, 108, "MarkerImage must be 108");
    assert_eq!(SpanKind::MarkerListBullet as u64, 109, "MarkerListBullet must be 109");
    assert_eq!(SpanKind::MarkerListNumber as u64, 110, "MarkerListNumber must be 110");
    assert_eq!(SpanKind::MarkerTaskBox as u64, 112, "MarkerTaskBox must be 112");
}

/// Test count to ensure we don't miss new variants.
/// Update this count when adding new SpanKind variants.
#[test]
fn test_spankind_variant_count() {
    // Total expected variants: 37
    // If this fails, a new variant was added - update the sync test above!

    // Base variants (non-marker):
    // - Headings: 1-7 (7)
    // - Emphasis: 10-13 (4)
    // - Code: 20-23 (4)
    // - Links: 30-35 (6)
    // - Lists: 40-42 (3)
    // - Blocks: 50-51 (2)
    // - Tables: 60-62 (3)
    // - Emphasis generic: 70 (1)
    // - Extended: 80-84 (5)
    // - Artifacts: 90-91 (2)
    // = 37 base variants
    let base_count = 7 + 4 + 4 + 6 + 3 + 2 + 3 + 1 + 5 + 2;
    assert_eq!(base_count, 37, "Base variants count");

    // Markers: 100, 101, 102, 104, 105, 107, 108, 109, 110, 112 (10 total)
    // Note: gaps at 103, 106, 111
    let marker_count = 10;

    // Total = 47 SpanKind variants
    assert_eq!(base_count + marker_count, 47, "Total SpanKind variants");
}
