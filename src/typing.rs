use crate::parser::{Expr, If};
use std::collections::{LinkedList, HashMap};

pub type Constraint = LinkedList<(Type, Type)>;
pub type Context    = HashMap<String, Type>;

#[derive(Debug)]
pub struct State {
    pub cnstr: Constraint,
    pub ctx: Context,
    pub tv: u64
}

impl State {
    fn new() -> State {
        State{cnstr: Constraint::new(),
              ctx: Context::new(),
              tv: 0}
    }

    fn get_tv(&mut self) -> u64 {
        let tv = self.tv;
        self.tv += 1;
        tv
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Fun(Box::<Type>, Box::<Type>),
    Bool,
    Int,
    TVar(u64)
}

pub fn infer(e: &Expr) -> State {
    let mut s = State::new();
    typing_expr(e, &mut s);
    s
}

fn typing_expr(e: &Expr, s: &mut State) -> Type {
    match e {
        Expr::Int(_)      => Type::Int,
        Expr::Bool(_)     => Type::Bool,
        Expr::If(e)       => typing_if(&e, s),
        Expr::Id(e)       => typing_id(&e, s),
        Expr::App(e1, e2) => typing_app(e1, e2, s),
        _ => Type::Int
    }
}

fn typing_if(e: &If, s: &mut State) -> Type {
    let ty_cond = typing_expr(&e.cond, s);
    let ty_then = typing_expr(&e.then, s);
    let ty_els  = typing_expr(&e.els, s);

    s.cnstr.push_back((ty_cond, Type::Bool));
    s.cnstr.push_back((ty_then, ty_els.clone()));

    ty_els
}

fn typing_id(e: &String, s: &mut State) -> Type {
    match e.as_str() {
        "true"   => Type::Bool,
        "false"  => Type::Bool,
        "succ"   => Type::Fun(Box::new(Type::Int), Box::new(Type::Int)),
        "pred"   => Type::Fun(Box::new(Type::Int), Box::new(Type::Int)),
        "iszero" => Type::Fun(Box::new(Type::Int), Box::new(Type::Bool)),
        id => {
            match s.ctx.get(id) {
                Some(t) => t.clone(),
                None => {
                    let tv = Type::TVar(s.get_tv());
                    s.ctx.insert(id.to_string(), tv.clone());
                    tv
                }
            }
        }
    }
}

fn typing_app(e1: &Expr, e2: &Expr, s: &mut State) -> Type {
    let t2 = typing_expr(e2, s);
    let t1 = typing_expr(e1, s);
    let tv = Type::TVar(s.get_tv());

    s.cnstr.push_back((t1, Type::Fun(Box::new(t2), Box::new(tv.clone()))));

    tv
}