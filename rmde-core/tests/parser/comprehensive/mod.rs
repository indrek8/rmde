//! Comprehensive parser tests based on example_test.md
//!
//! This module contains systematic tests for all 31 markdown syntax categories
//! to identify parsing failures and verify expected SpanKind output.
//!
//! Each category has a dedicated test file for isolated testing and debugging.

mod cat_01_atx_headings;
mod cat_02_setext_headings;
mod cat_03_paragraphs_linebreaks;
mod cat_04_blockquotes;
mod cat_05_unordered_lists;
mod cat_06_ordered_lists;
mod cat_07_task_lists;
mod cat_08_code_blocks;
mod cat_09_thematic_breaks;
mod cat_10_tables;
mod cat_11_emphasis_italic;
mod cat_12_strong_bold;
mod cat_13_combined_emphasis;
mod cat_14_strikethrough;
mod cat_15_code_spans;
mod cat_16_links;
mod cat_17_images;
mod cat_18_autolinks;
mod cat_19_html_blocks;
mod cat_20_footnotes;
