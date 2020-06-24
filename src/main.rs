mod parser;
mod typing;

fn main() {
    let s = "(fun x { if x { 100 } else { 200 } } true)";
    let result = parser::parse_expr(s);
    println!("result = {:?}", result);
}
