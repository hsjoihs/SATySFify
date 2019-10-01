pub mod tok {
    #[repr(C)]
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum TokenType {
        Alphanumeric,
        BackslashFollowedByAlphanumerics,
        Caret,
        Underscore,
        OrdinaryOperator,
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
    }

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct Token {
        pub kind: TokenType,
        pub str_repr: String,
    }
    pub fn to_tokens(input: &[char]) -> Vec<Token> {
        let mut iter = input.iter().peekable();

        let mut tokens = Vec::new();
        loop {
            match get_token2(&mut iter) {
                None => {
                    break;
                }
                Some(t) => {
                    eprintln!("{}", t.str_repr);
                    tokens.push(t);
                }
            }
        }

        tokens
    }
    fn get_token2(iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> Option<Token> {
        match iter.next() {
            None => None,
            Some(&ch) => {
                if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                    return get_token2(iter);
                }

                if ch == '+'
                    || ch == '*'
                    || ch == ','
                    || ch == '.'
                    || ch == '|'
                    || ch == '/'
                    || ch == '-'
                    || ch == '<'
                    || ch == '>'
                    || ch == '='
                {
                    return Some(Token {
                        kind: TokenType::OrdinaryOperator,
                        str_repr: ch.to_string(),
                    });
                } else if ch == '(' {
                    return Some(Token {
                        kind: TokenType::LeftParen,
                        str_repr: ch.to_string(),
                    });
                } else if ch == ')' {
                    return Some(Token {
                        kind: TokenType::RightParen,
                        str_repr: ch.to_string(),
                    });
                } else if ch == '^' {
                    return Some(Token {
                        kind: TokenType::Caret,
                        str_repr: ch.to_string(),
                    });
                } else if ch == '{' {
                    return Some(Token {
                        kind: TokenType::LeftBrace,
                        str_repr: ch.to_string(),
                    });
                } else if ch == '}' {
                    return Some(Token {
                        kind: TokenType::RightBrace,
                        str_repr: ch.to_string(),
                    });
                } else if ch == '_' {
                    return Some(Token {
                        kind: TokenType::Underscore,
                        str_repr: "_".to_string(),
                    });
                }

                if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')
                {
                    return Some(Token {
                        kind: TokenType::Alphanumeric,
                        str_repr: ch.to_string(),
                    });
                }

                if ch == '\\' {
                    match iter.next() {
                        None => {
                            eprintln!("Found unexpected end of input after a backslash\n");
                            panic!();
                        }
                        Some(&after_backslash) => {
                            // now on 1 + *offset

                            if !((after_backslash >= 'a' && after_backslash <= 'z')
                                || (after_backslash >= 'A' && after_backslash <= 'Z'))
                            {
                                eprintln!(
                                    "Found unexpected character after a backslash: '{}' ({})\n",
                                    after_backslash as char, after_backslash as i32
                                );
                                panic!();
                            }
                            let mut new_st = String::from("");
                            new_st.push(ch);
                            new_st.push(after_backslash);

                            loop {
                                match iter.peek() {
                                    None => {
                                        break;
                                    }
                                    Some(&&c) => {
                                        if !((c >= 'a' && c <= 'z')
                                            || (c >= 'A' && c <= 'Z')
                                            || (c >= '0' && c <= '9'))
                                        {
                                            break;
                                        }
                                        new_st.push(c);
                                        iter.next();
                                    }
                                }
                            }

                            return Some(Token {
                                kind: TokenType::BackslashFollowedByAlphanumerics,
                                str_repr: new_st,
                            });
                        }
                    }
                }

                eprintln!(
                    "Found unexpected character: '{}' ({})",
                    ch as char, ch as i32
                );
                panic!();
            }
        }
    }
}
