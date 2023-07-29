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
