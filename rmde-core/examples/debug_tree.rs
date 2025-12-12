use tree_sitter::{Parser, Tree};

fn print_tree(tree: &Tree, content: &str, depth: usize) {
    let mut cursor = tree.walk();
    print_node(&mut cursor, content, depth);
}

fn print_node(cursor: &mut tree_sitter::TreeCursor, content: &str, depth: usize) {
    let node = cursor.node();
    let kind = node.kind();
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &content[start..end.min(content.len())];
    let text_preview = if text.len() > 40 {
        format!("{}...", &text[..37])
    } else {
        text.to_string()
    };

    println!("{:indent$}{} [{}-{}]: {:?}", "", kind, start, end, text_preview, indent = depth * 2);

    if cursor.goto_first_child() {
        loop {
            print_node(cursor, content, depth + 1);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

fn main() {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_md::LANGUAGE.into();
    parser.set_language(&lang).unwrap();

    let test_cases = vec![
        "[Simple link](https://example.com)",
        "![Alt text](image.png)",
        "---",
        "***",
        "___",
        "* * *",
        "***text***",
    ];

    for content in test_cases {
        println!("\n\n=== Content: {:?} ===", content);
        if let Some(tree) = parser.parse(content, None) {
            print_tree(&tree, content, 0);
        }
    }
}
