extern crate libc;
use std::env;

#[link(name = "compile")]
extern "C" {}

mod tokenize;

pub use crate::tokenize::tok;

#[derive(Debug)]
enum Stuff {
    Simple(tok::Token),
    Braced(Vec<Stuff>),
}

#[derive(Copy, Clone)]
enum ParenKind {
    BareLeftParen,
    BareLeftBrace,
}

fn to_stuffs(input: Vec<tok::Token>) -> Vec<Stuff> {
    to_stuffs_(&mut input.into_iter(), Vec::new())
}

fn to_stuffs_(
    mut iter: &mut std::vec::IntoIter<tok::Token>,
    paren_stack: Vec<ParenKind>,
) -> Vec<Stuff> {
    let mut res = Vec::new();

    while let Some(x) = iter.next() {
        match x.kind {
            tok::TokenType::Alphanumeric => {
                res.push(Stuff::Simple(x));
            }
            tok::TokenType::OrdinaryOperator => {
                res.push(Stuff::Simple(x));
            }
            tok::TokenType::Underscore => {
                res.push(Stuff::Simple(x));
            }
            tok::TokenType::Caret => {
                res.push(Stuff::Simple(x));
            }
            tok::TokenType::BackslashFollowedByAlphanumerics => {
                res.push(Stuff::Simple(x));
            }

            tok::TokenType::LeftParen => {
                let mut new_stack = paren_stack.clone();
                new_stack.push(ParenKind::BareLeftParen);
                let inner_stuffs = to_stuffs_(&mut iter, new_stack);
                res.push(Stuff::Simple(tok::Token {
                    kind: tok::TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\paren".to_string(),
                }));
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::LeftBrace => {
                let mut new_stack = paren_stack.clone();
                new_stack.push(ParenKind::BareLeftBrace);
                let inner_stuffs = to_stuffs_(&mut iter, new_stack);
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::RightBrace => match paren_stack.last() {
                None => panic!("unmatched left brace"),
                Some(ParenKind::BareLeftParen) => {
                    panic!("right paren encountered before a left brace was matched")
                }
                Some(ParenKind::BareLeftBrace) => {
                    return res;
                }
            },
            tok::TokenType::RightParen => match paren_stack.last() {
                None => panic!("unmatched left paren"),
                Some(ParenKind::BareLeftBrace) => {
                    panic!("right brace encountered before a left paren was matched")
                }
                Some(ParenKind::BareLeftParen) => {
                    return res;
                }
            },
        };
    }

    res
}

fn print_expr_(stuffs: &[Stuff], indent: usize) {
    for st in stuffs {
        match st {
            Stuff::Simple(t) => {
                println!("{:indent$}{}", "", t.str_repr, indent = indent);
            }
            Stuff::Braced(vec) => {
                println!("{:indent$}{}", "", "{", indent = indent);
                print_expr_(vec, indent + 2);
                println!("{:indent$}{}", "", "}", indent = indent);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let input: Vec<char> = args[1].clone().chars().collect();
    let tokens = tok::to_tokens(&input);

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

    let stuffs = to_stuffs(tokens);
    print_expr_(&stuffs, 6);
    eprintln!("{:?}", stuffs);

    println!("    }});");
    println!("  >");
    println!(">");
}
