mod token;

use token::Tokenizer;

fn main() {
    let tokens = Tokenizer::new(r#"(add 2 (subtract "314" 2))"#.to_string());

    for token in tokens {
        println!("{:?}", token);
    }
}
