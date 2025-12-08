//! Parser integration tests
//!
//! Organized by specification:
//! - commonmark/ - CommonMark spec tests (headings, emphasis, code, etc.)
//! - gfm/ - GitHub Flavored Markdown extensions (tables, strikethrough, etc.)
//! - extended/ - Extended syntax (footnotes, math, highlighting) - Phase 4
//! - edge_cases - Malformed input, unicode, combined features
//! - integration - Multi-feature integration tests
//! - markers - Marker spans for ghost mode (Phase 3)

mod commonmark;
mod gfm;
mod extended;
mod edge_cases;
mod integration;
mod markers;
