extern crate libc;
extern crate termion;

use std::collections::HashMap;
use std::env;

mod math;
mod math_tokenize;

pub use crate::math_tokenize::tok;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Incorrect number of arguments\n".to_string());
    }
    print(&args[1], 6)
}

fn print(input: &str, indent: usize) -> Result<(), String> {
    let tokens = tok::to_tokens(&input)?;
    let math = math::to_math(tokens)?;

    for lib in &["stdjabook", "code", "itemize", "tabular", "math"] {
        println!("@require: {}", lib);
    }
    println!();

    let addon_defs: HashMap<String, &str> = [(
        "\\hbar".to_string(),
        "let-math \\hbar = math-char MathOrd `ℏ` in ",
    ), ("\\satysfifi-internal-paren-left-sqbracket-right".to_string(),
     "let-math \\satysfifi-internal-paren-left-sqbracket-right  = math-paren Math.paren-left Math.sqbracket-right in "
    ), ("\\satysfifi-internal-sqbracket-left-paren-right".to_string(),
     "let-math \\satysfifi-internal-sqbracket-left-paren-right  = math-paren Math.sqbracket-left Math.paren-right in "
    )]
    .iter()
    .cloned()
    .collect();

    let what_to_activate = math::get_what_to_activate_(&math);

    let mut activated_addons = Vec::new();

    for (key, code) in &addon_defs {
        if what_to_activate.get(key).is_some() {
            activated_addons.push(code);
        }
    }

    for code in &activated_addons {
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

    math::print_expr_(&math.stuffs, indent);
    eprintln!("{:?}", math.stuffs);

    println!("    }});");
    println!("  >");
    println!(">");

    Ok(())
}
