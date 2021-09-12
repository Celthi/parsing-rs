use std::{ vec::Vec};
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Unknown,
    Colon,       // :
    Commas,      // ,
    SLBracket, // [
    SRBracket, // ]
    LBracket,   // {
    RBracket,   // }
    String,
    Quote, // "
    Bool,  // true or false
    Null,  // null
}
#[derive(PartialEq, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
}

fn is_whitespace(i: u8) -> bool {
    i == b' ' || i == b'\t' || i == b'\n' || i == b'\r'
}
fn add_token(res: &mut Vec<Token>, i: usize, start: usize, s: &str) {
    if i - start != 0 {
        res.push(Token{_type: TokenType::String, lexeme: s[start..i].to_owned()});
    }
}
pub fn get_lexemes(s: &str) -> Vec<Token> {
    let l = s.len();
    let mut res = Vec::new();
    let mut i: usize = 0;
    let mut in_quote = false;
    let mut start = 0usize;
    let bytes = s.as_bytes();
    loop {
        if i >= l {
            break;
        }
        if bytes[i] == b'\"' {
            if in_quote {
                let lexeme = s[start..i].to_owned();
                res.push(Token {
                    _type: TokenType::String,
                    lexeme,
                });
                in_quote = false;
                res.push(Token {
                    _type: TokenType::Quote,
                    lexeme: "\"".to_owned(),
                });
                start = i+1
            } else {
                in_quote = true;
                start = i+1;
                res.push(Token {
                    _type: TokenType::Quote,
                    lexeme: "\"".to_owned(),
                });
            }
        } else if bytes[i] == b'{' {
            add_token(&mut res, i, start, s);

            start = i + 1;
            res.push(Token {
                _type: TokenType::LBracket,
                lexeme: "{".to_owned(),
            });
        } else if bytes[i] == b'}' {
            add_token(&mut res, i, start, s);
            start = i + 1;
            res.push(Token {
                _type: TokenType::RBracket,
                lexeme: "}".to_owned(),
            });
        } else if bytes[i] == b',' {
            add_token(&mut res, i, start, s);
            start = i + 1;
            res.push(Token {
                _type: TokenType::Commas,
                lexeme: ",".to_owned(),
            });
        } else if bytes[i] == b'[' {
            add_token(&mut res, i, start, s);
            start = i + 1;
            res.push(Token {
                _type: TokenType::SLBracket,
                lexeme: "[".to_owned(),
            });
        } else if bytes[i] == b']' {
            add_token(&mut res, i, start, s);
            start = i + 1;
            res.push(Token {
                _type: TokenType::SRBracket,
                lexeme: "]".to_owned(),
            });
        } else if bytes[i] == b':' {
            add_token(&mut res, i, start, s);
            start = i + 1;
            res.push(Token {
                _type: TokenType::Colon,
                lexeme: ":".to_owned(),
            })
        } else if is_whitespace(bytes[i]) {
            add_token(&mut res, i, start, s);
            start = i + 1;
        }
        i += 1;
    }
    res
}
fn compare_tokens(v1: &Vec<Token>, v2: &Vec<Token>) {
    assert_eq!(v1.len(), v2.len());
    for i in 0..v1.len() {
        assert_eq!(v1[i].lexeme, v2[i].lexeme);
        assert_eq!(v1[i]._type, v2[i]._type);
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::*;
    #[test]
    fn test_lexer_colon() {
        let term = ":";
        let v1 = vec![Token {
            _type: TokenType::Colon,
            lexeme: term.to_owned(),
        }];
        let v2 = get_lexemes(term);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexer_commas() {
        let term = ",";
        let v1 = vec![
            Token {
            _type: TokenType::Commas,
            lexeme: term.to_owned(),
        }];
        let v2 = get_lexemes(term);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexer_s_bracket() {
        let term = "[";
        let v1 = vec![Token {
            _type: TokenType::SLBracket,
            lexeme: term.to_owned(),
        }];
        let v2 = get_lexemes(term);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexer_s_r_bracket() {
        let term = "]";
        let v1 = vec![Token {
            _type: TokenType::SRBracket,
            lexeme: term.to_owned(),
        }];
        let v2 = get_lexemes(term);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexer_bracket() {
        let s = "{ }";
        let v1 = vec![
            Token {
                _type: TokenType::LBracket,
                lexeme: "{".to_owned(),
            },
            Token {
                _type: TokenType::RBracket,
                lexeme: "}".to_owned(),
            },
        ];
        let v2 = get_lexemes(s);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexers() {
        let s = r#"{ "key"  : "value" }"#;
        let v1 = vec![
            Token {
                _type: TokenType::LBracket,
                lexeme: "{".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "key".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::Colon,
                lexeme: ":".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "value".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::RBracket,
                lexeme: "}".to_owned(),
            },
        ];
        let v2 = get_lexemes(s);
        compare_tokens(&v1, &v2)
    }
    #[test]
    fn test_lexers_keyword() {
        let s = r#"{"key": [ true, false, null ]  }"#;
        let v1 = vec![
            Token {
                _type: TokenType::LBracket,
                lexeme: "{".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "key".to_owned(),
            },
            Token {
                _type: TokenType::Quote,
                lexeme: "\"".to_owned(),
            },
            Token {
                _type: TokenType::Colon,
                lexeme: ":".to_owned(),
            },
            Token {
                _type: TokenType::SLBracket,
                lexeme: "[".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "true".to_owned(),
            },
            Token {
                _type: TokenType::Commas,
                lexeme: ",".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "false".to_owned(),
            },
            Token {
                _type: TokenType::Commas,
                lexeme: ",".to_owned(),
            },
            Token {
                _type: TokenType::String,
                lexeme: "null".to_owned(),
            },

            Token {
                _type: TokenType::SRBracket,
                lexeme: "]".to_owned(),
            },
            Token {
                _type: TokenType::RBracket,
                lexeme: "}".to_owned(),
            },
        ];
        let v2 = get_lexemes(s);
        compare_tokens(&v1, &v2)
    }
}
