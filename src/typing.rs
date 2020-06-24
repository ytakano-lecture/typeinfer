use crate::parser::Expr;

enum Type {
    Fun(Box::<Type>, Box::<Type>),
    Bool,
    Int,
    TVar(u64)
}

fn typing_expr(e: &Expr) -> Type {
    match e {
        Expr::Int(_)  => Type::Int,
        Expr::Bool(_) => Type::Bool,
        _ => Type::Int
    }
}

fn typing_if(e: &Expr) -> Type {
    Type::Int
}