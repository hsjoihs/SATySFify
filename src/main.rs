extern crate libc;
use std::env;

#[link(name = "compile")]
extern "C" {}

mod tokenize;

pub use crate::tokenize::tok;

fn compile_(input: &[char]) {
    let tokens = tok::to_tokens(input);

    for lib in vec!["stdjabook", "code", "itemize", "tabular", "math"] {
        println!("@require: {}", lib);
    }
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
        if t.kind == tok::TokenType::End {
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
    let mut input: Vec<char> = args[1].clone().chars().collect();
    compile_(&mut input)
}
