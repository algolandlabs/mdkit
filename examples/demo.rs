use mdkit::markdown_to_html;
use mdkit::parser::Parser;

fn main() {
    let markdown_input = r#"
---
__Advertisement :)__

- __[pica](https://nodeca.github.io/pica/demo/)__ - high quality and fast image resize in browser.
- __[babelfish](https://github.com/nodeca/babelfish/)__ - developer friendly
  i18n with plurals support and easy syntax.

You will like those projects!
"#;

    println!("--- ORIGINAL MARKDOWN ---");
    println!("{}", markdown_input);

    let mut parser = Parser::new(markdown_input);
    let ast = parser.parse_document();

    println!("\n--- ABSTRACT SYNTAX TREE (AST) ---");
    println!("{:#?}", ast);

    let html_output = markdown_to_html(markdown_input);

    println!("\n--- RENDERED HTML ---");
    println!("{}", html_output);
}
