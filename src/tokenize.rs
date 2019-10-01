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
    fn some_char_token(ch: char, kind: TokenType) -> Option<Token> {
        Some(Token {
            kind,
            str_repr: ch.to_string(),
        })
    }
    fn get_token2(iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> Option<Token> {
        match iter.next() {
            None => None,
            Some(&ch) => match ch {
                ' ' | '\t' | '\n' | '\r' => get_token2(iter),
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
                    let after_backslash = *iter
                        .next()
                        .expect("Found unexpected end of input after a backslash\n");

                    if !((after_backslash >= 'a' && after_backslash <= 'z')
                        || (after_backslash >= 'A' && after_backslash <= 'Z'))
                    {
                        eprintln!(
                            "Found unexpected character after a backslash: '{}' ({})\n",
                            after_backslash as char, after_backslash as i32
                        );
                        panic!();
                    }
                    let mut new_st = "\\".to_string();
                    new_st.push(after_backslash);

                    while let Some(&&c) = iter.peek() {
                        if !((c >= 'a' && c <= 'z')
                            || (c >= 'A' && c <= 'Z')
                            || (c >= '0' && c <= '9'))
                        {
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
                    eprintln!(
                        "Found unexpected character: '{}' ({})",
                        ch as char, ch as i32
                    );
                    panic!();
                }
            },
        }
    }
}
