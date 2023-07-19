#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub identifier: String,
    // TODO:
    // pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {}

#[derive(Debug, PartialEq)]
pub struct Expression {}

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
