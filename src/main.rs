#![allow(dead_code)]

mod lexer;
mod parser;
fn main() {
    println!("Hello, world!");
    let _v = parser::parse(r#"{"key":"value"}"#);

}
