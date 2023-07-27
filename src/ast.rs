use crate::token::Token;

#[allow(dead_code)]
enum Node {
    Statement(Statement),
    Expression(Expression),
}

impl Node {
    fn token_literal(&self) -> String {
        match self {
            Node::Statement(stmt) => stmt.token_literal(),
            Node::Expression(_) => todo!(),
        }
    }
}

#[allow(dead_code)]
pub enum Statement {
    Let {
        token: Token,
        name: String,
        value: Expression,
    },
    Return {
        token: Token,
        value: Expression,
    },
}

impl Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.token_literal(),
            Statement::Return { token, .. } => token.token_literal(),
        }
    }
}

#[allow(dead_code)]
pub enum Expression {}

impl Expression {}

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
