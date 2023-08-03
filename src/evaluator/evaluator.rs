use std::{fmt::Display, result};

use crate::parser::ast::{
    expression::Expression,
    node::Node,
    operator::{InfixOperator, PrefixOperator},
    statement::Statement,
};

use super::object::Object;

#[derive(Debug)]
pub struct EvaluationError {
    msg: String,
}

impl EvaluationError {
    pub fn new(msg: impl Into<String>) -> Self {
        EvaluationError { msg: msg.into() }
    }
}

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

    pub fn eval(&self, node: impl Into<Node>) -> Result<Object, EvaluationError> {
        let node = node.into();
        match node {
            Node::Expression(expression) => self.eval_expression(expression),
            Node::Statement(statement) => self.eval_statement(statement),
            Node::Program(program) => self.eval_statements(program.statements),
        }
    }

    fn eval_statements(&self, statements: Vec<Statement>) -> Result<Object, EvaluationError> {
        let mut result: Option<Object> = None;

        for statement in statements {
            result = Some(self.eval(statement)?);
        }

        match result {
            Some(object) => Ok(object),
            None => Err(EvaluationError::new("program did not have any statements")),
        }
    }

    fn eval_statement(&self, statement: Statement) -> Result<Object, EvaluationError> {
        match statement {
            Statement::Let { .. } => todo!(),
            Statement::Return { .. } => todo!(),
            Statement::Expression(expression) => self.eval(expression),
        }
    }

    fn eval_expression(&self, expression: Expression) -> Result<Object, EvaluationError> {
        match expression {
            Expression::Int(int) => Ok(Object::Integer(int)),
            Expression::Bool(boolean) => Ok(Object::Boolean(boolean)),
            Expression::Identifier(_) => todo!(),
            Expression::If { .. } => todo!(),
            Expression::Function { .. } => todo!(),
            Expression::Call { .. } => todo!(),
            Expression::Prefix { operator, rhs } => self.eval_prefix_expression(operator, *rhs),
            Expression::Infix { rhs, operator, lhs } => {
                self.eval_infix_expression(operator, *lhs, *rhs)
            }
        }
    }

    fn eval_prefix_expression(
        &self,
        operator: PrefixOperator,
        rhs: Expression,
    ) -> Result<Object, EvaluationError> {
        let rhs = self.eval(rhs)?;
        match operator {
            PrefixOperator::Not => self.eval_bang_operator_prefix_expression(rhs),
            PrefixOperator::Negative => self.eval_negative_operator_prefix_expression(rhs),
        }
    }

    fn eval_infix_expression(
        &self,
        operator: InfixOperator,
        lhs: Expression,
        rhs: Expression,
    ) -> Result<Object, EvaluationError> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        match (operator, lhs, rhs) {
            (InfixOperator::Add, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 + int2))
            }
            (InfixOperator::Add, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Add, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Add, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Add, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Add, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Add, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Add, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Add, Object::Null, Object::Null) => todo!(),
            (InfixOperator::Sub, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 - int2))
            }
            (InfixOperator::Sub, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Sub, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Sub, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Sub, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Sub, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Sub, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Sub, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Sub, Object::Null, Object::Null) => todo!(),
            (InfixOperator::Mult, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 * int2))
            }
            (InfixOperator::Mult, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Mult, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Mult, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Mult, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Mult, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Mult, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Mult, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Mult, Object::Null, Object::Null) => todo!(),
            (InfixOperator::Div, Object::Integer(int1), Object::Integer(int2)) => {
                if int2 == 0 {
                    return Err(EvaluationError::new("cannot divide by zero"));
                }
                Ok(Object::Integer(int1 / int2))
            }
            (InfixOperator::Div, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Div, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Div, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Div, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Div, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Div, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Div, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Div, Object::Null, Object::Null) => todo!(),
            (InfixOperator::Modulo, Object::Integer(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Modulo, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Modulo, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Modulo, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Modulo, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Modulo, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Modulo, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Modulo, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Modulo, Object::Null, Object::Null) => todo!(),
            (InfixOperator::Equal, Object::Integer(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Equal, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Equal, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::Equal, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::Equal, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::Equal, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::Equal, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::Equal, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::Equal, Object::Null, Object::Null) => todo!(),
            (InfixOperator::NotEqual, Object::Integer(_), Object::Integer(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::NotEqual, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::NotEqual, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::NotEqual, Object::Null, Object::Null) => todo!(),
            (InfixOperator::GreaterThan, Object::Integer(_), Object::Integer(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::GreaterThan, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::GreaterThan, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::GreaterThan, Object::Null, Object::Null) => todo!(),
            (InfixOperator::LessThan, Object::Integer(_), Object::Integer(_)) => todo!(),
            (InfixOperator::LessThan, Object::Integer(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::LessThan, Object::Integer(_), Object::Null) => todo!(),
            (InfixOperator::LessThan, Object::Boolean(_), Object::Integer(_)) => todo!(),
            (InfixOperator::LessThan, Object::Boolean(_), Object::Boolean(_)) => todo!(),
            (InfixOperator::LessThan, Object::Boolean(_), Object::Null) => todo!(),
            (InfixOperator::LessThan, Object::Null, Object::Integer(_)) => todo!(),
            (InfixOperator::LessThan, Object::Null, Object::Boolean(_)) => todo!(),
            (InfixOperator::LessThan, Object::Null, Object::Null) => todo!(),
        }
    }

    fn eval_bang_operator_prefix_expression(&self, rhs: Object) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
            Object::Null => todo!(),
            Object::Integer(integer) => Ok(Object::Boolean(integer == 0)),
        }
    }

    fn eval_negative_operator_prefix_expression(
        &self,
        rhs: Object,
    ) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Integer(integer) => Ok(Object::Integer(-integer)),
            Object::Boolean(_) => Ok(Object::Null),
            Object::Null => todo!(),
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
    // #[ignore]
    fn test_bang_prefix_expression() {
        let tests = vec![
            ("!true", false),
            ("!5", false),
            ("!!5", true),
            ("!!true", true),
            ("!false", true),
            ("!!false", false),
        ];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Boolean(test.1));
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![("true", true), ("false", false)];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Boolean(test.1));
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", Object::Integer(5)),
            ("10", Object::Integer(10)),
            ("-5", Object::Integer(-5)),
            ("-10", Object::Integer(-10)),
            ("-true", Object::Null),
            ("-false", Object::Null),
            ("5", Object::Integer(5)),
            ("10", Object::Integer(10)),
            ("-5", Object::Integer(-5)),
            ("-10", Object::Integer(-10)),
            ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
            ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
            ("-50 + 100 + -50", Object::Integer(0)),
            ("5 * 2 + 10", Object::Integer(20)),
            ("5 + 2 * 10", Object::Integer(25)),
            ("20 + 2 * -10", Object::Integer(0)),
            ("50 / 2 * 2 + 10", Object::Integer(60)),
            ("2 * (5 + 10)", Object::Integer(30)),
            ("3 * 3 * 3 + 10", Object::Integer(37)),
            ("3 * (3 * 3) + 10", Object::Integer(37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
        ];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), test.1);
        }
    }

    fn make_program_node(input: &str) -> Node {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        return Node::Program(program);
    }
}
