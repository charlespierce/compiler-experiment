use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Expression {
    NumberLiteral(String),
    StringLiteral(String),
    FunctionCall {
        name: String,
        params: Vec<Expression>,
    },
}

#[derive(Debug)]
pub struct Program {
    body: Vec<Expression>,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Program {
        let mut body: Vec<Expression> = Vec::new();

        while let Some(token) = self.lexer.next() {
            body.push(self.parse_token(token));
        }

        Program { body }
    }

    pub fn parse_token(&mut self, token: Token) -> Expression {
        match token {
            Token::Number(number) => Expression::NumberLiteral(number),
            Token::String(string) => Expression::StringLiteral(string),
            Token::OpenParenthesis => self.parse_function_call(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    pub fn parse_function_call(&mut self) -> Expression {
        let name = match self.lexer.next() {
            Some(Token::Identifier(ident)) => ident,
            Some(wrong) => panic!("Unexpected token: {:?}", wrong),
            None => panic!("Unexpected end of file"),
        };

        let mut params: Vec<Expression> = Vec::new();
        while let Some(token) = self.lexer.next() {
            if token == Token::CloseParenthesis {
                return Expression::FunctionCall { name, params };
            }
            params.push(self.parse_token(token));
        }

        panic!("Unexpected end of file");
    }
}
