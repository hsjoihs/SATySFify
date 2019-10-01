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
enum LeftParenKind {
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

fn to_stuffs(input: Vec<tok::Token>) -> Result<Vec<Stuff>, String> {
    match to_stuffs_(&mut input.into_iter(), &Vec::new())? {
        (ans, None) => Ok(ans),
        _ => panic!("should not happen"),
    }
}

fn to_stuffs_(
    mut iter: &mut std::vec::IntoIter<tok::Token>,
    paren_stack: &[LeftParenKind],
) -> Result<(Vec<Stuff>, Option<BSRightKind>), String> {
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
                        .ok_or("end of input encountered after `\\left`")?;
                    match next_tok.kind {
                        tok::TokenType::LeftParen => {
                            let mut new_stack = paren_stack.to_owned();
                            new_stack.push(LeftParenKind::BackslashLeft(BSLeftKind::LeftParen));
                            let (inner_stuffs, hopefully_something) =
                                to_stuffs_(&mut iter, &*new_stack)?;

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
                        .ok_or("end of input encountered after `\\right`")?;
                    match next_tok.kind {
                        tok::TokenType::RightParen => match paren_stack.last() {
                            None => {
                                return Err("unmatched left brace".to_string());
                            }
                            Some(LeftParenKind::BackslashLeft(_)) => {
                                return Ok((res, Some(BSRightKind::RightParen)));
                            }
                            Some(&x) => {
                                return Err(format!(
                                    "`\\right)` encountered before {} was matched",
                                    x.msg()
                                ));
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
                new_stack.push(LeftParenKind::BareLeftParen);
                let (inner_stuffs, hopefully_none) = to_stuffs_(&mut iter, &*new_stack)?;
                if hopefully_none.is_some() {
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
                new_stack.push(LeftParenKind::BareLeftBrace);
                let (inner_stuffs, hopefully_none) = to_stuffs_(&mut iter, &*new_stack)?;
                if hopefully_none.is_some() {
                    panic!("shouldn't happen");
                }
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::RightBrace => match paren_stack.last() {
                None => return Err("unmatched right brace".to_string()),
                Some(LeftParenKind::BareLeftBrace) => {
                    return Ok((res, None));
                }
                Some(&x) => {
                    return Err(format!(
                        "right brace encountered before {} was matched",
                        x.msg()
                    ))
                }
            },
            tok::TokenType::RightParen => match paren_stack.last() {
                None => return Err("unmatched right paren".to_string()),
                Some(LeftParenKind::BareLeftParen) => {
                    return Ok((res, None));
                }
                Some(&x) => {
                    return Err(format!(
                        "right paren encountered before {} was matched",
                        x.msg()
                    ))
                }
            },
        };
    }

    Ok((res, None))
}

impl LeftParenKind {
    fn msg(&self) -> &'static str {
        match self {
            LeftParenKind::BareLeftBrace => "a left brace",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftParen) => "`\\left(`",
            LeftParenKind::BareLeftParen => "a left paren",
        }
    }
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let tokens = tok::to_tokens(&args[1])?;

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

    let stuffs = to_stuffs(tokens)?;
    print_expr_(&stuffs, 6);
    eprintln!("{:?}", stuffs);

    println!("    }});");
    println!("  >");
    println!(">");

    Ok(())
}
