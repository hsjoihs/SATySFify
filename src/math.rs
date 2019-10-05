pub use crate::math_tokenize::tok;
use termion::{color, style};
#[derive(Debug)]
pub enum Stuff {
    Simple(tok::Token),
    Braced(Vec<Stuff>),
    LeftRightPair(BSLeftKind, Vec<Stuff>, BSRightKind),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LeftParenKind {
    BareLeftParen,
    BareLeftBrace,
    BareLeftBracket,
    BackslashLeft(BSLeftKind),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BSLeftKind {
    LeftParen,
    LeftBracket,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BSRightKind {
    RightParen,
    RightBracket,
}

fn to_stuffs(input: Vec<tok::Token>) -> Result<Vec<Stuff>, String> {
    match to_stuffs_(&mut input.into_iter(), &Vec::new())? {
        (ans, None) => Ok(ans),
        _ => unreachable!(),
    }
}

pub fn activated_math_addons(math: &Math) -> Vec<&str> {
    let addon_defs: HashMap<String, &str> = [
        (
            "\\hbar".to_string(),
            "let-math \\hbar = math-char MathOrd `ℏ` in ",
        ),
        (
            "\\satysfifi-internal-prime".to_string(),
            "let-math \\satysfifi-internal-prime = math-char MathOrd `′` in ",
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let paren_addon_defs: HashMap<String, &str> = [(
        "\\satysfifi-internal-paren-left-sqbracket-right".to_string(),
     "let-math \\satysfifi-internal-paren-left-sqbracket-right  = math-paren Math.paren-left Math.sqbracket-right in "
    ), (
        "\\satysfifi-internal-sqbracket-left-paren-right".to_string(),
     "let-math \\satysfifi-internal-sqbracket-left-paren-right  = math-paren Math.sqbracket-left Math.paren-right in "
    )]
    .iter()
    .cloned()
    .collect();

    let what_to_activate = get_what_to_activate_(&math);

    let mut activated_addons = Vec::new();

    for (key, code) in &addon_defs {
        if what_to_activate.get(key).is_some() {
            activated_addons.push(*code);
        }
    }
    for (key, code) in &paren_addon_defs {
        if what_to_activate.get(key).is_some() {
            activated_addons.push(*code);
        }
    }
    activated_addons
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
                        tok::TokenType::LeftBracket => {
                            let mut new_stack = paren_stack.to_owned();
                            new_stack.push(LeftParenKind::BackslashLeft(BSLeftKind::LeftBracket));
                            let (inner_stuffs, hopefully_something) =
                                to_stuffs_(&mut iter, &*new_stack)?;

                            res.push(Stuff::LeftRightPair(
                                BSLeftKind::LeftBracket,
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
                                return Err("unmatched `\\right)`".to_string());
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
                        tok::TokenType::RightBracket => match paren_stack.last() {
                            None => {
                                return Err("unmatched `\\right]`".to_string());
                            }
                            Some(LeftParenKind::BackslashLeft(_)) => {
                                return Ok((res, Some(BSRightKind::RightBracket)));
                            }
                            Some(&x) => {
                                return Err(format!(
                                    "`\\right]` encountered before {} was matched",
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
                    unreachable!();
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
                    unreachable!();
                }
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::LeftBracket => {
                let mut new_stack = paren_stack.to_owned();
                new_stack.push(LeftParenKind::BareLeftBracket);
                let (inner_stuffs, hopefully_none) = to_stuffs_(&mut iter, &*new_stack)?;
                if hopefully_none.is_some() {
                    unreachable!();
                }
                res.push(Stuff::Simple(tok::Token {
                    kind: tok::TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\sqbracket".to_string(),
                }));
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
            tok::TokenType::RightBracket => match paren_stack.last() {
                None => return Err("unmatched right bracket".to_string()),
                Some(LeftParenKind::BareLeftBracket) => {
                    return Ok((res, None));
                }
                Some(&x) => {
                    return Err(format!(
                        "right bracket encountered before {} was matched",
                        x.msg()
                    ))
                }
            },
        };
    }

    match paren_stack.last() {
        None => Ok((res, None)),
        Some(&x) => Err(format!(
            "end of input encountered before {} was matched",
            x.msg()
        )),
    }
}

impl LeftParenKind {
    fn msg(self) -> &'static str {
        match self {
            LeftParenKind::BareLeftBrace => "a left brace `{`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftParen) => "`\\left(`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftBracket) => "`\\left[`",
            LeftParenKind::BareLeftParen => "a left paren `(`",
            LeftParenKind::BareLeftBracket => "a left bracket `[`",
        }
    }
}

impl BSLeftKind {
    fn matching_right(self) -> BSRightKind {
        match self {
            BSLeftKind::LeftParen => BSRightKind::RightParen,
            BSLeftKind::LeftBracket => BSRightKind::RightBracket,
        }
    }
}

pub fn print_math(stuffs: &[Stuff], indent: usize) {
    for st in stuffs {
        match st {
            Stuff::Simple(t) => {
                if t.str_repr == "\\le" {
                    println!("{:indent$}\\leq", "", indent = indent);
                } else if t.str_repr == "\\dots" {
                    eprintln!("{}{}`\\dots` detected; converting it into `\\ldots` (you might want to fix this) {}{}", 
                    color::Fg(color::Red),
                    style::Bold,
                    style::Reset,
                    color::Fg(color::Reset));
                    println!("{:indent$}\\ldots", "", indent = indent);
                } else {
                    println!("{:indent$}{}", "", t.str_repr, indent = indent);
                }
            }
            Stuff::Braced(vec) => {
                println!("{:indent$}{{", "", indent = indent);
                print_math(vec, indent + 2);
                println!("{:indent$}}}", "", indent = indent);
            }
            Stuff::LeftRightPair(left, vec, right) => {
                println!(
                    "{:indent$}{}",
                    "",
                    get_command_name_from_leftright(*left, *right),
                    indent = indent
                );
                println!("{:indent$}{{", "", indent = indent);
                print_math(vec, indent + 2);
                println!("{:indent$}}}", "", indent = indent);
            }
        }
    }
}

fn get_command_name_from_leftright(left: BSLeftKind, right: BSRightKind) -> String {
    if right == left.matching_right() {
        format!("\\{}", left.matching_name())
    } else {
        format!(
            "\\satysfifi-internal-{}-{}",
            left.satysfi_name(),
            right.satysfi_name()
        )
    }
}

impl BSLeftKind {
    fn satysfi_name(self) -> &'static str {
        match self {
            BSLeftKind::LeftBracket => "sqbracket-left",
            BSLeftKind::LeftParen => "paren-left",
        }
    }
    fn matching_name(self) -> &'static str {
        match self {
            BSLeftKind::LeftBracket => "sqbracket",
            BSLeftKind::LeftParen => "paren",
        }
    }
}

impl BSRightKind {
    fn satysfi_name(self) -> &'static str {
        match self {
            BSRightKind::RightBracket => "sqbracket-right",
            BSRightKind::RightParen => "paren-right",
        }
    }
}

use std::collections::HashMap;

pub struct Math {
    pub stuffs: Vec<Stuff>,
}
use std::collections::HashSet;
fn get_what_to_activate_(math: &Math) -> HashSet<String> {
    get_what_to_activate(&math.stuffs)
}
fn get_what_to_activate(stuffs: &[Stuff]) -> HashSet<String> {
    let mut defs = HashSet::new();
    for st in stuffs {
        match st {
            Stuff::Simple(t) => {
                defs.insert(t.str_repr.clone());
            }
            Stuff::Braced(vec) => {
                let internal = get_what_to_activate(vec);
                for k in &internal {
                    defs.insert(k.to_string());
                }
            }
            Stuff::LeftRightPair(left, vec, right) => {
                if *right != left.matching_right() {
                    defs.insert(get_command_name_from_leftright(*left, *right));
                }
                let internal = get_what_to_activate(vec);
                for k in &internal {
                    defs.insert(k.to_string());
                }
            }
        }
    }
    eprintln!("{:?}", defs);
    defs
}

pub fn to_math(input: Vec<tok::Token>) -> Result<Math, String> {
    let stuffs = to_stuffs(input)?;
    Ok(Math { stuffs })
}
