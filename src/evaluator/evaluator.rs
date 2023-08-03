use std::fmt::Display;

use crate::parser::ast::{expression::Expression, node::Node, statement::Statement};

use super::object::Object;

#[derive(Debug)]
pub struct EvaluationError {}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "evaluation error")
    }
}

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, node: Node) -> Result<Object, EvaluationError> {
        match node {
            Node::Expression(expression) => self.eval_expression(expression),
            Node::Statement(statement) => self.eval_statement(statement),
            Node::Program(program) => self.eval_statements(program.statements),
        }
    }

    fn eval_statements(&self, statements: Vec<Statement>) -> Result<Object, EvaluationError> {
        let mut result: Option<Object> = None;

        for statement in statements {
            result = Some(self.eval(Node::Statement(statement))?);
        }

        match result {
            Some(object) => Ok(object),
            None => Err(EvaluationError {}),
        }
    }

    fn eval_statement(&self, statement: Statement) -> Result<Object, EvaluationError> {
        match statement {
            Statement::Let { .. } => todo!(),
            Statement::Return { .. } => todo!(),
            Statement::Expression(expression) => self.eval(Node::Expression(expression)),
        }
    }

    fn eval_expression(&self, expression: Expression) -> Result<Object, EvaluationError> {
        match expression {
            Expression::Int(int) => Ok(Object::Integer(int)),
            Expression::Bool(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::If { .. } => todo!(),
            Expression::Function { .. } => todo!(),
            Expression::Call { .. } => todo!(),
            Expression::Prefix { .. } => todo!(),
            Expression::Infix { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluator::object::Object,
        lexer::lexer::Lexer,
        parser::{ast::node::Node, parser::Parser},
    };

    use super::Evaluator;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![("5", 5), ("10", 10)];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Integer(test.1));
        }
    }

    fn make_program_node(input: &str) -> Node {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        return Node::Program(program);
    }
}
