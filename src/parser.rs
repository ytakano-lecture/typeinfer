extern crate nom;

use nom::IResult;
use nom::character::complete::{char, alpha1, digit1, multispace0, multispace1};
use nom::bytes::complete::{tag};
use nom::sequence::delimited;
use nom::branch::alt;

#[derive(Debug)]
pub enum Expr {
    Fun(Box::<Fun>),
    If(Box::<If>),
    App(Box::<Expr>, Box::<Expr>),
    Id(String),
    Bool(bool),
    Int(u64),
}

#[derive(Debug)]
pub struct Fun {
    pub arg : String,
    pub expr : Expr
}

#[derive(Debug)]
pub struct If {
    pub cond : Expr,
    pub then : Expr,
    pub els  : Expr
}

pub fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, _) = multispace0(i)?;
    alt((
        parse_fun,
        parse_if,
        parse_bool,
        parse_int,
        parse_app,
        parse_id
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

    let e = Expr::If(Box::new(
        If {
            cond: cond,
            then: then,
            els: els
        }
    ));

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

    let e = Expr::Fun(Box::new(
        Fun {
            arg: arg.to_string(),
            expr: body
        }
    ));

    Ok((i, e))
}