extern crate libc;
use std::env;
use std::ffi::CStr;

#[link(name = "compile")]
extern "C" {}

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
    kind: TokenType,
    str_repr: String,
}

fn get_token2(initial: &[u8], offset: &mut usize) -> Token {
    if initial[*offset] == b'\0' {
        return Token {
            kind: TokenType::End,
            str_repr: "".to_string(),
        };
    }

    if initial[*offset] == b' '
        || initial[*offset] == b'\t'
        || initial[*offset] == b'\n'
        || initial[*offset] == b'\r'
    {
        *offset += 1;
        return get_token2(initial, offset);
    }

    if initial[*offset] == b'+' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "+".to_string(),
        };
    } else if initial[*offset] == b'*' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "*".to_string(),
        };
    } else if initial[*offset] == b'(' {
        *offset += 1;
        return Token {
            kind: TokenType::LeftParen,
            str_repr: "(".to_string(),
        };
    } else if initial[*offset] == b')' {
        *offset += 1;
        return Token {
            kind: TokenType::RightParen,
            str_repr: ")".to_string(),
        };
    } else if initial[*offset] == b',' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ",".to_string(),
        };
    } else if initial[*offset] == b'^' {
        *offset += 1;
        return Token {
            kind: TokenType::Caret,
            str_repr: "^".to_string(),
        };
    } else if initial[*offset] == b'{' {
        *offset += 1;
        return Token {
            kind: TokenType::LeftBrace,
            str_repr: "{".to_string(),
        };
    } else if initial[*offset] == b'}' {
        *offset += 1;
        return Token {
            kind: TokenType::RightBrace,
            str_repr: "}".to_string(),
        };
    } else if initial[*offset] == b'<' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "<".to_string(),
        };
    } else if initial[*offset] == b'>' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ">".to_string(),
        };
    } else if initial[*offset] == b'=' {
        *offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "=".to_string(),
        };
    } else if initial[*offset] == b'_' {
        *offset += 1;
        return Token {
            kind: TokenType::Underscore,
            str_repr: "_".to_string(),
        };
    }

    if (initial[*offset] >= b'a' && initial[*offset] <= b'z')
        || (initial[*offset] >= b'A' && initial[*offset] <= b'Z')
        || (initial[*offset] >= b'0' && initial[*offset] <= b'9')
    {
        let mut st = String::from("");
        st.push(initial[*offset] as char);

        *offset += 1;

        return Token {
            kind: TokenType::Alphanumeric,
            str_repr: st,
        };
    }

    if initial[*offset] == b'\\' {
        let after_backslash = initial[1 + *offset];
        if !((after_backslash >= b'a' && after_backslash <= b'z')
            || (after_backslash >= b'A' && after_backslash <= b'Z'))
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
            if !((c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || (c >= b'0' && c <= b'9')) {
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

fn compile_(input: &mut str) {
    let init = input.as_mut_ptr();
    let mut box_offset: Box<usize> = Box::new(0);

    let mut tokens = Vec::new();
    loop {
        let ptr = Box::into_raw(box_offset);
        unsafe {
            let initial = CStr::from_ptr(init as *const i8).to_bytes_with_nul();
            let t = get_token2(initial, &mut *ptr);
            box_offset = Box::from_raw(ptr);

            if t.kind == TokenType::End {
                tokens.push(t);
                break;
            } else {
                let st = t.str_repr.clone();
                tokens.push(t);
                eprintln!("{}", st);
            }
        }
    }

    print!("{}", "@require: stdjabook\n");
    print!("{}", "@require: code\n");
    print!("{}", "@require: itemize\n");
    print!("{}", "@require: tabular\n");
    print!("{}", "@require: math\n");
    print!("{}", "\n");
    print!("{}", "document (|\n");
    print!("{}", "  title = {};\n");
    print!("{}", "  author = {};\n");
    print!("{}", "  show-title = false;\n");
    print!("{}", "  show-toc = false;\n");
    print!("{}", "|) '<\n");
    print!("{}", "  +section{}<\n");
    print!("{}", "    +math(${\n");

    for t in &tokens {
        if t.kind == TokenType::End {
            break;
        }
        println!("      {}", t.str_repr)
    }

    print!("{}", "    });\n");
    print!("{}", "  >\n");
    print!("{}", ">\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let mut input = args[1].clone();
    compile_(&mut input)
}
