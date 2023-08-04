use std::fmt::Display;

use crate::parser::ast::{
    expression::Expression,
    node::Node,
    operator::{InfixOperator, PrefixOperator},
    program::Program,
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
            let evaluated = self.eval(statement)?;

            if let Object::ReturnValue(_) = evaluated {
                return Ok(evaluated);
            }

            result = Some(evaluated);
        }

        Ok(result.unwrap_or(Object::Null))
    }

    fn eval_statement(&self, statement: Statement) -> Result<Object, EvaluationError> {
        match statement {
            Statement::Let { .. } => todo!(),
            Statement::Return { value } => {
                let value = self.eval(value)?;
                Ok(Object::return_value(value))
            }
            Statement::Expression(expression) => self.eval(expression),
        }
    }

    fn eval_expression(&self, expression: Expression) -> Result<Object, EvaluationError> {
        match expression {
            Expression::Int(int) => Ok(Object::Integer(int)),
            Expression::Bool(boolean) => Ok(Object::Boolean(boolean)),
            Expression::Identifier(_) => todo!(),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(*condition, consequence, alternative),
            Expression::Function { .. } => todo!(),
            Expression::Call { .. } => todo!(),
            Expression::Prefix { operator, rhs } => self.eval_prefix_expression(operator, *rhs),
            Expression::Infix { rhs, operator, lhs } => {
                self.eval_infix_expression(operator, *lhs, *rhs)
            }
        }
    }

    fn eval_if_expression(
        &self,
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    ) -> Result<Object, EvaluationError> {
        let condition = self.eval(condition)?;

        if self.is_truthy(condition) {
            self.eval_statements(consequence)
        } else if let Some(alternative) = alternative {
            self.eval_statements(alternative)
        } else {
            Ok(Object::Null)
        }
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Integer(integer) => integer != 0,
            Object::Boolean(boolean) => boolean,
            _ => todo!(),
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
            (InfixOperator::Sub, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 - int2))
            }
            (InfixOperator::Mult, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 * int2))
            }
            (InfixOperator::Div, Object::Integer(int1), Object::Integer(int2)) => {
                if int2 == 0 {
                    return Err(EvaluationError::new("cannot divide by zero"));
                }
                Ok(Object::Integer(int1 / int2))
            }
            (InfixOperator::Modulo, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Integer(int1 % int2))
            }
            (InfixOperator::Equal, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Boolean(int1 == int2))
            }
            (InfixOperator::Equal, Object::Boolean(bool1), Object::Boolean(bool2)) => {
                Ok(Object::Boolean(bool1 == bool2))
            }
            (InfixOperator::NotEqual, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Boolean(int1 != int2))
            }
            (InfixOperator::GreaterThan, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Boolean(int1 > int2))
            }
            (InfixOperator::LessThan, Object::Integer(int1), Object::Integer(int2)) => {
                Ok(Object::Boolean(int1 < int2))
            }
            (InfixOperator::NotEqual, Object::Boolean(bool1), Object::Boolean(bool2)) => {
                Ok(Object::Boolean(bool1 != bool2))
            }
            x => todo!("need to implement infix operator for {:?}", x),
        }
    }

    fn eval_bang_operator_prefix_expression(&self, rhs: Object) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
            Object::Integer(integer) => Ok(Object::Boolean(integer == 0)),
            x => todo!(),
        }
    }

    fn eval_negative_operator_prefix_expression(
        &self,
        rhs: Object,
    ) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Integer(integer) => Ok(Object::Integer(-integer)),
            Object::Boolean(_) => Ok(Object::Null),
            x => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{
        evaluator::object::Object,
        lexer::lexer::Lexer,
        parser::{ast::node::Node, parser::Parser},
    };

    use super::Evaluator;

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", Object::return_value(Object::Integer(10))),
            ("return 10; 9;", Object::return_value(Object::Integer(10))),
            (
                "return 2 * 5; 9;",
                Object::return_value(Object::Integer(10)),
            ),
            (
                "9; return 2 * 5; 9;",
                Object::return_value(Object::Integer(10)),
            ),
        ];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), test.1);
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1) { 10 }", Object::Integer(10)),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
            (
                indoc! {"
                    if (10 > 1) {
                        if (10 > 1) {
                            return 10;
                        }
                        return 1;
                    }
                "},
                Object::return_value(Object::Integer(10)),
            ),
        ];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), test.1);
        }
    }

    #[test]
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
    fn test_eval_modulo() {
        let tests = vec![("10 % 2", 0), ("2 % 3", 2), ("5 % 2", 1)];
        for test in tests {
            let program = make_program_node(test.0);
            let evaluator = Evaluator::new();
            let evaluated = evaluator.eval(program);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Integer(test.1));
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
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
