//! Parser integration tests
//!
//! Organized by specification:
//! - commonmark/ - CommonMark spec tests (headings, emphasis, code, etc.)
//! - gfm/ - GitHub Flavored Markdown extensions (tables, strikethrough, etc.)
//! - extended/ - Extended syntax (footnotes, math, highlighting) - Phase 4
//! - edge_cases - Malformed input, unicode, combined features
//! - integration - Multi-feature integration tests
//! - markers - Marker spans for ghost mode (Phase 3)
//! - streaming - Streaming parser for LLM output (Phase 6)
//! - artifacts - Claude artifact parsing (Phase 6 - Optional)
//! - spankind_sync - Rust/Swift SpanKind value synchronization tests

mod commonmark;
mod gfm;
mod extended;
mod edge_cases;
mod integration;
mod markers;
mod streaming;
mod artifacts;
mod spankind_sync;
