pub mod tok {
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
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
    End,
}

#[repr(C)]
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub str_repr: String,
}

pub fn get_token2(initial: &[char], offset: &mut usize) -> Token {
    if initial.len() == *offset {
        return Token {
            kind: TokenType::End,
            str_repr: "".to_string(),
        };
    }

    if initial[*offset] == ' '
        || initial[*offset] == '\t'
        || initial[*offset] == '\n'
        || initial[*offset] == '\r'
    {
        *offset += 1;
        return get_token2(initial, offset);
    }

    if initial[*offset] == '+' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "+".to_string(),
        };
    } else if initial[*offset] == '*' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "*".to_string(),
        };
    } else if initial[*offset] == '(' {
        *offset += 1;
        return Token {
            kind: TokenType::LeftParen,
            str_repr: "(".to_string(),
        };
    } else if initial[*offset] == ')' {
        *offset += 1;
        return Token {
            kind: TokenType::RightParen,
            str_repr: ")".to_string(),
        };
    } else if initial[*offset] == ',' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ",".to_string(),
        };
    } else if initial[*offset] == '^' {
        *offset += 1;
        return Token {
            kind: TokenType::Caret,
            str_repr: "^".to_string(),
        };
    } else if initial[*offset] == '{' {
        *offset += 1;
        return Token {
            kind: TokenType::LeftBrace,
            str_repr: "{".to_string(),
        };
    } else if initial[*offset] == '}' {
        *offset += 1;
        return Token {
            kind: TokenType::RightBrace,
            str_repr: "}".to_string(),
        };
    } else if initial[*offset] == '<' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "<".to_string(),
        };
    } else if initial[*offset] == '>' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ">".to_string(),
        };
    } else if initial[*offset] == '=' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "=".to_string(),
        };
    } else if initial[*offset] == '_' {
        *offset += 1;
        return Token {
            kind: TokenType::Underscore,
            str_repr: "_".to_string(),
        };
    }

    if (initial[*offset] >= 'a' && initial[*offset] <= 'z')
        || (initial[*offset] >= 'A' && initial[*offset] <= 'Z')
        || (initial[*offset] >= '0' && initial[*offset] <= '9')
    {
        let mut st = String::from("");
        st.push(initial[*offset] as char);

        *offset += 1;

        return Token {
            kind: TokenType::Alphanumeric,
            str_repr: st,
        };
    }

    if initial[*offset] == '\\' {
        let after_backslash = initial[1 + *offset];
        if !((after_backslash >= 'a' && after_backslash <= 'z')
            || (after_backslash >= 'A' && after_backslash <= 'Z'))
        {
            eprintln!(
                "Found unexpected character after a backslash: '{}' ({})\n",
                after_backslash as char, after_backslash as i32
            );
            panic!();
        }

        let mut i = 2;

        loop {
            let c = initial[i + *offset];
            if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')) {
                break;
            }
            i += 1;
        }

        /*
            identifier: initial[offset + 1] ~ initial[offset + i-1]
        */
        let mut new_st = String::from("");
        for j in 0..i {
            new_st.push(initial[*offset + j] as char)
        }
        *offset += i;

        return Token {
            kind: TokenType::BackslashFollowedByAlphanumerics,
            str_repr: new_st,
        };
    }

    eprintln!(
        "Found unexpected character: '{}' ({})",
        initial[*offset] as char, initial[*offset] as i32
    );
    panic!();
}
}