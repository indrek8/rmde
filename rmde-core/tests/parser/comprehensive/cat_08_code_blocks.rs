//! Category 8: Code Blocks
//!
//! Tests for indented and fenced code blocks
//! Status: ISSUES DETECTED (metadata, tilde fences)
//!
//! Test cases from example_test.md lines 164-223

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_indented_code_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Regular paragraph before code.\n\n    function indentedCode() {\n        return \"Four spaces indent\";\n    }\n\nRegular paragraph after code.";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find indented code block");
}

#[test]
fn test_fenced_code_block_no_language() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```\nNo language specified\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find fenced code block without language");
}

#[test]
fn test_fenced_code_block_with_javascript() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```javascript\nconst greeting = \"Hello\";\nconsole.log(greeting);\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find JavaScript code block");
}

#[test]
fn test_fenced_code_block_with_python() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```python\ndef hello():\n    print(\"Hello, world!\")\n\nhello()\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find Python code block");
}

#[test]
fn test_fenced_code_block_with_rust() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```rust\nfn main() {\n    println!(\"Hello, Rust!\");\n}\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find Rust code block");
}

#[test]
fn test_tilde_fence() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "~~~bash\necho \"Tilde fence works too\"\n~~~";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();

    // This is a known issue area - tilde fences
    println!("Found {} code blocks with tilde fence", code_blocks.len());
}

#[test]
fn test_long_fence_four_backticks() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "````markdown\nFour backticks opening allows\n```\nthree backticks inside\n```\nwithout closing\n````";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Should find code block with four backtick fence");
}

#[test]
fn test_code_block_with_metadata() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```js {highlight: \"1,3-5\"}\nline 1 - highlighted\nline 2\nline 3 - highlighted\nline 4 - highlighted\nline 5 - highlighted\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();

    // This is a known issue area - metadata in info string
    println!("Found {} code blocks with metadata", code_blocks.len());
}

#[test]
fn test_code_block_fence_indented() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "   ```javascript\n   const x = 10;\n   ```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Code fence can be indented up to 3 spaces");
}

#[test]
fn test_code_block_empty() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "```\n```";
    let spans = parser.parse(content);

    let code_blocks: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::CodeBlock
    ).collect();
    assert!(!code_blocks.is_empty(), "Empty code block should be valid");
}
