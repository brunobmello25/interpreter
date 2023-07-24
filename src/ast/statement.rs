use std::fmt;

use super::expression::Expression;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let {
        identifier: String,
        value: Expression,
    },
    Return {
        value: Expression,
    },

    Expression(Expression),
}

impl Statement {
    pub fn r#let(identifier: impl Into<String>, value: Expression) -> Self {
        Statement::Let {
            identifier: identifier.into(),
            value,
        }
    }
    pub fn r#return(value: Expression) -> Self {
        Statement::Return { value }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: add expression
            Statement::Let { identifier, value } => write!(f, "let {} = {};", identifier, value),
            // TODO: add expression
            Statement::Return { value } => write!(f, "return {}", value),
            // TODO: add expression
            Statement::Expression(expression) => write!(f, "{expression}"),
        }
    }
}
