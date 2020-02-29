use verilog_lang::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lex(input: &str) -> JsValue {
    let lexer = lexer::Lexer::lex(input);
    let result = (lexer.tokens, lexer.diag);
    JsValue::from_serde(&result).unwrap()
}