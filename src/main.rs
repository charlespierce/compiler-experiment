mod emitter;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let input = r#"(add 2 (subtract "314" 2))"#;
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    println!("Input : {}", input);
    println!("Output: {}", result.emit());
}
