//! Parser integration tests
//!
//! Organized by specification:
//! - commonmark/ - CommonMark spec tests (headings, emphasis, code, etc.)
//! - gfm/ - GitHub Flavored Markdown extensions (tables, strikethrough, etc.)
//! - edge_cases - Malformed input, unicode, combined features
//! - integration - Multi-feature integration tests

mod commonmark;
mod gfm;
mod edge_cases;
mod integration;
