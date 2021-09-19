use std::collections::HashMap;

use crate::lexer_lt::{gen_lexemes, Lexeme};
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
fn parse_lt(s: &str) -> Result<Value, &'static str> {
    let lexemes = gen_lexemes(s);
    let v = parse_value(&lexemes)?;
    Ok(v.0)
}
fn parse_value<'a, 'b>(
    lexemes: &'a [Lexeme<'b>],
) -> Result<(Value, &'a [Lexeme<'b>]), &'static str> {
    if lexemes.len() == 0 {
        return Ok((Value::String("".to_owned()), lexemes));
    }
    if lexemes[0]._type == b'{' {
        return parse_object(lexemes);
    }
    if lexemes[0]._type == b'[' {
        return parse_array(lexemes);
    }
    if lexemes[0]._type == b'"' {
        return parse_string(lexemes);
    }
    if lexemes[0]._type == b'n' {
        return Ok((Value::Null, &lexemes[1..lexemes.len()]));
    }
    if lexemes[0]._type == b't' {
        return Ok((
            Value::Bool(if lexemes[0].s == "true".as_bytes() {
                true
            } else {
                false
            }),
            &lexemes[1..lexemes.len()],
        ));
    }
    // if it is number
    if lexemes[0]._type == b'u' {
        if let Ok(num) = std::str::from_utf8(lexemes[0].s).unwrap().parse::<f64>() {
            return Ok((Value::Number(num), &lexemes[1..lexemes.len()]));
        }
    }
    unimplemented!()
}

fn parse_object<'a, 'b>(
    lexemes: &'a [Lexeme<'b>],
) -> Result<(Value, &'a [Lexeme<'b>]), &'static str> {
    if lexemes.len() < 2 || lexemes[0]._type != b'{' {
        return Err("Not a object.");
    }
    if lexemes[1]._type == b'}' {
        return Ok((Value::Object(HashMap::new()), &lexemes[2..lexemes.len()]));
    }
    let mut m = HashMap::new();
    let mut lexemes = &lexemes[1..lexemes.len()];
    loop {
        if let Ok(k) = parse_string(lexemes) {
            match k.0 {
                Value::String(s) => {
                    if k.1[0]._type != b':' {
                        return Err("colon expected.");
                    }
                    lexemes = &k.1[1..k.1.len()];
                    if let Ok(v) = parse_value(lexemes) {
                        let value = v.0;
                        m.insert(s, value);
                        if v.1[0]._type != b',' {
                            lexemes = v.1;

                            break;
                        }
                        lexemes = &v.1[1..v.1.len()];
                    } else {
                        return Err("not a value.");
                    }
                }
                _ => {
                    return Err("not a string.");
                }
            }
        } else {
            return Err("string expected.");
        }
    }
    if lexemes.len() < 1 || lexemes[0]._type != b'}' {
        return Err("right bracket expected.");
    }
    return Ok((Value::Object(m), &lexemes[1..lexemes.len()]));
}

fn parse_array<'a, 'b>(
    lexemes: &'a [Lexeme<'b>],
) -> Result<(Value, &'a [Lexeme<'b>]), &'static str> {
    if lexemes.len() < 2 || lexemes[0]._type != b'[' {
        return Err("expect array");
    }
    let mut vec = vec![];
    let len = lexemes.len();
    let mut lexemes = &lexemes[1..len];
    loop {
        if lexemes.len() == 0 {
            return Err("array expected.");
        }
        if lexemes[0]._type == b']' {
            break;
        }
        if let Ok(v) = parse_value(lexemes) {
            vec.push(v.0);
            lexemes = v.1;
            if lexemes.len() == 0 {
                return Err("array expected.");
            }
            if lexemes[0]._type == b',' {
                lexemes = &lexemes[1..lexemes.len()];
            }
        } else {
            return Err("expect value inside array");
        }
    }

    Ok((Value::Array(vec), &lexemes[1..lexemes.len()]))
}
fn parse_string<'a, 'b>(
    lexemes: &'a [Lexeme<'b>],
) -> Result<(Value, &'a [Lexeme<'b>]), &'static str> {
    if lexemes.len() < 3
        || lexemes[0]._type != b'"'
        || lexemes[2]._type != b'"'
        || lexemes[1]._type != b's'
    {
        return Err("expected string");
    }
    Ok((
        Value::String(std::str::from_utf8(lexemes[1].s).unwrap().to_owned()),
        &lexemes[3..lexemes.len()],
    ))
}
#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_parsing() {
        {
            let v = parse_lt("{}");
            let exp = Value::Object(HashMap::new());
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key":"value"}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::String("value".to_owned()));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key": null}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Null);
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key": true   }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(true));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key": false   }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key": false , "k2": "v2"  }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            m.insert("k2".to_owned(), Value::String("v2".to_owned()));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"{"key": false , "k2": {"k3": null}  }"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Bool(false));
            let mut nm = HashMap::new();
            nm.insert("k3".to_owned(), Value::Null);
            m.insert("k2".to_owned(), Value::Object(nm));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse_lt(r#"[]"#);
            let vec = vec![];
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }
        {
            let v = parse_lt(r#"[ null ]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse_lt(r#"[ null , false]"#);
            let mut vec = vec![];
            vec.push(Value::Null);
            vec.push(Value::Bool(false));
            let exp = Value::Array(vec);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse_lt(r#"[ null , false, {"k1": "v2"}]"#);
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
            let v = parse_lt(r#"[ null , false, {"k1": "v2"}, "ss"]"#);
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
            let v = parse_lt(r#"{ "kk": [ null , false, {"k1": "v2"}, "ss"]}"#);
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
            let v = parse_lt(r#"{"key":345}"#);
            let mut m = HashMap::new();
            m.insert("key".to_owned(), Value::Number(345.0));
            let exp = Value::Object(m);
            assert_eq!(exp, v.unwrap());
        }

        {
            let v = parse_lt(r#"{"key":345, "k2": [123, true]}"#);
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
            let v = parse_lt(r#"{"key":345, "k2": [123e2, true]}"#);
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
