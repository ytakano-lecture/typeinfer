use std::{env, fs};

mod parser;
mod typing;

fn main() {
    // コマンドライン引数の検査
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("以下のようにファイル名を指定して実行してください\ncargo run examples/ex1.lambda");
        return;
    }

    // ファイル読み込み
    let content = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            println!("エラー: {:?}", e);
            return;
        }
    };

    // パース
    match parser::parse_expr(&content) {
        Ok((_, ast)) => {
            println!("抽象構文木: {:?}", ast);
            let s = typing::infer(&ast); // 型推論
            println!("型環境: {:?}", s.0);
            println!("型制約: {:?}", s.1);
            match s.2 {
                Some(sbst) => println!("代入:   {:?}", sbst),
                None => println!("型推論に失敗しました"),
            }
        }
        Err(msg) => {
            println!("error: {:?}", msg);
        }
    }
}
