use crate::parser::parser::parse_from_string;
use crate::parser::nom::expression::{expr, disjunction_expr};

mod ast;
mod parser;


fn main() {
    println!("Hello, world!");
    // let file_node = parse_from_string("1".to_string());
    // println!("{:?}", file_node);
    // 1 + (1 * (12 + 2))
    let expr = disjunction_expr("1||2 || 3"); // expr("1 + (1 * 12) + 2");
    match expr {
        Ok((s, e)) => {
            println!("unused => {}", s);
            println!("AST => {:?}", e);
        }
        Err(e) => {
        }
    }
}
