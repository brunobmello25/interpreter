use super::{expression::Expression, program::Program, statement::Statement};

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}
