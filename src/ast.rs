#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub identifier: String,
    // TODO:
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    return_value: Expression,
}

impl LetStatement {
    pub fn new(identifier: String, value: Expression) -> Self {
        LetStatement { identifier, value }
    }
}

impl ReturnStatement {
    pub fn new(return_value: Expression) -> Self {
        ReturnStatement { return_value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {}

impl Expression {
    pub fn new() -> Self {
        Expression {}
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}
