extern crate libc;
extern crate termion;

use std::env;

mod math;
mod math_tokenize;

pub use crate::math_tokenize::tok;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Incorrect number of arguments\n".to_string());
    }
    print_satysfi(
        &["stdjabook", "code", "itemize", "tabular", "math"],
        &args[1],
        6,
    )
}

fn print_satysfi(libs: &[&str], input: &str, indent: usize) -> Result<(), String> {
    let tokens = tok::to_tokens(&input)?;
    let math = math::to_math(tokens)?;

    for lib in libs {
        println!("@require: {}", lib);
    }
    println!();

    for code in &math::activated_math_addons(&math) {
        println!("{}", code);
    }

    println!("document (|");
    println!("  title = {{}};");
    println!("  author = {{}};");
    println!("  show-title = false;");
    println!("  show-toc = false;");
    println!("|) '<");
    println!("  +section{{}}<");
    println!("    +math(${{");

    math::print_math(&math.stuffs, indent);

    println!("    }});");
    println!("  >");
    println!(">");

    Ok(())
}
