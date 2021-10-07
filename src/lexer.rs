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
                i = add_delimiter_token(bytes, i, &mut tokens);
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

// input `start` is the next character to process.
// return the index of the next character to process.
fn add_quoted_string<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    tokens: &'b mut Vec<Token<'a>>,
) -> usize {
    let mut start = add_quote_token(bytes, start, tokens);
    start = get_string_in_quote(bytes, start, tokens);
    add_quote_token(bytes, start, tokens)
}

fn add_quote_token<'a, 'b>(bytes: &'a [u8], start: usize, tokens: &'b mut Vec<Token<'a>>) -> usize {
    if start < bytes.len() && bytes[start] == b'"' {
        let token = Token {
            s: &bytes[start..start + 1],
            start,
            _type: TokenType::Quote,
        };
        tokens.push(token);
        return start + 1;
    }
    start
}

fn is_delimiters(c: u8) -> bool {
    c == b'{' || c == b'}' || c == b'[' || c == b']' || c == b':' || c == b','
}

// add delimiter token
fn add_delimiter_token<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    tokens: &'b mut Vec<Token<'a>>,
) -> usize {
    if start >= bytes.len() {
        return start;
    }
    let token = Token {
        s: &bytes[start..start + 1],
        start,
        _type: get_token_type(bytes[start]),
    };
    tokens.push(token);
    start + 1
}

fn add_keyword_or_number<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    tokens: &'b mut Vec<Token<'a>>,
) -> usize {
    if start >= bytes.len() {
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
    } else {
        let s = std::str::from_utf8(b).unwrap(); // let's panic if unsupported character is met.
        match s {
            "null" => {
                add_null_token(bytes, start, "null".len(), tokens);
            }
            "false" => {
                add_boolean_token(bytes, start, "false".len(), tokens);
            }
            "true" => {
                add_boolean_token(bytes, start, "true".len(), tokens);
            }
            _ => {
                panic!("Unsupported keyword or number.");
            }
        }
    }
    end
}
fn add_null_token<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    length: usize,
    tokens: &'b mut Vec<Token<'a>>,
) {
    let token = Token {
        s: &bytes[start..start + length],
        start,
        _type: TokenType::Null,
    };
    tokens.push(token);
}
fn add_boolean_token<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    length: usize,
    tokens: &'b mut Vec<Token<'a>>,
) {
    let token = Token {
        s: &bytes[start..start + length],
        start,
        _type: TokenType::Boolean,
    };
    tokens.push(token);
}

fn get_string_in_quote<'a, 'b>(
    bytes: &'a [u8],
    start: usize,
    tokens: &'b mut Vec<Token<'a>>,
) -> usize {
    if start >= bytes.len() {
        return start;
    }
    let mut iter = bytes[start..].split_inclusive(|&c| c == b'"');
    let length = iter.next().unwrap().len();
    let token = Token {
        s: &bytes[start..(start + length - 1)],
        start,
        _type: TokenType::String,
    };
    tokens.push(token);
    start + length - 1
}

#[cfg(test)]
mod test {
    use super::*;
    fn compare_tokens(left: &Vec<Token<'_>>, right: &Vec<Token<'_>>) {
        assert_eq!(left.len(), right.len());
        for i in 0..left.len() {
            assert_eq!(left[i], right[i]);
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
