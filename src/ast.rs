pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

struct LetStatement {}

struct ReturnStatement {}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}
