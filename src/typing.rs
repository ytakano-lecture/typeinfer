use crate::parser::{Expr, If, Fun};
use std::collections::{LinkedList, HashMap};

pub type Constraint = LinkedList<(Type, Type)>;
pub type Context    = HashMap<String, Type>;

struct Subst(HashMap<u64, Type>);

#[derive(Debug)]
pub struct State {
    pub cnstr: Constraint,
    pub ctx: Context,
    pub tv: u64
}

#[derive(Debug, Clone)]
pub enum Type {
    Fun(Box::<Type>, Box::<Type>),
    Bool,
    Int,
    TVar(u64)
}

impl State {
    fn new() -> State {
        State{cnstr: Constraint::new(),
              ctx: Context::new(),
              tv: 0}
    }

    fn get_tv(&mut self) -> Type {
        let tv = self.tv;
        self.tv += 1;
        Type::TVar(tv)
    }
}

impl Subst {
    fn new() -> Subst {
        Subst(HashMap::new())
    }

    fn apply_type(&self, t: &Type) -> Type {
        match t {
            Type::Bool => Type::Bool,
            Type::Int  => Type::Int,
            Type::Fun(e1, e2) => {
                let t1 = self.apply_type(e1);
                let t2 = self.apply_type(e2);
                Type::Fun(Box::new(t1), Box::new(t2))
            }
            Type::TVar(tv) => {
                match self.0.get(tv) {
                    Some(ty) => ty.clone(),
                    None => t.clone()
                }
            }
        }
    }

    fn apply_constraint(&self, c: &Constraint) -> Constraint {
        let mut result = Constraint::new();
        for (t1, t2) in c {
            let t1 = self.apply_type(t1);
            let t2 = self.apply_type(t2);
            result.push_back((t1, t2));
        }
        result
    }
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
        Expr::If(e)       => typing_if(e, s),
        Expr::Id(e)       => typing_id(e, s),
        Expr::App(e1, e2) => typing_app(e1, e2, s),
        Expr::Fun(e)      => typing_fun(e, s),
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
                    let tv = s.get_tv();
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
    let tv = s.get_tv();

    s.cnstr.push_back((t1, Type::Fun(Box::new(t2), Box::new(tv.clone()))));

    tv
}

fn typing_fun(e: &Fun, s: &mut State) -> Type {
    let targ = s.get_tv();
    s.ctx.insert(e.arg.clone(), targ.clone());

    let texp = typing_expr(&e.expr, s);

    Type::Fun(Box::new(targ), Box::new(texp))
}

fn compose(s1: &Subst, s2: &Subst) -> Subst {
    let mut s = Subst::new();

    for (key, val) in &s2.0 {
        let t = s1.apply_type(val);
        s.0.insert(*key, t);
    }

    for (key, val) in &s1.0 {
        match s2.0.get(key) {
            Some(_) => (),
            None => {
                s.0.insert(*key, val.clone());
            }
        }
    }

    s
}