use mdkit::markdown_to_html;
use mdkit::parser::Parser;

fn main() {
    // 1. Sinov uchun murakkab Markdown matni
    let markdown_input = r#"
This image ![The San Juan Mountains are beautiful](/assets/images/san-juan-mountains.jpg "San Juan Mountains")
My favorite search engine is [Duck Duck Go](https://duckduckgo.com).
"#;

    println!("--- ORIGINAL MARKDOWN ---");
    println!("{}", markdown_input);

    // 2. AST strukturasini ko'rish (Debug uchun)
    let mut parser = Parser::new(markdown_input);
    let ast = parser.parse_document();

    println!("\n--- ABSTRACT SYNTAX TREE (AST) ---");
    println!("{:#?}", ast);

    // 3. HTML render qilish
    let html_output = markdown_to_html(markdown_input);

    println!("\n--- RENDERED HTML ---");
    println!("{}", html_output);
}
