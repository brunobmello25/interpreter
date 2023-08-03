use super::{expression::Expression, program::Program, statement::Statement};

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}

impl Into<Node> for Program {
    fn into(self) -> Node {
        Node::Program(self)
    }
}

impl Into<Node> for Expression {
    fn into(self) -> Node {
        Node::Expression(self)
    }
}

impl Into<Node> for Statement {
    fn into(self) -> Node {
        Node::Statement(self)
    }
}
