use std::fmt;

use super::{
    operator::{InfixOperator, PrefixOperator},
    statement::Statement,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Prefix {
        operator: PrefixOperator,
        rhs: Box<Expression>,
    },
    Infix {
        operator: InfixOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Identifier(String),
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Integer(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
}

impl Expression {
    pub fn prefix(operator: PrefixOperator, rhs: Box<Expression>) -> Self {
        Expression::Prefix { operator, rhs }
    }

    pub fn infix(operator: InfixOperator, lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Expression::Infix { operator, lhs, rhs }
    }

    pub fn call(function: Box<Expression>, arguments: Vec<Expression>) -> Self {
        Expression::Call {
            function,
            arguments,
        }
    }

    pub fn identifier(id: impl Into<String>) -> Self {
        Expression::Identifier(id.into())
    }

    pub fn function(parameters: Vec<String>, body: Vec<Statement>) -> Self {
        Expression::Function { parameters, body }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Prefix { operator, rhs } => write!(f, "{}{}", operator, rhs),
            Expression::Infix { operator, lhs, rhs } => write!(f, "{}{}{}", lhs, operator, rhs),
            Expression::Call {
                function,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{}{}", function, arguments)
            }
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::Function { parameters, body } => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| parameter.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "fn({parameters}) {{")?;

                for statement in body {
                    write!(f, "{statement}")?;
                }

                write!(f, "}}")
            }
            Expression::Integer(value) => write!(f, "{value}"),
            Expression::Boolean(value) => write!(f, "{value}"),
            Expression::String(value) => write!(f, "{value}"),
            Expression::Array(values) => {
                let values = values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[{values}]")
            }
        }
    }
}
