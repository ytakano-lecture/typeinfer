mod parser;
mod typing;

fn main() {
    let s = "(fun x { if x { 100 } else { 200 } } true)";
//    let s = "if (iszero x) { 10 } else { 20 }";
//    let s = "(a 10)";

    match parser::parse_expr(s) {
        Ok((_, ast)) => {
            println!("AST = {:?}", ast);
            let s = typing::infer(&ast);
            println!("s = {:?}", s);
        }
        Err(msg) => {
            println!("error: {:?}", msg);
        }
    }
//    let ty = typing::typing_expr();
}
