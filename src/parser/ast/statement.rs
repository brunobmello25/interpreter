use std::fmt::Display;

use super::expression::Expression;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    Expression(Expression),
}

impl Statement {
    pub fn r#let(name: impl Into<String>, value: Expression) -> Self {
        Statement::Let {
            name: name.into(),
            value,
        }
    }

    pub fn r#return(value: Expression) -> Self {
        Statement::Return { value }
    }

    pub fn expression(expression: Expression) -> Self {
        Statement::Expression(expression)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Statement::Let { name, value } => write!(f, "let {} = {}", name, value),
            Statement::Return { value } => write!(f, "return {}", value),
            Statement::Expression(expression) => write!(f, "{}", expression),
        }
    }
}
