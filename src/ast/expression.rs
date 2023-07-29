use super::{
    operator::{InfixOperator, PrefixOperator},
    statement::Statement,
};

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Int(i64),
    Bool(bool),
    Identifier(String),
    String(String),
    Condition {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Prefix {
        rhs: Box<Expression>,
        operator: PrefixOperator,
    },
    Infix {
        lhs: Box<Expression>,
        operator: InfixOperator,
        rhs: Box<Expression>,
    },
    // Index {
    //     lhs: Expression,
    //     idx: Expression
    // }
    // Array(Vec<Expression>),
    // Map(Vec<(Expression, Expression)>),
    // TODO: map, array, index
}

impl Expression {
    fn token_literal(&self) -> String {
        match self {
            _ => todo!(),
        }
    }

    pub fn identifier(identifier: impl Into<String>) -> Self {
        Expression::Identifier(identifier.into())
    }
}
