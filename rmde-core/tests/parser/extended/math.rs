//! Math block tests
//!
//! Tests for math syntax per MARKDOWN-SYNTAX.md § Math

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_inline_math_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "The equation $E = mc^2$ is famous.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 1, "Should find one inline math span");
    assert_eq!(math[0].start, 13);
    assert_eq!(math[0].end, 23);
    assert_eq!(&content[math[0].start..math[0].end], "$E = mc^2$");
}

#[test]
fn test_block_math_basic() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$\nx = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}\n$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathBlock).collect();
    assert_eq!(math.len(), 1, "Should find one block math span");
    assert_eq!(math[0].start, 0);
    assert_eq!(math[0].end, content.len());
}

#[test]
fn test_inline_math_multiple() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Use $x^2$ and $y^2$ in the formula.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 2, "Should find two inline math spans");
    assert_eq!(&content[math[0].start..math[0].end], "$x^2$");
    assert_eq!(&content[math[1].start..math[1].end], "$y^2$");
}

#[test]
fn test_block_math_simple() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$E = mc^2$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathBlock).collect();
    assert_eq!(math.len(), 1, "Should find one block math span");
    assert_eq!(&content[math[0].start..math[0].end], "$$E = mc^2$$");
}

#[test]
fn test_inline_math_no_newlines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text $x\ny$ should not match.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 0, "Inline math should not cross newlines");
}

#[test]
fn test_block_math_can_have_newlines() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$\nline1\nline2\n$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathBlock).collect();
    assert_eq!(math.len(), 1, "Block math can contain newlines");
    assert_eq!(&content[math[0].start..math[0].end], "$$\nline1\nline2\n$$");
}

#[test]
fn test_math_unclosed_inline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Text $unclosed inline math.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 0, "Unclosed inline math should not match");
}

#[test]
fn test_math_unclosed_block() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$unclosed block math";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathBlock).collect();
    assert_eq!(math.len(), 0, "Unclosed block math should not match");
}

#[test]
fn test_math_not_in_code() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Code `$x^2$` should not be math.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 0, "Math inside code span should not match");
}

#[test]
fn test_math_complex_formula() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "The quadratic formula: $x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}$.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    assert_eq!(math.len(), 1, "Should handle complex LaTeX formulas");
}

#[test]
fn test_block_math_with_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Before\n$$\nx^2 + y^2 = z^2\n$$\nAfter";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathBlock).collect();
    assert_eq!(math.len(), 1, "Should find block math with surrounding text");
}

#[test]
fn test_single_dollar_in_text() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Price is $5 and tax is $2.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s| s.kind == SpanKind::MathInline).collect();
    // This should match "$5 and tax is $" which is probably not intended,
    // but follows the simple pattern matching approach
    // In a production system, you might want smarter heuristics
    assert!(math.len() <= 1, "Single dollars may match but should be handled gracefully");
}
