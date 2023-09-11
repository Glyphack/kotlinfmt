use tree_sitter::Parser;
use tree_sitter_kotlin;

fn main() {
    let code = r#"
  data class Point(
    val x: Int,
    val y: Int
  )
"#;
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_kotlin::language())
        .expect("Error loading Kotlin grammar");
    let parsed = parser.parse(code, None);
    println!("{:?}", parsed);
}

