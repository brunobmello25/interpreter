use std::fmt;

use super::expression::Expression;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let {
        identifier: String,
        // TODO: add expression
        // expression: Expression,
    },
    Return {
        // TODO: add expression
        // expression: Expression,
    },

    Expression(Expression),
}

impl Statement {
    pub fn r#let(identifier: impl Into<String>) -> Self {
        Statement::Let {
            identifier: identifier.into(),
        }
    }

    pub fn r#return() -> Self {
        Statement::Return {}
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { identifier } => write!(f, "let {} = ;", identifier),
            Statement::Return {} => write!(f, "return "),
            Statement::Expression(expression) => write!(f, "{expression}"),
        }
    }
}
