pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

pub struct LetStatement {}

pub struct ReturnStatement {}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}
