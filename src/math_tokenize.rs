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
        LeftBracket,
        RightBracket,
        Ampersand,
        DoubleBackslash,
        BackslashBegin,
        BackslashEnd,
    }

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct Token {
        pub kind: TokenType,
        pub str_repr: String,
    }
    pub fn to_tokens(input: &str) -> Result<Vec<Token>, String> {
        let mut iter = input.chars().peekable();

        let mut tokens = Vec::new();
        while let Some(t) = get_token2(&mut iter)? {
            tokens.push(t);
        }

        Ok(tokens)
    }
    fn some_char_token(ch: char, kind: TokenType) -> Option<Token> {
        Some(Token {
            kind,
            str_repr: ch.to_string(),
        })
    }
    fn get_token2(
        iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    ) -> Result<Option<Token>, String> {
        let opt_tok = match iter.next() {
            None => None,
            Some(ch) => match ch {
                ' ' | '\t' | '\n' | '\r' => get_token2(iter)?,
                '(' => some_char_token(ch, TokenType::LeftParen),
                ')' => some_char_token(ch, TokenType::RightParen),
                '^' => some_char_token(ch, TokenType::Caret),
                '{' => some_char_token(ch, TokenType::LeftBrace),
                '}' => some_char_token(ch, TokenType::RightBrace),
                '[' => some_char_token(ch, TokenType::LeftBracket),
                ']' => some_char_token(ch, TokenType::RightBracket),
                '_' => some_char_token(ch, TokenType::Underscore),
                '&' => some_char_token(ch, TokenType::Ampersand),
                'a'..='z' | 'A'..='Z' | '0'..='9' => some_char_token(ch, TokenType::Alphanumeric),
                '\'' => Some(Token {
                    kind: TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\satysfify-internal-prime".to_string(),
                }),
                '+' | '*' | ',' | '.' | '/' | '-' | '<' | '>' | '=' => {
                    some_char_token(ch, TokenType::OrdinaryOperator)
                }
                '|' => Some(Token {
                    kind: TokenType::OrdinaryOperator,
                    str_repr: "\\|".to_string(),
                }),
                '\\' => {
                    let after_backslash = iter
                        .next()
                        .ok_or("Found unexpected end of input after a backslash\n")?;

                    if after_backslash == '\\' {
                        Some(Token {
                            kind: TokenType::DoubleBackslash,
                            str_repr: "\\\\".to_string(),
                        })
                    } else {
                        if !((after_backslash >= 'a' && after_backslash <= 'z')
                            || (after_backslash >= 'A' && after_backslash <= 'Z'))
                        {
                            return Err(format!(
                                "Found unexpected character after a backslash: '{}' ({})\n",
                                after_backslash as char, after_backslash as i32
                            ));
                        }
                        let new_st = {
                            let mut new_st = "\\".to_string();
                            new_st.push(after_backslash);

                            while let Some(&c) = iter.peek() {
                                if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
                                    break;
                                }
                                new_st.push(c);
                                iter.next();
                            }
                            new_st
                        };

                        if new_st == "\\begin" || new_st == "\\end" {
                            // skip spaces
                            while let Some(&c) = iter.peek() {
                                if !c.is_whitespace() {
                                    break;
                                }
                                iter.next();
                            }

                            // expect {
                            match iter.next() {
                                None => {
                                    return Err(format!(
                                        "unexpected end of input after `{}`",
                                        new_st
                                    ))
                                }
                                Some('{') => (),
                                Some(_) => {
                                    return Err(format!(
                                        "expected `{{` after `{}`, but did not find it.",
                                        new_st
                                    ))
                                }
                            }

                            // NO SPACE ALLOWED HERE (WHAT!!!!!!!????????)
                            let command_name = {
                                let mut command_name = String::new();
                                while let Some(&c) = iter.peek() {
                                    if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
                                        break;
                                    }
                                    command_name.push(c);
                                    iter.next();
                                }
                                command_name
                            };
                            // NO SPACE ALLOWED HERE (WHAT!!!!!!!????????)
                            // expect }
                            match iter.next() {
                                None => {
                                    return Err(format!(
                                        "unexpected end of input after `{}`",
                                        new_st
                                    ))
                                }
                                Some('}') => (),
                                Some(_) => {
                                    return Err(format!(
                                        "expected `}}` after `{}{{{}`, but did not find it.",
                                        new_st, command_name
                                    ))
                                }
                            }

                            Some(Token {
                                kind: if new_st == "\\begin" {
                                    TokenType::BackslashBegin
                                } else {
                                    TokenType::BackslashEnd
                                },
                                str_repr: format!("{}{{{}}}", new_st, command_name),
                            })
                        } else {
                            Some(Token {
                                kind: TokenType::BackslashFollowedByAlphanumerics,
                                str_repr: new_st,
                            })
                        }
                    }
                }

                _ => {
                    return Err(format!(
                        "Found unexpected character: '{}' ({})",
                        ch as char, ch as i32
                    ))
                }
            },
        };
        Ok(opt_tok)
    }
}
