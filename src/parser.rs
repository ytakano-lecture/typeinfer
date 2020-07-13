extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace0, multispace1};
use nom::sequence::delimited;
use nom::IResult;

// 抽象構文木
#[derive(Debug)]
pub enum Expr {
    Fun(Box<Fun>),             // 関数定義
    If(Box<If>),               // if式
    App(Box<Expr>, Box<Expr>), // 関数適用
    Id(String),                // 変数
    Bool(bool),                // 真偽値リテラル
    Int(u64),                  // 整数値リテラル
}

// 関数定義
#[derive(Debug)]
pub struct Fun {
    pub arg: String, // 変数名
    pub expr: Expr,  // 関数の中身
}

// if式
#[derive(Debug)]
pub struct If {
    pub cond: Expr, // 条件分岐
    pub then: Expr, // 条件が真の場合に行う式
    pub els: Expr,  // 条件が偽の場合に行う式
}

// ここ以下は式をパースしているだけなので、コードを読む必要はない

pub fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, _) = multispace0(i)?;
    alt((
        parse_fun, parse_if, parse_bool, parse_int, parse_app, parse_id,
    ))(i)
}

fn parse_id(i: &str) -> IResult<&str, Expr> {
    let (i, id) = alpha1(i)?;
    Ok((i, Expr::Id(id.to_string())))
}

fn parse_bool(i: &str) -> IResult<&str, Expr> {
    let (i, val) = alt((tag("true"), tag("false")))(i)?;
    if val == "true" {
        Ok((i, Expr::Bool(true)))
    } else {
        Ok((i, Expr::Bool(false)))
    }
}

fn parse_int(i: &str) -> IResult<&str, Expr> {
    let (i, val) = digit1(i)?;
    Ok((i, Expr::Int(val.parse().unwrap())))
}

fn parse_app(i: &str) -> IResult<&str, Expr> {
    let (i, _) = char('(')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, e1) = parse_expr(i)?;
    let (i, _) = multispace1(i)?;
    let (i, e2) = parse_expr(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;

    Ok((i, (Expr::App(Box::new(e1), Box::new(e2)))))
}

fn parse_if(i: &str) -> IResult<&str, Expr> {
    let (i, _) = tag("if")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, cond) = parse_expr(i)?;
    let (i, _) = multispace0(i)?;

    let (i, then) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag("else")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, els) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    let e = Expr::If(Box::new(If {
        cond: cond,
        then: then,
        els: els,
    }));

    Ok((i, e))
}

fn parse_fun(i: &str) -> IResult<&str, Expr> {
    let (i, _) = tag("fun")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, arg) = alpha1(i)?;
    let (i, _) = multispace0(i)?;

    let (i, body) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    let e = Expr::Fun(Box::new(Fun {
        arg: arg.to_string(),
        expr: body,
    }));

    Ok((i, e))
}
