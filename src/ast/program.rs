use core::fmt;

use super::statement::Statement;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let program = self
            .statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{program}")
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{expression::Expression, statement::Statement};

    use super::Program;

    #[test]
    fn test_program_display() {
        let mut program = Program::new();
        program.statements.push(Statement::r#let(
            "myVar",
            Expression::identifier("anotherVar"),
        ));

        assert_eq!(program.to_string(), "let myVar = anotherVar;")
    }
}
