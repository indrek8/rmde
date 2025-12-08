use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rmde_core::MarkdownParser;

/// Benchmark parsing a small file (~10KB)
fn bench_parse_small(c: &mut Criterion) {
    // Create ~10KB of Markdown content with various elements
    let heading_content = "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n".repeat(50);
    let emphasis_content = "**bold** and *italic* and ***bold italic*** text. ".repeat(100);
    let code_content = "Inline `code` and more `code blocks` here. ".repeat(50);
    let link_content = "[Link text](https://example.com) and ![Image](img.png). ".repeat(50);
    let list_content = "- Item 1\n- Item 2\n  - Nested item\n- Item 3\n\n".repeat(50);

    let content = format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}",
        heading_content, emphasis_content, code_content, link_content, list_content
    );

    c.bench_function("parse_small_10kb", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        b.iter(|| {
            parser.reset();
            parser.parse(&content)
        })
    });
}

/// Benchmark parsing a large file (~1MB)
fn bench_parse_large(c: &mut Criterion) {
    // Create ~1MB of Markdown content
    let heading_content = "# Heading\n\n## Subheading\n\nParagraph with **bold** and *italic* text.\n\n".repeat(5000);
    let code_block = "```rust\nfn example() {\n    println!(\"Hello, world!\");\n}\n```\n\n".repeat(500);
    let table = "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n\n".repeat(500);

    let content = format!("{}\n\n{}\n\n{}", heading_content, code_block, table);

    c.bench_function("parse_large_1mb", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        b.iter(|| {
            parser.reset();
            parser.parse(&content)
        })
    });
}

/// Benchmark incremental parsing (simulated edits)
fn bench_incremental_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental_parsing");

    // Base content
    let base_content = "# Heading\n\n".repeat(100) + &"**bold** text\n".repeat(500);

    // Test full parse vs incremental parse
    group.bench_function("full_parse", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        b.iter(|| {
            parser.reset();
            parser.parse(&base_content)
        })
    });

    group.bench_function("incremental_parse", |b| {
        let mut parser = MarkdownParser::new().unwrap();
        // First parse to establish baseline
        parser.parse(&base_content);

        b.iter(|| {
            // Simulate small edit - replace "bold" with "BOLD"
            let modified = base_content.replace("bold", "BOLD");
            parser.parse(&modified)
        })
    });

    group.finish();
}

/// Benchmark parsing documents with different amounts of code blocks
/// This tests the skip_regions optimization
fn bench_code_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("code_heavy_documents");

    // Document with 10% code blocks
    let text = "Normal paragraph with **bold** and *italic*.\n\n".repeat(90);
    let code = "```rust\nfn example() {}\n```\n\n".repeat(10);
    let content_10pct = format!("{}{}", text, code);

    // Document with 50% code blocks
    let text = "Normal paragraph with **bold** and *italic*.\n\n".repeat(50);
    let code = "```rust\nfn example() {}\n```\n\n".repeat(50);
    let content_50pct = format!("{}{}", text, code);

    // Document with 90% code blocks
    let text = "Normal paragraph with **bold** and *italic*.\n\n".repeat(10);
    let code = "```rust\nfn example() {}\n```\n\n".repeat(90);
    let content_90pct = format!("{}{}", text, code);

    for (name, content) in &[
        ("10pct_code", &content_10pct),
        ("50pct_code", &content_50pct),
        ("90pct_code", &content_90pct),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            let mut parser = MarkdownParser::new().unwrap();
            b.iter(|| {
                parser.reset();
                parser.parse(content)
            })
        });
    }

    group.finish();
}

/// Benchmark parsing different Markdown element types
fn bench_element_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("element_types");

    // Heavy emphasis (bold/italic)
    let emphasis_heavy = "**bold** and *italic* and ***both*** ".repeat(1000);

    // Heavy links
    let links_heavy = "[link](https://example.com) and [another](https://test.com) ".repeat(1000);

    // Heavy lists
    let lists_heavy = "- Item 1\n- Item 2\n  - Nested\n- Item 3\n\n".repeat(500);

    // Heavy headings
    let headings_heavy = "# H1\n\n## H2\n\n### H3\n\n".repeat(500);

    for (name, content) in &[
        ("emphasis_heavy", &emphasis_heavy),
        ("links_heavy", &links_heavy),
        ("lists_heavy", &lists_heavy),
        ("headings_heavy", &headings_heavy),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            let mut parser = MarkdownParser::new().unwrap();
            b.iter(|| {
                parser.reset();
                parser.parse(content)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_small,
    bench_parse_large,
    bench_incremental_parse,
    bench_code_heavy,
    bench_element_types
);
criterion_main!(benches);
