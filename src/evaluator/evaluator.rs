use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::parser::ast::{
    expression::Expression,
    node::Node,
    operator::{InfixOperator, PrefixOperator},
    statement::Statement,
};

use super::{environment::Environment, object::Object};

#[derive(Debug)]
pub struct EvaluationError {
    #[allow(dead_code)]
    msg: String,
}

impl EvaluationError {
    pub fn new(msg: impl Into<String>) -> Self {
        EvaluationError { msg: msg.into() }
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(
        &mut self,
        node: impl Into<Node>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let node = node.into();
        match node {
            Node::Expression(expression) => self.eval_expression(expression, environment),
            Node::Statement(statement) => self.eval_statement(statement, environment),
            Node::Program(program) => self.eval_statements(program.statements, environment),
        }
    }

    fn eval_statements(
        &mut self,
        statements: Vec<Statement>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let mut result: Option<Object> = None;

        for statement in statements {
            let evaluated = self.eval(statement, Rc::clone(&environment))?;

            if let Object::ReturnValue(_) = evaluated {
                return Ok(evaluated);
            }

            result = Some(evaluated);
        }

        Ok(result.unwrap_or(Object::Null))
    }

    fn eval_statement(
        &mut self,
        statement: Statement,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        match statement {
            Statement::Let { name, value } => self.eval_let_statement(name, value, environment),
            Statement::Return { value } => {
                let value = self.eval(value, environment)?;
                Ok(Object::return_value(value))
            }
            Statement::Expression(expression) => self.eval(expression, environment),
            Statement::Block(statements) => self.eval_statements(statements, environment),
        }
    }

    fn eval_let_statement(
        &mut self,
        name: String,
        value: Expression,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let value = self.eval(value, Rc::clone(&environment))?;

        environment.borrow_mut().set(&name, value.clone());

        Ok(value)
    }

    fn eval_expression(
        &mut self,
        expression: Expression,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        match expression {
            Expression::Int(int) => Ok(Object::Integer(int)),
            Expression::Bool(boolean) => Ok(Object::Boolean(boolean)),
            Expression::Identifier(identifier) => self.eval_identifier(identifier, environment),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(*condition, consequence, alternative, environment),
            Expression::Function { parameters, body } => {
                self.eval_function(parameters, body, environment)
            }
            Expression::Call {
                function,
                arguments,
            } => self.eval_call(*function, arguments, environment),
            Expression::Prefix { operator, rhs } => {
                self.eval_prefix_expression(operator, *rhs, environment)
            }
            Expression::Infix { rhs, operator, lhs } => {
                self.eval_infix_expression(operator, *lhs, *rhs, environment)
            }
            Expression::Null => Ok(Object::Null),
        }
    }

    fn eval_call(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let function = self.eval(function, Rc::clone(&environment))?;

        let Object::Function { parameters,environment, body } = function else {
            return Err(EvaluationError::new(format!("not a function: {}", function)));
        };

        if parameters.len() != arguments.len() {
            return Err(EvaluationError::new(format!(
                "wrong number of arguments: got {}, but function wants {}",
                arguments.len(),
                parameters.len()
            )));
        }

        let local_env = Environment::with_outer(Rc::clone(&environment));

        for (parameter, argument) in parameters.iter().zip(arguments) {
            let argument = self.eval(argument, Rc::clone(&environment))?;

            local_env.borrow_mut().set(parameter, argument);
        }

        let body = match self.eval(Statement::Block(body), Rc::clone(&local_env))? {
            Object::ReturnValue(value) => *value,
            value => value,
        };

        Ok(body)
    }

    fn eval_function(
        &mut self,
        parameters: Vec<String>,
        body: Vec<Statement>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        Ok(Object::Function {
            parameters,
            body,
            environment: Environment::with_outer(Rc::clone(&environment)),
        })
    }

    fn eval_if_expression(
        &mut self,
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let condition = self.eval(condition, Rc::clone(&environment))?;

        if self.is_truthy(condition) {
            self.eval_statements(consequence, Rc::clone(&environment))
        } else if let Some(alternative) = alternative {
            self.eval_statements(alternative, environment)
        } else {
            Ok(Object::Null)
        }
    }

    fn eval_identifier(
        &mut self,
        identifier: String,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        match environment.borrow().get(&identifier) {
            Some(object) => Ok(object),
            None => Err(EvaluationError::new(format!(
                "identifier not found: {}",
                identifier
            ))),
        }
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Integer(integer) => integer != 0,
            Object::Boolean(boolean) => boolean,
            Object::Null => false,
            Object::ReturnValue(value) => self.is_truthy(*value),
            Object::Function { .. } => todo!(),
        }
    }

    fn eval_prefix_expression(
        &mut self,
        operator: PrefixOperator,
        rhs: Expression,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let rhs = self.eval(rhs, environment)?;
        match operator {
            PrefixOperator::Not => self.eval_bang_operator_prefix_expression(rhs),
            PrefixOperator::Negative => self.eval_negative_operator_prefix_expression(rhs),
        }
    }

    fn eval_infix_expression(
        &mut self,
        operator: InfixOperator,
        lhs: Expression,
        rhs: Expression,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, EvaluationError> {
        let lhs = self.eval(lhs, Rc::clone(&environment))?;
        let rhs = self.eval(rhs, Rc::clone(&environment))?;

        match (&operator, &lhs, &rhs) {
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
                if *int2 == 0 {
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
            _ => {
                return Err(EvaluationError::new(format!(
                    "invalid operation: {} {} {}",
                    lhs, operator, rhs
                )))
            }
        }
    }

    fn eval_bang_operator_prefix_expression(&self, rhs: Object) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
            Object::Integer(integer) => Ok(Object::Boolean(integer == 0)),
            x => Err(EvaluationError::new(format!("invalid operation: !{}", x))),
        }
    }

    fn eval_negative_operator_prefix_expression(
        &self,
        rhs: Object,
    ) -> Result<Object, EvaluationError> {
        match rhs {
            Object::Integer(integer) => Ok(Object::Integer(-integer)),
            x => Err(EvaluationError::new(format!("invalid operation: -{}", x))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use indoc::indoc;

    use crate::{
        evaluator::{environment::Environment, object::Object},
        lexer::lexer::Lexer,
        parser::{
            ast::{expression::Expression, operator::InfixOperator, statement::Statement},
            parser::Parser,
        },
    };

    use super::{EvaluationError, Evaluator};

    #[test]
    fn test_closures() {
        let input = indoc! {"
            let newAdder = fn(x) {
                fn(y) { x + y };
            };
            let addTwo = newAdder(2);
            addTwo(2);
        "};
        let evaluated = evaluate(input);
        assert!(evaluated.is_ok());
        assert_eq!(evaluated.unwrap(), Object::Integer(4));
    }

    #[test]
    fn test_apply_function() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];
        for test in tests {
            let evaluated = evaluate(test.0);
            assert_eq!(evaluated.unwrap(), Object::Integer(test.1));
        }
    }

    #[test]
    fn test_function_object() {
        let evaluated = evaluate("fn(x) { x + 2 };");
        assert_eq!(
            evaluated.unwrap(),
            Object::Function {
                parameters: vec!["x".to_string()],
                body: vec![Statement::Expression(Expression::infix(
                    Expression::identifier("x"),
                    Expression::Int(2),
                    InfixOperator::Add
                ))],
                environment: Environment::with_outer(Environment::new()),
            }
        );
    }

    #[test]
    fn test_eval_not_null() {
        let evaluated = evaluate("!null");
        assert_eq!(evaluated.unwrap_err().msg, "invalid operation: !null");
    }

    #[test]
    fn test_identifier_not_found() {
        let evaluated = evaluate("foobar");
        assert_eq!(evaluated.unwrap_err().msg, "identifier not found: foobar");
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; a;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];
        for test in tests {
            let evaluated = evaluate(test.0);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Integer(test.1));
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "invalid operation: 5 + true"),
            ("false + 5;", "invalid operation: false + 5"),
            ("5 + true; 5;", "invalid operation: 5 + true"),
            ("-true", "invalid operation: -true"),
            ("true + false;", "invalid operation: true + false"),
            ("5; true + false; 5", "invalid operation: true + false"),
            (
                "if (10 > 1) { true + false; }",
                "invalid operation: true + false",
            ),
            (
                indoc! {"
                    if (10 > 1) {
                        if (10 > 1) {
                            return true + false;
                        }
                        return 1;
                    }
                "},
                "invalid operation: true + false",
            ),
        ];
        for test in tests {
            let evaluated = evaluate(test.0);
            assert!(evaluated.is_err());
            assert_eq!(evaluated.unwrap_err().msg, test.1);
        }
    }

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
            let evaluated = evaluate(test.0);
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
            let evaluated = evaluate(test.0);
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
            let evaluated = evaluate(test.0);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), Object::Boolean(test.1));
        }
    }

    #[test]
    fn test_eval_modulo() {
        let tests = vec![("10 % 2", 0), ("2 % 3", 2), ("5 % 2", 1)];
        for test in tests {
            let evaluated = evaluate(test.0);
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
            let evaluated = evaluate(test.0);
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
            let evaluated = evaluate(test.0);
            assert!(evaluated.is_ok());
            assert_eq!(evaluated.unwrap(), test.1);
        }
    }

    fn evaluate(input: &str) -> Result<Object, EvaluationError> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let environment = Environment::new();
        let mut evaluator = Evaluator::new();
        evaluator.eval(program, Rc::clone(&environment))
    }
}
