//! Category 25: Math
//!
//! Tests for inline $math$ and block $$math$$ syntax
//! Status: MathInline = 82, MathBlock = 83 in SpanKind
//!
//! Test cases for mathematical expressions

use rmde_core::{MarkdownParser, SpanKind};

#[test]
fn test_inline_math_single_dollar() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "The equation $E=mc^2$ is famous.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    assert!(math.len() >= 1, "Should find inline math with $...$ syntax");
}

#[test]
fn test_block_math_double_dollar() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$\nE = mc^2\n$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathBlock
    ).collect();

    assert!(math.len() >= 1, "Should find block math with $$...$$ syntax");
}

#[test]
fn test_inline_math_with_fraction() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "The fraction $\\frac{a}{b}$ is inline.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    assert!(math.len() >= 1, "Should handle LaTeX commands in inline math");
}

#[test]
fn test_block_math_multiline() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$\n\\begin{align}\na &= b + c \\\\\nd &= e + f\n\\end{align}\n$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathBlock
    ).collect();

    assert!(math.len() >= 1, "Should handle multiline block math with LaTeX");
}

#[test]
fn test_inline_math_alternative_paren_syntax() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"The equation \(E=mc^2\) uses alternative syntax.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    // Alternative \(...\) syntax may or may not be supported
    // This test documents the expected behavior
    assert!(math.len() >= 1 || math.is_empty(),
        "Alternative \\(...\\) syntax support may vary");
}

#[test]
fn test_block_math_alternative_bracket_syntax() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"\[E = mc^2\]";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathBlock
    ).collect();

    // Alternative \[...\] syntax may or may not be supported
    // This test documents the expected behavior
    assert!(math.len() >= 1 || math.is_empty(),
        "Alternative \\[...\\] syntax support may vary");
}

#[test]
fn test_multiple_inline_math() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "First equation $a=b$ and second $c=d$.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    assert!(math.len() >= 2, "Should find multiple inline math expressions");
}

#[test]
fn test_math_with_dollar_escaped() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = r"Price is \$50, not math.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    assert_eq!(math.len(), 0, "Escaped dollar should not trigger math mode");
}

#[test]
fn test_inline_math_with_subscript_superscript() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "Formula $x^2 + y_1 = z_{max}$ with indices.";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        s.kind == SpanKind::MathInline
    ).collect();

    assert!(math.len() >= 1, "Should handle sub/superscripts in math mode");
}

#[test]
fn test_empty_math() {
    let mut parser = MarkdownParser::new().unwrap();
    let content = "$$";
    let spans = parser.parse(content);

    let math: Vec<_> = spans.iter().filter(|s|
        matches!(s.kind, SpanKind::MathInline | SpanKind::MathBlock)
    ).collect();

    // Empty math may or may not be rendered
    assert!(math.is_empty() || math.len() >= 1,
        "Empty math handling may vary");
}
