use crate::parser::{Expression, Program};

impl Expression {
    pub fn emit(&self) -> String {
        match self {
            Expression::NumberLiteral(number) => number.to_string(),
            Expression::StringLiteral(string) => format!(r#""{}""#, string),
            Expression::FunctionCall { name, params } => format!(
                "{}({})",
                name,
                params
                    .iter()
                    .map(|expr| expr.emit())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Program {
    pub fn emit(&self) -> String {
        self.body
            .iter()
            .map(|expr| expr.emit())
            .collect::<Vec<String>>()
            .join(";\n")
    }
}
