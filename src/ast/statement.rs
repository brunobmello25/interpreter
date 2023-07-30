use std::fmt::Display;

use crate::token::Token;

use super::expression::Expression;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let {
        name: String,
        // value: Expression,
    },
    Return {
        // value: Expression,
    },
    Expression(Expression),
}

impl Statement {
    pub fn r#let(token: Token, name: impl Into<String>) -> Self {
        Statement::Let {
            name: name.into(),
            // value,
        }
    }

    pub fn r#return(token: Token) -> Self {
        Statement::Return {}
    }

    pub fn expression(expression: Expression) -> Self {
        Statement::Expression(expression)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Statement::Let { name } => write!(f, "let {} = ", name),
            Statement::Return {} => write!(f, "return "),
            Statement::Expression(expression) => write!(f, "{}", expression),
        }
    }
}
