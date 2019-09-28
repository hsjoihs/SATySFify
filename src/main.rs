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

unsafe fn get_token2(initial: &[u8], ptr_offset: *mut usize) -> Token {
    if initial[*ptr_offset] == b'\0' {
        return Token {
            kind: TokenType::End,
            str_repr: "".to_string(),
        };
    }

    if initial[*ptr_offset] == b' '
        || initial[*ptr_offset] == b'\t'
        || initial[*ptr_offset] == b'\n'
        || initial[*ptr_offset] == b'\r'
    {
        *ptr_offset += 1;
        return get_token2(initial, ptr_offset);
    }

    if initial[*ptr_offset] == b'+' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "+".to_string(),
        };
    } else if initial[*ptr_offset] == b'*' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "*".to_string(),
        };
    } else if initial[*ptr_offset] == b'(' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::LeftParen,
            str_repr: "(".to_string(),
        };
    } else if initial[*ptr_offset] == b')' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::RightParen,
            str_repr: ")".to_string(),
        };
    } else if initial[*ptr_offset] == b',' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ",".to_string(),
        };
    } else if initial[*ptr_offset] == b'^' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::Caret,
            str_repr: "^".to_string(),
        };
    } else if initial[*ptr_offset] == b'{' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::LeftBrace,
            str_repr: "{".to_string(),
        };
    } else if initial[*ptr_offset] == b'}' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::RightBrace,
            str_repr: "}".to_string(),
        };
    } else if initial[*ptr_offset] == b'<' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "<".to_string(),
        };
    } else if initial[*ptr_offset] == b'>' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: ">".to_string(),
        };
    } else if initial[*ptr_offset] == b'=' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::OrdinaryOperator,
            str_repr: "=".to_string(),
        };
    } else if initial[*ptr_offset] == b'_' {
        *ptr_offset += 1;
        return Token {
            kind: TokenType::Underscore,
            str_repr: "_".to_string(),
        };
    }

    if (initial[*ptr_offset] >= b'a' && initial[*ptr_offset] <= b'z')
        || (initial[*ptr_offset] >= b'A' && initial[*ptr_offset] <= b'Z')
        || (initial[*ptr_offset] >= b'0' && initial[*ptr_offset] <= b'9')
    {
        let mut s = Vec::new();
        s.push(initial[*ptr_offset]);
        s.push(b'\0');
        let ptr = s.as_ptr();
        std::mem::forget(s);

        let mut st = String::from("");
        st.push(initial[*ptr_offset] as char);

        *ptr_offset += 1;

        return Token {
            kind: TokenType::Alphanumeric,
            str_repr: st,
        };
    }

    if initial[*ptr_offset] == b'\\' {
        let after_backslash = initial[1 + *ptr_offset];
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
            let c = initial[i + *ptr_offset];
            if !((c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || (c >= b'0' && c <= b'9')) {
                break;
            }
            i += 1;
        }

        /*
            identifier: initial[*ptr_offset + 1] ~ initial[*ptr_offset + i-1]
        */
        let mut new_str = Vec::new();
        let mut new_st = String::from("");
        for j in 0..i {
            new_str.push(initial[*ptr_offset + j]);
            new_st.push(initial[*ptr_offset + j] as char)
        }
        new_str.push(b'\0');
        let ptr = new_str.as_ptr();
        std::mem::forget(new_str);
        *ptr_offset += i;

        return Token {
            kind: TokenType::BackslashFollowedByAlphanumerics,
            str_repr: new_st,
        };
    }

    eprintln!(
        "Found unexpected character: '{}' ({})",
        initial[*ptr_offset] as char, initial[*ptr_offset] as i32
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
            let t = get_token2(initial, ptr);
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
