use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn compile(input: String) -> String {
    Parser::new(Lexer::new(input)).parse().emit()
}
