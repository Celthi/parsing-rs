use std::collections::HashMap;

/// A parser to parse JSON from string written with top-down parsing method.
use crate::lexer::{generate_tokens, Token, TokenType};
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub fn parse(s: &str) -> Result<Value, &'static str> {
    // first tokenize the string into tokens
    let tokens = generate_tokens(s);
    // then construct the Json value from the tokens.
    let (value, tokens) = parse_value(&tokens)?;
    if !tokens.is_empty() {
        return Err("trailing string after json.");
    }
    Ok(value)
}

// construct a value from the tokens and return the value and any left tokens.
fn parse_value<'a, 'b>(tokens: &'a [Token<'b>]) -> Result<(Value, &'a [Token<'b>]), &'static str> {
    if tokens.is_empty() {
        return Ok((Value::String("".to_owned()), tokens));
    }
    match tokens[0]._type {
        TokenType::LeftBracket => parse_object(tokens),
        TokenType::LeftSquareBracket => parse_array(tokens),
        TokenType::Quote => parse_string(tokens),
        TokenType::Null => Ok((Value::Null, &tokens[1..])),
        TokenType::Boolean => Ok((Value::Bool(tokens[0].s == "true".as_bytes()), &tokens[1..])),
        // if it is number, for simplicity, we use f64 always
        TokenType::Number => {
            if let Ok(num) = std::str::from_utf8(tokens[0].s).unwrap().parse::<f64>() {
                Ok((Value::Number(num), &tokens[1..]))
            } else {
                Err("cannot parse the string into the numbers.")
            }
        }
        _ => Err("unsupported format."),
    }
}

fn parse_object<'a, 'b>(tokens: &'a [Token<'b>]) -> Result<(Value, &'a [Token<'b>]), &'static str> {
    if tokens.len() < 2 || tokens[0]._type != TokenType::LeftBracket {
        return Err("Not a object.");
    }
    // empty object
    if tokens[1]._type == TokenType::RightBracket {
        return Ok((Value::Object(HashMap::new()), &tokens[2..]));
    }
    let mut m = HashMap::new();
    let mut tokens = &tokens[1..];
    loop {
        if let (Value::String(s), token) = parse_string(tokens)? {
            if token[0]._type != TokenType::Colon {
                return Err("colon expected.");
            }
            let (value, token) = parse_value(&token[1..])?;
            m.insert(s, value);
            // if there is no more key value pair to deal with.
            if token[0]._type != TokenType::Comma {
                tokens = token;
                break;
            }
            tokens = &token[1..];
        }
    }
    if tokens.is_empty() || tokens[0]._type != TokenType::RightBracket {
        return Err("right bracket expected.");
    }
    Ok((Value::Object(m), &tokens[1..]))
}

fn parse_array<'a, 'b>(tokens: &'a [Token<'b>]) -> Result<(Value, &'a [Token<'b>]), &'static str> {
    if tokens.len() < 2 || tokens[0]._type != TokenType::LeftSquareBracket {
        return Err("expect array");
    }
    let mut vec = vec![];
    let mut tokens = &tokens[1..];

    loop {
        if tokens.is_empty() {
            return Err("array expected.");
        }
        if tokens[0]._type == TokenType::RightSquareBracket {
            break;
        }
        let (value, token) = parse_value(tokens)?;
        vec.push(value);
        if token.is_empty() {
            return Err("array expected.");
        }
        if token[0]._type == TokenType::Comma {
            tokens = &token[1..];
        } else {
            tokens = token;
        }
    }
    Ok((Value::Array(vec), &tokens[1..]))
}

fn parse_string<'a, 'b>(tokens: &'a [Token<'b>]) -> Result<(Value, &'a [Token<'b>]), &'static str> {
    if tokens.len() < 3
        || tokens[0]._type != TokenType::Quote
        || tokens[2]._type != TokenType::Quote
        || tokens[1]._type != TokenType::String
    {
        return Err("expected string");
    }
    Ok((
        Value::String(std::str::from_utf8(tokens[1].s).unwrap().to_owned()),
        &tokens[3..],
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_parsing() {
        {
            let v = parse("{}");
            let exp = Value::Object(HashMap::new());
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key":"value"}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::String("value".to_owned()));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key": null}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Null);
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key": true   }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(true));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key": false   }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key": false , "k2": "v2"  }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            m.insert("k2".to_owned(), Value::String("v2".to_owned()));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key": false , "k2": {"k3": null}  }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            let mut nm = HashMap::new();
            nm.insert("k3".to_owned(), Value::Null);
            m.insert("k2".to_owned(), Value::Object(nm));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"[]"#);
            let vec = vec![];
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"[ null ]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"[ null , false]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            vec.push(Value::Bool(false));
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"[ null , false, {"k1": "v2"}]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            vec.push(Value::Bool(false));
            let mut m = HashMap::new();
            m.insert("k1".to_owned(), Value::String("v2".to_owned()));
            vec.push(Value::Object(m));
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"[ null , false, {"k1": "v2"}, "ss"]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            vec.push(Value::Bool(false));
            let mut m = HashMap::new();
            m.insert("k1".to_owned(), Value::String("v2".to_owned()));
            vec.push(Value::Object(m));
            vec.push(Value::String("ss".to_owned()));
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"{ "kk": [ null , false, {"k1": "v2"}, "ss"]}"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            vec.push(Value::Bool(false));
            let mut m = HashMap::new();
            m.insert("k1".to_owned(), Value::String("v2".to_owned()));
            vec.push(Value::Object(m));
            vec.push(Value::String("ss".to_owned()));
            let mut mo = HashMap::new();
            mo.insert("kk".to_owned(), Value::Array(vec));
            let exp = Value::Object(mo);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"{"key":345}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Number(345.0));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse(r#"{"key":345, "k2": [123, true]}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Number(345.0));
            let mut vec = vec![];
            vec.push(Value::Number(123.0));
            vec.push(Value::Bool(true));
            m.insert("k2".to_owned(), Value::Array(vec));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse(r#"{"key":345, "k2": [123e2, true]}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Number(345.0));
            let mut vec = vec![];
            vec.push(Value::Number(123e2));
            vec.push(Value::Bool(true));
            m.insert("k2".to_owned(), Value::Array(vec));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
    }
}
