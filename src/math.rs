pub use crate::math_tokenize::tok;
use termion::{color, style};
#[derive(Debug)]
pub enum Stuff {
    Simple(tok::Token),
    Braced(Vec<Stuff>),
    LeftRightPair(BSLeftKind, Vec<Stuff>, BSRightKind),
    MatrixBody(MatrixBody),
}

type MatrixBody = Vec<Vec<Vec<Stuff>>>;

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
    LeftPipe,
    LeftEmpty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BSRightKind {
    RightParen,
    RightBracket,
    RightEmpty,
    RightPipe,
}

pub fn activated_math_addons(math: &Math) -> Vec<String> {
    let addon_defs: HashMap<String, String> = [
        (
            "\\hbar".to_string(),
            "let-math \\hbar = math-char MathOrd `ℏ` in ".to_string(),
        ),
        (
            "\\satysfify-internal-prime".to_string(),
            "let-math \\satysfify-internal-prime = math-char MathOrd `′` in ".to_string(),
        ),
        (
            "\\matrix-body".to_string(),
            // Taken from https://github.com/nekketsuuu/satysfi-matrix
            r#"
let matrix-body mss = let center ctx m = inline-fil ++ embed-math ctx m ++ inline-fil in
    let body = text-in-math MathInner (fun ctx -> (
      let size = get-font-size ctx in
      let pads-normal = (0pt, 0pt, size *' 0.25, size *' 0.25) in
      let pads-top = (0pt, 0pt, 0pt, size *' 0.25) in
      let pads-bottom = (0pt, 0pt, size *' 0.25, 0pt) in
      let pads-single = (0pt, 0pt, 0pt, 0pt) in
      let cell-of-m pads m = NormalCell(pads, inline-skip size ++ center ctx m) in
      let cells-of-ms pads ms =
        (match ms with
         | [] -> []
         | m :: ms ->
           (let leftmost = NormalCell(pads, center ctx m) in
            let rest = List.map (cell-of-m pads) ms in
            leftmost :: rest))
      in
      let-rec cellss-of-mss-tail mss =
        (match mss with
         | [] -> []
         | ms :: [] -> [cells-of-ms pads-bottom ms]
         | ms :: mss -> (cells-of-ms pads-normal ms) :: (cellss-of-mss-tail mss))
      in
      let cellss =
        (match mss with
         | [] -> []
         | ms :: [] -> [cells-of-ms pads-single ms]
         | ms :: mss -> (cells-of-ms pads-top ms) :: (cellss-of-mss-tail mss))
      in
      let grf = fun _ _ -> [] in
      let ib = tabular cellss grf in
      let (_, height, depth) = get-natural-metrics ib in
      let hgtaxis = get-axis-height ctx in
      let rising = hgtaxis -' (height +' depth) *' 0.5 in
      raise-inline rising ib)) 
    in body in
let-math \matrix mss = math-paren Math.paren-left Math.paren-right (matrix-body mss) in
let-math \matrix-body mss = matrix-body mss in
            "#
            .to_string(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let what_to_activate = get_what_to_activate_(&math);

    let mut activated_addons = Vec::new();

    for (key, code) in &addon_defs {
        if what_to_activate.contains(key) {
            activated_addons.push(code.clone());
        }
    }

    for left in &["paren-left", "sqbracket-left", "empty-paren", "bar-middle"] {
        for right in &[
            "sqbracket-right",
            "paren-right",
            "empty-paren",
            "bar-middle",
        ] {
            if what_to_activate
                .contains(&format!("\\satysfify-internal-{}-{}", left, right).clone())
            {
                activated_addons.push(
                    format!(
                        "let-math \\satysfify-internal-{left}-{right}  = math-paren Math.{left} Math.{right} in ",
                        left=left, right=right
                    )
                    .to_string(),
                );
            }
        }
    }

    activated_addons
}
impl tok::Token {
    fn to_bsrightkind(&self) -> BSRightKind {
        match self.kind {
            tok::TokenType::RightParen => BSRightKind::RightParen,
            tok::TokenType::RightBracket => BSRightKind::RightBracket,
            tok::TokenType::OrdinaryOperator => {
                if self.str_repr == "." {
                    BSRightKind::RightEmpty
                } else if self.str_repr == "\\|" {
                    BSRightKind::RightPipe
                } else {
                    unimplemented!("unimplemented token found after `\\right`")
                }
            }
            _ => unimplemented!("unimplemented token found after `\\right`"),
        }
    }

    // convert into BSLeft kind by specifying what it should become after `\left`
    fn to_bsleftkind(&self) -> Option<BSLeftKind> {
        match self.kind {
            tok::TokenType::LeftParen => Some(BSLeftKind::LeftParen),
            tok::TokenType::LeftBracket => Some(BSLeftKind::LeftBracket),
            tok::TokenType::OrdinaryOperator => {
                if self.str_repr == "\\|" {
                    Some(BSLeftKind::LeftPipe)
                } else if self.str_repr == "." {
                    Some(BSLeftKind::LeftEmpty)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn read_with_bare_paren_pair(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<tok::Token>>,
    kind: LeftParenKind,
    right: tok::TokenType,
) -> Result<Vec<Stuff>, String> {
    iter.next();
    let inner_stuffs = read_until_rightdelimiter_or_ampersand_or_bsbs(iter)?;
    let x = iter.next().ok_or(&format!(
        "end of input encountered before {} was matched",
        kind.msg()
    ))?;
    if x.kind != right {
        return Err(format!(
            "`{}` encountered before {} was matched",
            x.str_repr,
            kind.msg()
        ));
    }
    Ok(inner_stuffs)
}

fn read_matrixbody(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<tok::Token>>,
    env: MatrixEnvironment,
) -> Result<MatrixBody, String> {
    iter.next(); // parses off \begin{matrix}
    type Stuffs = Vec<Stuff>;
    let mut matrix_body: Vec<Vec<Stuffs>> = Vec::new();
    let mut row: Vec<Stuffs> = Vec::new();
    loop {
        let inner_stuffs = read_until_rightdelimiter_or_ampersand_or_bsbs(iter)?;
        match iter.peek() {
            None => {
                return Err(format!(
                    "end of input encountered before `\\begin{{{}}}` was matched",
                    env.to_string()
                ))
            }
            Some(x) => match x.kind {
                tok::TokenType::Ampersand => {
                    row.push(inner_stuffs);
                    iter.next();
                }
                tok::TokenType::DoubleBackslash => {
                    row.push(inner_stuffs);
                    matrix_body.push(row);
                    row = Vec::new();
                    iter.next();
                }
                tok::TokenType::BackslashEnd => {
                    if x.str_repr != format!("\\end{{{}}}", env.to_string()) {
                        return Err(format!(
                            "{} encountered before `\\begin{{{}}}` was matched",
                            env.to_string(),
                            x.str_repr
                        ));
                    }

                    iter.next();

                    // We need to ignore exactly one \\ .
                    if row.is_empty() {
                        // \\ has done its job
                        break;
                    } else {
                        row.push(inner_stuffs);
                        matrix_body.push(row);
                        break;
                    }
                }
                _ => return Err(format!("unmatched `{}` inside a matrix", x.str_repr)),
            },
        }
    }
    Ok(matrix_body)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MatrixEnvironment {
    Matrix,
}

impl MatrixEnvironment {
    fn to_string(self) -> String {
        match self {
            MatrixEnvironment::Matrix => "matrix".to_string(),
        }
    }
}

fn read_until_rightdelimiter_or_ampersand_or_bsbs(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<tok::Token>>,
) -> Result<Vec<Stuff>, String> {
    let mut res = Vec::new();

    while let Some(x) = iter.peek() {
        match x.kind {
            tok::TokenType::RightBrace
            | tok::TokenType::RightParen
            | tok::TokenType::RightBracket
            | tok::TokenType::Ampersand
            | tok::TokenType::DoubleBackslash
            | tok::TokenType::BackslashEnd => {
                return Ok(res);
            }
            tok::TokenType::Alphanumeric
            | tok::TokenType::OrdinaryOperator
            | tok::TokenType::Underscore
            | tok::TokenType::Caret => {
                let x_ = iter.next().unwrap(); // iter.peek() gave Some(); hence never fails
                res.push(Stuff::Simple(x_));
            }

            tok::TokenType::BackslashBegin => {
                if x.str_repr == "\\begin{matrix}" {
                    let inner_stuffs = read_matrixbody(iter, MatrixEnvironment::Matrix)?;
                    res.push(Stuff::MatrixBody(inner_stuffs));
                } else {
                    return Err(format!("`{}` is not implemented", x.str_repr).to_string());
                }
            }

            tok::TokenType::LeftParen => {
                let inner_stuffs = read_with_bare_paren_pair(
                    iter,
                    LeftParenKind::BareLeftParen,
                    tok::TokenType::RightParen,
                )?;
                res.push(Stuff::Simple(tok::Token {
                    kind: tok::TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\paren".to_string(),
                }));
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::LeftBrace => {
                let inner_stuffs = read_with_bare_paren_pair(
                    iter,
                    LeftParenKind::BareLeftBrace,
                    tok::TokenType::RightBrace,
                )?;
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::LeftBracket => {
                let inner_stuffs = read_with_bare_paren_pair(
                    iter,
                    LeftParenKind::BareLeftBracket,
                    tok::TokenType::RightBracket,
                )?;
                res.push(Stuff::Simple(tok::Token {
                    kind: tok::TokenType::BackslashFollowedByAlphanumerics,
                    str_repr: "\\sqbracket".to_string(),
                }));
                res.push(Stuff::Braced(inner_stuffs));
            }
            tok::TokenType::BackslashFollowedByAlphanumerics => {
                if x.str_repr == "\\left" {
                    iter.next();
                    let next_tok = iter
                        .next()
                        .ok_or("end of input encountered after `\\left`")?;
                    let bsleftkind = match next_tok.to_bsleftkind() {
                        Some(x) => x,
                        None => unimplemented!("unimplemented token found after `\\left`"),
                    };
                    let kind = LeftParenKind::BackslashLeft(bsleftkind);
                    let inner_stuffs = read_until_rightdelimiter_or_ampersand_or_bsbs(iter)?;
                    let x = iter.next().ok_or(&format!(
                        "end of input encountered before {} was matched",
                        kind.msg()
                    ))?;
                    if x.kind == tok::TokenType::BackslashFollowedByAlphanumerics {
                        if x.str_repr != "\\right" {
                            unreachable!();
                        }

                        let next_tok = iter
                            .next()
                            .ok_or("end of input encountered after `\\right`")?;
                        let bsrightkind = next_tok.to_bsrightkind();
                        res.push(Stuff::LeftRightPair(bsleftkind, inner_stuffs, bsrightkind));
                    } else {
                        return Err(format!(
                            "{} encountered before {} was matched",
                            x.str_repr,
                            kind.msg()
                        ));
                    }
                } else if x.str_repr == "\\right" {
                    // no consumption; return
                    return Ok(res);
                } else {
                    let x_ = iter.next().unwrap(); // iter.peek() gave Some(); hence never fails
                    res.push(Stuff::Simple(x_));
                }
            }
        };
    }

    iter.next();
    Ok(res)
}

impl LeftParenKind {
    fn msg(self) -> &'static str {
        match self {
            LeftParenKind::BareLeftBrace => "a left brace `{`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftParen) => "`\\left(`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftBracket) => "`\\left[`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftEmpty) => "`\\left.`",
            LeftParenKind::BackslashLeft(BSLeftKind::LeftPipe) => "`\\left|`",
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
            BSLeftKind::LeftEmpty => BSRightKind::RightEmpty,
            BSLeftKind::LeftPipe => BSRightKind::RightPipe,
        }
    }
    fn satysfi_name(self) -> &'static str {
        match self {
            BSLeftKind::LeftBracket => "sqbracket-left",
            BSLeftKind::LeftParen => "paren-left",
            BSLeftKind::LeftEmpty => "empty-paren",
            BSLeftKind::LeftPipe => "bar-middle",
        }
    }
    fn satysfi_name_when_pair_matches(self) -> &'static str {
        match self {
            BSLeftKind::LeftBracket => "sqbracket",
            BSLeftKind::LeftParen => "paren",
            BSLeftKind::LeftEmpty => "satysfify-internal-empty-paren-empty-paren",
            BSLeftKind::LeftPipe => "abs",
        }
    }
}

impl BSRightKind {
    fn satysfi_name(self) -> &'static str {
        match self {
            BSRightKind::RightBracket => "sqbracket-right",
            BSRightKind::RightParen => "paren-right",
            BSRightKind::RightEmpty => "empty-paren",
            BSRightKind::RightPipe => "bar-middle",
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
            Stuff::MatrixBody(matrix_body) => {
                println!("{:indent$}\\matrix-body![", "", indent = indent);
                for row in matrix_body {
                    println!("{:indent$}[", "", indent = indent + 2);
                    for cell in row {
                        println!("{:indent$}${{", "", indent = indent + 4);
                        print_math(cell, indent + 6);
                        println!("{:indent$}}};", "", indent = indent + 4);
                    }
                    println!("{:indent$}];", "", indent = indent + 2);
                }
                println!("{:indent$}]", "", indent = indent);
            }
        }
    }
}

fn get_command_name_from_leftright(left: BSLeftKind, right: BSRightKind) -> String {
    if right == left.matching_right() {
        format!("\\{}", left.satysfi_name_when_pair_matches())
    } else {
        format!(
            "\\satysfify-internal-{}-{}",
            left.satysfi_name(),
            right.satysfi_name()
        )
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
                if *right != left.matching_right() || *left == BSLeftKind::LeftEmpty {
                    defs.insert(get_command_name_from_leftright(*left, *right));
                }
                let internal = get_what_to_activate(vec);
                for k in &internal {
                    defs.insert(k.to_string());
                }
            }
            Stuff::MatrixBody(matrix_body) => {
                for row in matrix_body {
                    for cell in row {
                        let internal = get_what_to_activate(cell);
                        for k in &internal {
                            defs.insert(k.to_string());
                        }
                    }
                }
                defs.insert("\\matrix-body".to_string());
            }
        }
    }
    defs
}

pub fn to_math(input: Vec<tok::Token>) -> Result<Math, String> {
    let mut iter = input.into_iter().peekable();
    let ans = read_until_rightdelimiter_or_ampersand_or_bsbs(&mut iter)?;
    match iter.next() {
        None => Ok(Math { stuffs: ans }), /* have consumed all the inputs */
        Some(x) => Err(format!("unmatched `{}`", x.str_repr)),
    }
}
