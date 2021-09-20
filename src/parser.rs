use crate::lexer::*;
use std::collections::HashMap;
use std::vec::Vec;
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn parse_value(lexemes: &Vec<Token>) -> Result<(Value, Vec<Token>), &'static str> {
    if lexemes.len() == 0 {
        return Err("Empty value");
    }
    match lexemes[0]._type {
        TokenType::LBracket => parse_object(lexemes),
        TokenType::Quote => {
            let (s, lexemes) = get_key(&lexemes.to_owned())?;
            Ok((Value::String(s), lexemes))
        }
        TokenType::Bool => Ok((
            Value::Bool(lexemes[0].lexeme == "true"),
            lexemes[1..].to_owned(),
        )),
        TokenType::Null => {
            if lexemes.len() == 1 {
                Ok((Value::Null, vec![]))
            } else {
                Ok((Value::Null, lexemes[1..].to_owned()))
            }
        }
        TokenType::SLBracket => parse_array(lexemes),
        TokenType::String => parse_bool_null_number(lexemes),
        _ => Err("expect a value"),
    }
}
fn parse_bool_null_number(lexemes: &Vec<Token>) -> Result<(Value, Vec<Token>), &'static str> {
    if lexemes[0].lexeme == "true" {
        return Ok((Value::Bool(true), lexemes[1..].to_owned()));
    }
    if  lexemes[0].lexeme == "false" {
        return Ok((Value::Bool(false), lexemes[1..].to_owned()));
    }
    if lexemes[0].lexeme == "null" {
        return Ok((Value::Null, lexemes[1..].to_owned()));
    }
    match lexemes[0].lexeme.parse::<f64>() {
        Ok(n) => Ok((Value::Number(n), lexemes[1..].to_owned())),
        Err(_e) => Err("Unknown keyword. Not a number, or true, or false, or null"),
    }
}
fn parse_object(lexemes: &Vec<Token>) -> Result<(Value, Vec<Token>), &'static str> {
    let mut map = HashMap::new();
    if lexemes.len() == 1 {
        return Err("expect more data");
    }
    if lexemes.len() == 2 {
        if lexemes[1]._type == TokenType::RBracket {
            return Ok((Value::Object(map), vec![]));
        } else {
            return Err("Ill format data");
        }
    }
    let (key, lexemes) = get_key(&lexemes[1..].to_vec())?;
    let (_, lexemes) = get_colon(&lexemes)?;
    let (value, lexemes) = parse_value(&lexemes)?;
    map.insert(key, value);
    let mut lexemes = lexemes.to_owned();
    loop {
        if lexemes.len() > 0 && lexemes[0]._type == TokenType::Commas {
            let (key, lex) = get_key(&lexemes[1..].to_owned())?;
            let (_, lex) = get_colon(&lex)?;
            let (value, lex) = parse_value(&lex)?;
            lexemes = lex;
            map.insert(key, value);
        } else {
            break;
        }
    }
    if lexemes[0]._type != TokenType::RBracket {
        return Err("object bracket is not matched.")
    }
    Ok((Value::Object(map), lexemes[1..].to_owned()))
}
fn get_key(lexemes: &Vec<Token>) -> Result<(String, Vec<Token>), &'static str> {
    if lexemes.len() < 3 {
        return Err("String should be enclosed with quote.");
    }

    if lexemes[1]._type == TokenType::String {
        return Ok((lexemes[1].lexeme.clone(), lexemes[3..].to_vec()));
    }
    Err("Key should be string!")
}

fn get_colon(lexemes: &Vec<Token>) -> Result<(Value, Vec<Token>), &'static str> {
    if lexemes[0]._type == TokenType::Colon {
        return Ok((Value::Null, lexemes[1..].to_vec()));
    }
    Err("Expect a colon!")
}
fn parse_array(lexemes: &Vec<Token>) -> Result<(Value, Vec<Token>), &'static str> {
    let mut arr = Vec::new();
    if lexemes.len() < 2 {
        return Err("expect more data for array.");
    }

    if lexemes[1]._type == TokenType::SRBracket {
        return Ok((Value::Array(arr), lexemes[2..].to_owned()));
    }

    let (value, lexemes) = parse_value(&lexemes[1..].to_owned())?;
    arr.push(value);
    let mut lexemes = lexemes.to_owned();
    loop {
        if lexemes.len() > 0 && lexemes[0]._type == TokenType::Commas {
            let (value, lex) = parse_value(&lexemes[1..].to_owned())?;
            lexemes = lex;
            arr.push(value);
        } else {
            break;
        }
    }

    Ok((Value::Array(arr), lexemes[1..].to_owned()))
}
pub fn parse(s: &str) -> Result<(Value, Vec<Token>), &str> {
    let lexemes = get_lexemes(s);
    parse_value(&lexemes)
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_lexemes() {}
    #[test]
    fn parse_string() {
        if let Ok(v) = parse(r#"{"key": "value" }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o["key"], Value::String("value".to_owned()));
                    assert_eq!(o.len(), 1);
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }

        if let Ok(v) = parse("{}") {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 0);
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": []}"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Array(vec![]));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": ["value"]}"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Array(vec![Value::String("value".to_owned())]));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": ["value", "v2"]}"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Array(vec![Value::String("value".to_owned()), Value::String("v2".to_owned())]));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": [  ]  }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Array(vec![]));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }

        if let Ok(v) = parse(r#"{"key": ["value"], "k2": "hi"}"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 2);
                    assert_eq!(o["key"], Value::Array(vec![Value::String("value".to_owned())]));
                    assert_eq!(o["k2"], Value::String("hi".to_owned()));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }

        if let Ok(v) = parse(r#"{"key": true  }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Bool(true));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }

        if let Ok(v) = parse(r#"{"key": false  }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Bool(false));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": null  }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Null);
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
        if let Ok(v) = parse(r#"{"key": [ true, false, null ]  }"#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Object(o) => {
                    assert_eq!(o.len(), 1);
                    assert_eq!(o["key"], Value::Array(vec![Value::Bool(true), Value::Bool(false), Value::Null]));
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
    }
    #[test]
    fn parse_array() {
        if let Ok(v) = parse(r#"[ true, false, null, "hi" ] "#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Array(o) => {
                    assert_eq!(o.len(), 4);
                    assert_eq!(o, vec![Value::Bool(true), Value::Bool(false), Value::Null, Value::String("hi".to_owned())]);
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
    }
    #[test]
    fn parse_number() {
        if let Ok(v) = parse(r#"345 "#) {
            assert!(v.1.len() == 0);
            match v.0 {
                Value::Number(o) => {
                    assert_eq!(o, 345f64);
                }
                _ => assert!(false),
            };
        } else {
            assert!(false);
        }
    }
}
