use std::collections::HashMap;
use std::vec::Vec;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Null,
    Number,
    String,
    Quote,
    Boolean,
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Colon,
    Comma,
}

#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub s: &'a [u8],
    pub start: usize, // start position
    pub _type: TokenType,
}

fn get_token_type(b: u8) -> TokenType {
    let mut delimiter_map = HashMap::new();
    delimiter_map.insert(b'{', TokenType::LeftBracket);
    delimiter_map.insert(b'}', TokenType::RightBracket);
    delimiter_map.insert(b'[', TokenType::LeftSquareBracket);
    delimiter_map.insert(b']', TokenType::RightSquareBracket);
    delimiter_map.insert(b':', TokenType::Colon);
    delimiter_map.insert(b',', TokenType::Comma);
    *delimiter_map.get(&b).unwrap()
}

/// use DFA to produce the tokens from the string s.
///
pub fn generate_tokens(s: &str) -> Vec<Token<'_>> {
    if s.is_empty() {
        return vec![];
    }

    let bytes = s.as_bytes();
    let mut tokens = vec![];
    let mut i = 0;
    loop {
        if i >= bytes.len() {
            break;
        }
        match bytes[i] {
            b'"' => {
                i = add_quoted_string(bytes, i, &mut tokens);
            }
            c if is_delimiters(c) => {
                // add delimiter token
                let token = Token {
                    s: &bytes[i..i + 1],
                    start: i,
                    _type: get_token_type(bytes[i]),
                };
                tokens.push(token);
                i += 1;
            }
            c if c.is_ascii_whitespace() => {
                i += 1;
            }
            _ => {
                i = add_keyword_or_number(bytes, i, &mut tokens);
            }
        }
    }

    tokens
}
// input `i` is the next character to process.
// return the index of the next character to process.
fn add_quoted_string<'a, 'b>(bytes: &'a [u8], i: usize, tokens: &'b mut Vec<Token<'a>>) -> usize {
    let token = Token {
        s: &bytes[i..i + 1],
        start: i,
        _type: TokenType::Quote,
    };
    tokens.push(token);

    let mut i = i + 1;
    if i < bytes.len() {
        i = get_string_in_quote(bytes, i, tokens);
        i = add_quote_token(bytes, i, tokens);
    }
    i
}

fn add_quote_token<'a, 'b>(bytes: &'a [u8], i: usize, tokens: &'b mut Vec<Token<'a>>) -> usize {
    if i < bytes.len() && bytes[i] == b'"' {
        let token = Token {
            s: &bytes[i..i + 1],
            start: i,
            _type: TokenType::Quote,
        };
        tokens.push(token);
        i + 1
    } else {
        i
    }
}
fn is_delimiters(c: u8) -> bool {
    c == b'{' || c == b'}' || c == b'[' || c == b']' || c == b':' || c == b','
}
fn add_keyword_or_number<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    tokens: &'b mut Vec<Token<'a>>,
) -> usize {
    if bytes.len() <= start {
        return start;
    }
    let mut iter = bytes[start..].split_inclusive(|&c| c.is_ascii_whitespace() || is_delimiters(c));
    let end = start + iter.next().unwrap().len() - 1;
    let b = &bytes[start..end];

    if b[0].is_ascii_digit() {
        let token = Token {
            s: &bytes[start..end],
            start,
            _type: TokenType::Number,
        };
        tokens.push(token);
    } else if let Ok(s) = std::str::from_utf8(b) {
            match s {
                "null" => {
                    let token = Token {
                        s: &bytes[start..end],
                        start,
                        _type: TokenType::Null,
                    };
                    tokens.push(token);
                }
                "false" | "true" => {
                    let token = Token {
                        s: &bytes[start..end],
                        start,
                        _type: TokenType::Boolean,
                    };
                    tokens.push(token);
                }
                _ => {
                    panic!("Unsupported keyword or number.");
                }
            }
    }
    end
}
fn get_string_in_quote<'a, 'b>(bytes: &'a [u8], i: usize, tokens: &'b mut Vec<Token<'a>>) -> usize {
    let mut iter = bytes[i..].split_inclusive(|&c| c == b'"');
    let length = iter.next().unwrap().len();
    let token = Token {
        s: &bytes[i..(i + length - 1)],
        start: i,
        _type: TokenType::String,
    };
    tokens.push(token);
    i + length - 1
}

#[cfg(test)]
mod test {
    use super::*;
    fn compare_tokens(lft: &Vec<Token<'_>>, rght: &Vec<Token<'_>>) {
        assert_eq!(lft.len(), rght.len());
        for i in 0..lft.len() {
            assert_eq!(lft[i], rght[i]);
        }
    }
    #[test]
    fn test_tokenize() {
        println!("Testing.");
        for &t in &[b'{', b'}', b'[', b']', b':', b','] {
            let bytes = &[t];
            let s = std::str::from_utf8(bytes).unwrap();
            let res = generate_tokens(s);
            let exp = vec![Token {
                s: bytes,
                start: 0,
                _type: get_token_type(t),
            }];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 1,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{     }");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 6,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }

        {
            let res = generate_tokens("       {     }");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 7,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 13,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }

        {
            let res = generate_tokens("{[]}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b']'],
                    start: 2,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 3,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }

        {
            let res = generate_tokens("{  []}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 3,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b']'],
                    start: 4,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 5,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{  [    ]}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 3,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b']'],
                    start: 8,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 9,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }

        {
            let res = generate_tokens("{[true]}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: TokenType::Boolean,
                },
                Token {
                    s: &[b']'],
                    start: 6,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 7,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{[true, false]}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: TokenType::Boolean,
                },
                Token {
                    s: &[b','],
                    start: 6,
                    _type: TokenType::Comma,
                },
                Token {
                    s: &[b'f', b'a', b'l', b's', b'e'],
                    start: 8,
                    _type: TokenType::Boolean,
                },
                Token {
                    s: &[b']'],
                    start: 13,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 14,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{[\"k1\":true]}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: TokenType::LeftSquareBracket,
                },
                Token {
                    s: &[b'"'],
                    start: 2,
                    _type: TokenType::Quote,
                },
                Token {
                    s: &[b'k', b'1'],
                    start: 3,
                    _type: TokenType::String,
                },
                Token {
                    s: &[b'"'],
                    start: 5,
                    _type: TokenType::Quote,
                },
                Token {
                    s: &[b':'],
                    start: 6,
                    _type: TokenType::Colon,
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 7,
                    _type: TokenType::Boolean,
                },
                Token {
                    s: &[b']'],
                    start: 11,
                    _type: TokenType::RightSquareBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 12,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
    }

    #[test]
    fn test_tokenize_split_inclusive() {
        {
            let res = generate_tokens(r#"""#);
            let exp = vec![Token {
                s: &[b'"'],
                start: 0,
                _type: TokenType::Quote,
            }];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{     }");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: TokenType::LeftBracket,
                },
                Token {
                    s: &[b'}'],
                    start: 6,
                    _type: TokenType::RightBracket,
                },
            ];
            compare_tokens(&res, &exp);
        }
    }
}
