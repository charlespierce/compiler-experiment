mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let lexer = Lexer::new(r#"(add 2 (subtract "314" 2))"#.to_string());
    let mut parser = Parser::new(lexer);

    println!("{:?}", parser.parse());
}
