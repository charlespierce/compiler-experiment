mod compiler;
mod emitter;
mod lexer;
mod parser;

use compiler::compile;

fn main() {
    let input = r#"(add 2 (subtract "314" 2))"#;
    let output = compile(input.to_string());
    println!("Input : {}", input);
    println!("Output: {}", output);
}
