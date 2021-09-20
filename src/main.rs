#![allow(dead_code)]

mod parser;
mod lexer;
mod lexer_lt;
mod parser_lt;
fn main() {
    println!("Hello, world!");
    let _v = parser_lt::parse_lt(r#"{"key":"value"}"#);

}
