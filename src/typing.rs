use crate::parser::{Expr, Fun, If};
use std::collections::{HashMap, LinkedList};

// 型制約
pub type Constraint = LinkedList<(Type, Type)>;

// 型環境
pub type Context = HashMap<String, Type>;

// 置換
pub type Subst = HashMap<u64, Type>;

#[derive(Debug)]
pub struct State {
    pub cnstr: Constraint,
    pub ctx: Context,
    pub tv: u64,
}

// 型の表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Fun(Box<Type>, Box<Type>), // 関数の型
    Bool,                      // 真偽値型
    Int,                       // 整数型
    TVar(u64),                 // 型変数
}

impl State {
    fn new() -> State {
        State {
            cnstr: Constraint::new(),
            ctx: Context::new(),
            tv: 0,
        }
    }

    // 新しい型変数を取得する関数
    fn get_tv(&mut self) -> Type {
        let tv = self.tv;
        self.tv += 1;
        Type::TVar(tv)
    }
}

// 置換を型に適用する関数
fn apply_type(s: &Subst, t: &Type) -> Type {
    match t {
        Type::Bool => Type::Bool,
        Type::Int => Type::Int,
        Type::Fun(e1, e2) => {
            let t1 = apply_type(s, e1);
            let t2 = apply_type(s, e2);
            Type::Fun(Box::new(t1), Box::new(t2))
        }
        Type::TVar(tv) => match s.get(tv) {
            Some(ty) => ty.clone(),
            None => t.clone(),
        },
    }
}

// 置換を型制約に適用する関数
fn apply_constraint(s: &Subst, c: &Constraint) -> Constraint {
    let mut result = Constraint::new();
    for (t1, t2) in c {
        let t1 = apply_type(s, t1);
        let t2 = apply_type(s, t2);
        result.push_back((t1, t2));
    }
    result
}

impl Type {
    // 型中にtvと同じ型変数がある場合true
    fn has_tv(&self, tv: u64) -> bool {
        match self {
            Type::Fun(t1, t2) => t1.has_tv(tv) || t2.has_tv(tv),
            Type::Bool => false,
            Type::Int => false,
            Type::TVar(x) => tv == *x,
        }
    }
}

// 型推論を行う関数
pub fn infer(e: &Expr) -> (Context, Constraint, Option<Subst>) {
    let mut s = State::new();

    // まず、typing_exprで型制約を求める
    typing_expr(e, &mut s);
    let c = s.cnstr.clone();

    // 次に、得られた型制約から、unify関数で単一化を行う
    // unify関数の結果とコンテキストが型推論の結果となる
    (s.ctx, c, unify(s.cnstr))
}

// 型付けを行いつつ、型制約を求める関数
fn typing_expr(e: &Expr, s: &mut State) -> Type {
    match e {
        Expr::Int(_) => Type::Int,                  // 整数値リテラル
        Expr::Bool(_) => Type::Bool,                // 真偽値リテラル
        Expr::If(e) => typing_if(e, s),             // if式
        Expr::Id(e) => typing_id(e, s),             // 変数
        Expr::App(e1, e2) => typing_app(e1, e2, s), // 関数適用
        Expr::Fun(e) => typing_fun(e, s),           // 関数定義
    }
}

// if式の型付けを行う関数
fn typing_if(e: &If, s: &mut State) -> Type {
    let ty_cond = typing_expr(&e.cond, s);
    let ty_then = typing_expr(&e.then, s);
    let ty_els = typing_expr(&e.els, s);

    s.cnstr.push_back((ty_cond, Type::Bool));
    s.cnstr.push_back((ty_then, ty_els.clone()));

    ty_els
}

// 変数の型付けを行う関数
fn typing_id(e: &String, s: &mut State) -> Type {
    match e.as_str() {
        "true" => Type::Bool,
        "false" => Type::Bool,
        "succ" => Type::Fun(Box::new(Type::Int), Box::new(Type::Int)),
        "pred" => Type::Fun(Box::new(Type::Int), Box::new(Type::Int)),
        "iszero" => Type::Fun(Box::new(Type::Int), Box::new(Type::Bool)),
        id => match s.ctx.get(id) {
            Some(t) => t.clone(),
            None => {
                let tv = s.get_tv();
                s.ctx.insert(id.to_string(), tv.clone());
                tv
            }
        },
    }
}

// 関数定義の型付けを行う関数
fn typing_fun(e: &Fun, s: &mut State) -> Type {
    let targ = s.get_tv();
    s.ctx.insert(e.arg.clone(), targ.clone());

    let texp = typing_expr(&e.expr, s);

    Type::Fun(Box::new(targ), Box::new(texp))
}

// 関数適用の型付けを行う関数
fn typing_app(e1: &Expr, e2: &Expr, s: &mut State) -> Type {
    // ここを実装せよ

    Type::Bool // コンパイルを通すためのダミーなので実装する際はこの行は削除すること
}

// 置換の合成を行う関数
fn compose(s1: &Subst, s2: &Subst) -> Subst {
    let mut s = Subst::new();

    // ここを実装せよ

    s
}

// 単一化を行う関数
fn unify(mut c: Constraint) -> Option<Subst> {
    if c.is_empty() {
        return Some(Subst::new());
    }

    // 型制約中の先頭の制約を取り出す
    let pair = c.pop_front().unwrap();
    if pair.0 == pair.1 {
        return unify(c);
    }

    match pair {
        (Type::TVar(tv), _) => {
            if !pair.1.has_tv(tv) {
                let mut s2 = Subst::new();
                s2.insert(tv, pair.1);
                let c = apply_constraint(&s2, &c);
                let s1 = unify(c)?;
                return Some(compose(&s1, &s2));
            }
        }

        // ここを実装せよ
        _ => {
            return None;
        }
    }

    return None;
}
