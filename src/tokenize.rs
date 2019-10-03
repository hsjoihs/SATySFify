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
    pub fn to_tokens(input: &str) -> Result<Vec<Token>, String> {
        let mut iter = input.chars().peekable();

        let mut tokens = Vec::new();
        while let Some(t) = get_token2(&mut iter)? {
            eprintln!("{}", t.str_repr);
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
                '_' => some_char_token(ch, TokenType::Underscore),
                'a'..='z' | 'A'..='Z' | '0'..='9' => some_char_token(ch, TokenType::Alphanumeric),
                '+' | '*' | ',' | '.' | '|' | '/' | '-' | '<' | '>' | '=' => {
                    some_char_token(ch, TokenType::OrdinaryOperator)
                }
                '\\' => {
                    let after_backslash = iter
                        .next()
                        .ok_or("Found unexpected end of input after a backslash\n")?;

                    if !((after_backslash >= 'a' && after_backslash <= 'z')
                        || (after_backslash >= 'A' && after_backslash <= 'Z'))
                    {
                        return Err(format!(
                            "Found unexpected character after a backslash: '{}' ({})\n",
                            after_backslash as char, after_backslash as i32
                        ));
                    }
                    let mut new_st = "\\".to_string();
                    new_st.push(after_backslash);

                    while let Some(&c) = iter.peek() {
                        if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
                            break;
                        }
                        new_st.push(c);
                        iter.next();
                    }

                    Some(Token {
                        kind: TokenType::BackslashFollowedByAlphanumerics,
                        str_repr: new_st,
                    })
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
