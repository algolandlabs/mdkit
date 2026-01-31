use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

pub mod ast;
pub mod parser;
pub mod renderer;

#[wasm_bindgen]
pub fn markdown_to_html(input: &str) -> String {
    let mut parser = parser::Parser::new(input);
    let ast = parser.parse_document();
    renderer::render(&ast)
}

#[wasm_bindgen]
pub fn markdown_to_ast(input: &str) -> JsValue {
    let mut parser = crate::parser::Parser::new(input);
    let nodes = parser.parse_document();

    // JS ob'ektiga aylantirish
    to_value(&nodes).unwrap_or(JsValue::NULL)
}
