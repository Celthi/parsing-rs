use std::vec::Vec;
#[derive(PartialEq, Debug)]
pub struct Lexeme<'a> {
    pub s: &'a [u8],
    pub start: usize, // start position
    pub _type: u8,
}
/// b'n' -> null
/// b'u' -> number
/// b's' -> string in quote
/// b't' -> true of false
/// b'k' -> keyword

/// use DFA to produce the lexemes from the string s.
pub fn gen_lexemes(s: &str) -> Vec<Lexeme<'_>> {
    if s.len() == 0 {
        return vec![];
    }
    let bytes = s.as_bytes();
    let mut lexemes = vec![];
    let mut i = 0;
    loop {
        if i >= bytes.len() {
            break;
        }
        match bytes[i] {
            b'"' => {
                let lexeme = Lexeme {
                    s: &bytes[i..i + 1],
                    start: i,
                    _type: b'"',
                };
                lexemes.push(lexeme);
                if i + 1 >= bytes.len() {
                    break;
                }
                let (start, end) = get_string_in_quote(bytes, i + 1);
                if end > start {
                    let lexeme = Lexeme {
                        s: &bytes[start..end],
                        start,
                        _type: b's',
                    };
                    lexemes.push(lexeme);
                    if end < s.len() && bytes[end] == b'"' {
                        let lexeme = Lexeme {
                            s: &bytes[end..end + 1],
                            start: end,
                            _type: b'"',
                        };
                        lexemes.push(lexeme);
                        i = end + 1;
                    } else {
                        i = end;
                    }
                } else {
                    i = end;
                }
            }
            b'{' | b'}' | b'[' | b']' | b':' | b',' => {
                let lexeme = Lexeme {
                    s: &bytes[i..i + 1],
                    start: i,
                    _type: bytes[i],
                };
                lexemes.push(lexeme);
                i += 1;
            }
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1;
            }
            _ => {
                let (start, end) = get_string(bytes, i);
                if end > start {
                    let b = &bytes[start..end];
                    if let Ok(s) = std::str::from_utf8(b) {
                        let mut lexeme;
                        if s == "null" {
                            lexeme = Lexeme {
                                s: &bytes[start..end],
                                start: start,
                                _type: b'n',
                            };
                            lexemes.push(lexeme);

                        }
                        if s == "false" || s == "true" {
                            lexeme = Lexeme {
                                s: &bytes[start..end],
                                start: start,
                                _type: b't',
                            };
                            lexemes.push(lexeme);

                        }
                        if b[0] == b'0' || b[0] == b'1' || b[0] == b'2' || b[0] == b'3' || b[0] == b'4' || b[0] == b'5' || b[0] == b'6' || b[0] == b'7' || b[0] == b'8' || b[0] == b'9' {
                            lexeme = Lexeme {
                                s: &bytes[start..end],
                                start: start,
                                _type: b'u',
                            };
                            lexemes.push(lexeme);

                        }
                    }
                }
                i = end;
            }
        }
    }

    return lexemes;
}

fn get_string_in_quote(s: &[u8], i: usize) -> (usize, usize) {
    if s.len() <= i {
        return (i + 5, i + 1); // we are at the end of the string
    }
    let mut j = i;
    loop {
        if s[j] == b'"' || j >= s.len() {
            break;
        }
        j += 1;
    }
    return (i, j);
}
fn get_string(bytes: &[u8], i: usize) -> (usize, usize) {
    if bytes.len() <= i {
        return (i + 5, i);
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
    (i, end)
}
#[cfg(test)]
mod test {
    use super::*;
    fn compare_lexemes(lft: &Vec<Lexeme<'_>>, rght: &Vec<Lexeme<'_>>) {
        assert_eq!(lft.len(), rght.len());
        for i in 0..lft.len() {
            assert_eq!(lft[i], rght[i]);
        }
    }
    #[test]
    fn test_lexemes() {
        println!("Testing.");
        for &t in &[b'{', b'}', b'[', b']', b':', b','] {
            let bytes = &[t];
            let s = std::str::from_utf8(bytes).unwrap();
            let res = gen_lexemes(s);
            let exp = vec![Lexeme {
                s: bytes,
                start: 0,
                _type: t,
            }];
            compare_lexemes(&res, &exp);
        }
        {
            let res = gen_lexemes("{}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 1,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }
        {
            let res = gen_lexemes("{     }");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 6,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }

        {
            let res = gen_lexemes("       {     }");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 7,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 13,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }

        {
            let res = gen_lexemes("{[]}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b']'],
                    start: 2,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 3,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }

        {
            let res = gen_lexemes("{  []}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 3,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b']'],
                    start: 4,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 5,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }
        {
            let res = gen_lexemes("{  [    ]}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 3,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b']'],
                    start: 8,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 9,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }

        {
            let res = gen_lexemes("{[true]}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: b't',
                },
                Lexeme {
                    s: &[b']'],
                    start: 6,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 7,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }
        {
            let res = gen_lexemes("{[true, false]}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 2,
                    _type: b't',
                },
                Lexeme {
                    s: &[b','],
                    start: 6,
                    _type: b',',
                },
                Lexeme {
                    s: &[b'f', b'a', b'l', b's', b'e'],
                    start: 8,
                    _type: b't',
                },
                Lexeme {
                    s: &[b']'],
                    start: 13,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 14,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }
        {
            let res = gen_lexemes("{[\"k1\":true]}");
            let exp = vec![
                Lexeme {
                    s: &[b'{'],
                    start: 0,
                    _type: b'{',
                },
                Lexeme {
                    s: &[b'['],
                    start: 1,
                    _type: b'[',
                },
                Lexeme {
                    s: &[b'"'],
                    start: 2,
                    _type: b'"',
                },
                Lexeme {
                    s: &[b'k', b'1'],
                    start: 3,
                    _type: b's',
                },
                Lexeme {
                    s: &[b'"'],
                    start: 5,
                    _type: b'"',
                },
                Lexeme {
                    s: &[b':'],
                    start: 6,
                    _type: b':',
                },
                Lexeme {
                    s: &[b't', b'r', b'u', b'e'],
                    start: 7,
                    _type: b't',
                },
                Lexeme {
                    s: &[b']'],
                    start: 11,
                    _type: b']',
                },
                Lexeme {
                    s: &[b'}'],
                    start: 12,
                    _type: b'}',
                },
            ];
            compare_lexemes(&res, &exp);
        }
    }
}
