use crate::token::Token;

#[derive(Debug)]
enum Node {
    Statement(Statement),
    Expression(Expression),
}

impl Node {
    fn token_literal(&self) -> String {
        match self {
            Node::Statement(stmt) => stmt.token_literal(),
            Node::Expression(exp) => exp.token_literal(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let {
        token: Token,
        name: String,
        // value: Expression,
    },
    Return {
        token: Token,
        // value: Expression,
    },
}

impl Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.token_literal(),
            Statement::Return { token, .. } => token.token_literal(),
        }
    }

    pub fn r#let(token: Token, name: impl Into<String>) -> Self {
        Statement::Let {
            token,
            name: name.into(),
            // value,
        }
    }

    pub fn r#return(token: Token) -> Self {
        Statement::Return { token }
    }
}

#[derive(PartialEq, Debug)]
enum Operator {}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Infix {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    Prefix {
        operator: Operator,
        rhs: Box<Expression>,
    },
    Call {
        function_name: String,
        arguments: Vec<Expression>,
    },
    Identifier {
        identifier: String,
    },
    Function {
        arguments: Vec<Expression>,
        body: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
    },
}

impl Expression {
    fn token_literal(&self) -> String {
        match self {
            _ => todo!(),
        }
    }

    pub fn identifier(identifier: impl Into<String>) -> Self {
        Expression::Identifier {
            identifier: identifier.into(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[cfg(test)]
mod tests {}
