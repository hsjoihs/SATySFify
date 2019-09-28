extern crate libc;
use std::env;
use std::ffi::CStr;

#[link(name = "compile")]
extern "C" {
    fn get_token(initial: *const u8, ptr_offset: *mut usize) -> Token;
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    ALPHANUMERIC,
    BACKSLASH_FOLLOWED_BY_ALPHANUMERICS,
    CARET,
    UNDERSCORE,
    ORDINARY_OPERATOR,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Token {
    kind: TokenType,
    string_representation: *const u8,
}

unsafe fn compile_(input: *mut u8) {
    let init = input;
    let mut box_offset: Box<usize> = Box::new(0);

    let mut tokens = Vec::new();
    loop {
        let ptr = Box::into_raw(box_offset);
        let t = get_token(init, ptr);
        box_offset = Box::from_raw(ptr);
        tokens.push(t);

        if t.kind == TokenType::END {
            break;
        }

        eprintln!(
            "{}",
            CStr::from_ptr(t.string_representation as *const i8)
                .to_str()
                .unwrap()
        );
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
        if t.kind == TokenType::END {
            break;
        }
        println!(
            "      {}",
            CStr::from_ptr(t.string_representation as *const i8)
                .to_str()
                .unwrap()
        )
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
    unsafe { compile_(input.as_mut_ptr()) }
}
