extern crate libc;
use std::env;

mod tokenize;

pub use crate::tokenize::tok;

#[derive(Debug)]
enum Stuff {
    Simple(tok::Token),
    Braced(Vec<Stuff>),
    LeftRightPair(BSLeftKind, Vec<Stuff>, BSRightKind),
}

#[derive(Copy, Clone)]
enum ParenKind {
    BareLeftParen,
    BareLeftBrace,
    BackslashLeft(BSLeftKind),
}

#[derive(Copy, Clone, Debug)]
enum BSLeftKind {
    LeftParen,
}

#[derive(Copy, Clone, Debug)]
enum BSRightKind {
    RightParen,
}

fn to_stuffs(input: Vec<tok::Token>) -> Vec<Stuff> {
    match to_stuffs_(&mut input.into_iter(), &Vec::new()) {
        (ans, None) => ans,
        _ => panic!("should not happen"),
    }
}

fn to_stuffs_(
    mut iter: &mut std::vec::IntoIter<tok::Token>,
    paren_stack: &[ParenKind],
) -> (Vec<Stuff>, Option<BSRightKind>) {
    let mut res = Vec::new();

    while let Some(x) = iter.next() {
        match x.kind {
            tok::TokenType::Alphanumeric
            | tok::TokenType::OrdinaryOperator
            | tok::TokenType::Underscore
            | tok::TokenType::Caret => {
                res.push(Stuff::Simple(x));
            }
            tok::TokenType::BackslashFollowedByAlphanumerics => {
                if x.str_repr == "\\left" {
                    let next_tok = iter
                        .next()
                        .expect("end of input encountered after `\\left`");
                    match next_tok.kind {
                        tok::TokenType::LeftParen => {
                            let mut new_stack = paren_stack.to_owned();
                            new_stack.push(ParenKind::BackslashLeft(BSLeftKind::LeftParen));
                            let (inner_stuffs, hopefully_something) =
                                to_stuffs_(&mut iter, &*new_stack);

                            res.push(Stuff::LeftRightPair(
                                BSLeftKind::LeftParen,
                                inner_stuffs,
                                hopefully_something.expect("should not happen"),
                            ));
                        }
                        _ => unimplemented!("unimplemented token found after `\\left`"),
                    }
                } else if x.str_repr == "\\right" {
                    let next_tok = iter
                        .next()
                        .expect("end of input encountered after `\\right`");
                    match next_tok.kind {
                        tok::TokenType::RightParen => match paren_stack.last() {
                            None => panic!("unmatched left brace"),
                            Some(ParenKind::BareLeftParen) => {
                                panic!("`\\right)` encountered before a left paren was matched")
                            }
                            Some(ParenKind::BareLeftBrace) => {
                                panic!("`\\right)` encontered before a left brace was matched");
                            }
                            Some(ParenKind::BackslashLeft(_)) => {
                                return (res, Some(BSRightKind::RightParen));
                            }
                        },
                        _ => unimplemented!("unimplemented token found after `\\right`"),
                    }
                } else {
                    res.push(Stuff::Simple(x));
                }
            }

            tok::TokenType::LeftParen => {
                let mut new_stack = paren_stack.to_owned();
                new_stack.push(ParenKind::BareLeftParen);
                let (inner_stuffs, hopefully_none) = to_stuffs_(&mut iter, &*new_stack);
                if let Some(_) = hopefully_none {
                    panic!("shouldn't happen");
                }
                res.push(Stuff::Simple(tok::Token {
                    kind: tok::TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\paren".to_string(),
                }));
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::LeftBrace => {
                let mut new_stack = paren_stack.to_owned();
                new_stack.push(ParenKind::BareLeftBrace);
                let (inner_stuffs, hopefully_none) = to_stuffs_(&mut iter, &*new_stack);
                if let Some(_) = hopefully_none {
                    panic!("shouldn't happen");
                }
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::RightBrace => match paren_stack.last() {
                None => panic!("unmatched left brace"),
                Some(ParenKind::BareLeftParen) => {
                    panic!("right brace encountered before a left paren was matched")
                }
                Some(ParenKind::BareLeftBrace) => {
                    return (res, None);
                }
                Some(ParenKind::BackslashLeft(BSLeftKind::LeftParen)) => {
                    panic!("right brace encountered before `\\left(` was matched")
                }
            },
            tok::TokenType::RightParen => match paren_stack.last() {
                None => panic!("unmatched left paren"),
                Some(ParenKind::BareLeftBrace) => {
                    panic!("right paren encountered before a left brace was matched")
                }
                Some(ParenKind::BareLeftParen) => {
                    return (res, None);
                }
                Some(ParenKind::BackslashLeft(BSLeftKind::LeftParen)) => {
                    panic!("right paren encountered before `\\left(` was matched")
                }
            },
        };
    }

    (res, None)
}

fn print_expr_(stuffs: &[Stuff], indent: usize) {
    for st in stuffs {
        match st {
            Stuff::Simple(t) => {
                println!("{:indent$}{}", "", t.str_repr, indent = indent);
            }
            Stuff::Braced(vec) => {
                println!("{:indent$}{{", "", indent = indent);
                print_expr_(vec, indent + 2);
                println!("{:indent$}}}", "", indent = indent);
            }
            Stuff::LeftRightPair(BSLeftKind::LeftParen, vec, BSRightKind::RightParen) => {
                println!("{:indent$}\\paren", "", indent = indent);
                println!("{:indent$}{{", "", indent = indent);
                print_expr_(vec, indent + 2);
                println!("{:indent$}}}", "", indent = indent);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let tokens = tok::to_tokens(&args[1]).unwrap();

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
