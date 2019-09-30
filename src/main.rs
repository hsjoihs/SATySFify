extern crate libc;
use std::env;

#[link(name = "compile")]
extern "C" {}

mod tokenize;

pub use crate::tokenize::tok;

fn compile_(input: &[char]) {
    let tokens = tok::to_tokens(input);

    for lib in &["stdjabook", "code", "itemize", "tabular", "math"] {
        println!("@require: {}", lib);
    }
    println!();
    println!("document (|");
    println!("  title = {{}};");
    println!("  author = {{}};");
    println!("  show-title = false;");
    println!("  show-toc = false;");
    println!("|) '<");
    println!("  +section{{}}<");
    println!("    +math(${{");

    for t in &tokens {
        if t.kind == tok::TokenType::End {
            break;
        }
        println!("      {}", t.str_repr)
    }

    println!("    }});");
    println!("  >");
    println!(">");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let input: Vec<char> = args[1].clone().chars().collect();
    compile_(&input)
}
