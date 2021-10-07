use std::vec::Vec;
#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub s: &'a [u8],
    pub start: usize, // start position
    pub _type: u8,
}
/// b'n' -> null
/// b'u' -> number
/// b's' -> string in quote
/// b't' -> true of false

/// use DFA to produce the tokens from the string s.
pub fn generate_tokens(s: &str) -> Vec<Token<'_>> {
    if s.len() == 0 {
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
            b'{' | b'}' | b'[' | b']' | b':' | b',' => {
                // add delimiter token
                let token = Token {
                    s: &bytes[i..i + 1],
                    start: i,
                    _type: bytes[i],
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

    return tokens;
}

fn add_quoted_string<'a, 'b>(bytes: &'a [u8], i: usize, tokens: &'b mut Vec<Token<'a>>) -> usize  {
    let mut i = i;
    let token = Token {
        s: &bytes[i..i+1],
        start: i,
        _type: b'"',
    };
    tokens.push(token);
    if i + 1 >= bytes.len() {
        return i+1
    }
    if let Some((start, end)) = get_string_in_quote(bytes, i + 1) {
        let token = Token {
            s: &bytes[start..end],
            start,
            _type: b's',
        };
        tokens.push(token);
        if end < bytes.len() && bytes[end] == b'"' {
            let token = Token {
                s: &bytes[end..end + 1],
                start: end,
                _type: b'"',
            };
            tokens.push(token);
            i = end + 1;
        } else {
            i = end;
        }
    } else {
        i = i+1;
    }
    i 
}
fn add_keyword_or_number<'a, 'b>(bytes: &'a [u8], i: usize, tokens: &'b mut Vec<Token<'a>>) -> usize {
    if let Some((start, end)) = get_string(bytes, i) {
        let b = &bytes[start..end];
        if let Ok(s) = std::str::from_utf8(b) {
            let mut token;
            if s == "null" {
                token = Token {
                    s: &bytes[start..end],
                    start: start,
                    _type: b'n',
                };
                tokens.push(token);

            }
            if s == "false" || s == "true" {
                token = Token {
                    s: &bytes[start..end],
                    start: start,
                    _type: b't',
                };
                tokens.push(token);

            }
            if b[0].is_ascii_digit() {
                token = Token {
                    s: &bytes[start..end],
                    start: start,
                    _type: b'u',
                };
                tokens.push(token);

            }
        }
        end
    } else {
        i
    }
}
fn get_string_in_quote(s: &[u8], i: usize) -> Option<(usize, usize)> {
    if s.len() <= i {
        return None; // we are at the end of the string
    }
    let mut j = i;
    loop {
        if s[j] == b'"' || j >= s.len() {
            break;
        }
        j += 1;
    }
    Some((i, j))
}
fn get_string(bytes: &[u8], i: usize) -> Option<(usize, usize)> {
    if bytes.len() <= i {
        return None;
    }
    let mut j = i;
    let mut end = j;
    loop {
        if j >= bytes.len() {
            break;
        }
        match bytes[j] {
            b'{' | b'}' | b'[' | b']' | b':' | b',' | b'"' | b' ' | b'\t' | b'\n' | b'\r' => {
                end = j;
                break;
            }
            _ => {
                j += 1;
            }
        }
    }
    Some((i, end))
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
                _type: t,
            }];
            compare_tokens(&res, &exp);
        }
        {
            let res = generate_tokens("{}");
            let exp = vec![
                Token {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Token {
                    s: &[b'}'],
                    start: 1,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'}'],
                    start: 6,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'}'],
                    start: 13,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Token {
                    s: &[b']'],
                    start: 2,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 3,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 3,
                    _type: b'[',
                },
                Token {
                    s: &[b']'],
                    start: 4,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 5,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 3,
                    _type: b'[',
                },
                Token {
                    s: &[b']'],
                    start: 8,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 9,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: b't',
                },
                Token {
                    s: &[b']'],
                    start: 6,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 7,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: b't',
                },
                Token {
                    s: &[b','],
                    start: 6,
                    _type: b',',
                },
                Token {
                    s: &[b'f', b'a', b'l', b's', b'e'],
                    start: 8,
                    _type: b't',
                },
                Token {
                    s: &[b']'],
                    start: 13,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 14,
                    _type: b'}',
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
                    _type: b'{',
                },
                Token {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Token {
                    s: &[b'"'],
                    start: 2,
                    _type: b'"',
                },
                Token {
                    s: &[b'k', b'1'],
                    start: 3,
                    _type: b's',
                },
                Token {
                    s: &[b'"'],
                    start: 5,
                    _type: b'"',
                },
                Token {
                    s: &[b':'],
                    start: 6,
                    _type: b':',
                },
                Token {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 7,
                    _type: b't',
                },
                Token {
                    s: &[b']'],
                    start: 11,
                    _type: b']',
                },
                Token {
                    s: &[b'}'],
                    start: 12,
                    _type: b'}',
                },
            ];
            compare_tokens(&res, &exp);
        }
    }
}
