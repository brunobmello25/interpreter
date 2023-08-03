use std::fmt::Display;

use super::{
    operator::{InfixOperator, PrefixOperator},
    statement::Statement,
};

#[derive(PartialEq, Debug)]
pub enum Expression {
    Int(i64),
    Bool(bool),
    Identifier(String),
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Prefix {
        rhs: Box<Expression>,
        operator: PrefixOperator,
    },
    Infix {
        lhs: Box<Expression>,
        operator: InfixOperator,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn identifier(identifier: impl Into<String>) -> Self {
        Expression::Identifier(identifier.into())
    }

    pub fn function(parameters: Vec<Expression>, body: Vec<Statement>) -> Self {
        Expression::Function { parameters, body }
    }

    pub fn prefix(rhs: Expression, operator: PrefixOperator) -> Self {
        Expression::Prefix {
            rhs: Box::new(rhs),
            operator,
        }
    }

    pub fn infix(lhs: Expression, rhs: Expression, operator: InfixOperator) -> Self {
        Expression::Infix {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operator,
        }
    }

    pub fn r#if(
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    ) -> Self {
        Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        }
    }

    pub fn call(function: Expression, arguments: Vec<Expression>) -> Self {
        Expression::Call {
            function: Box::new(function),
            arguments,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expression::Infix { lhs, operator, rhs } => write!(f, "({} {} {})", lhs, operator, rhs),
            Expression::Prefix { operator, rhs } => write!(f, "({}{})", operator, rhs),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::Int(i) => write!(f, "{}", i),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => write!(
                f,
                "if {} {{ {} }} else {{ {} }}",
                condition,
                consequence
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(" "),
                alternative
                    .as_ref()
                    .map(|s| {
                        s.iter()
                            .map(|s| format!("{}", s))
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                    .unwrap_or_else(|| "".to_string())
            ),
            Expression::Function { parameters, body } => write!(
                f,
                "fn({}) {{ {} }}",
                parameters
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", "),
                body.iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expression::Call {
                function,
                arguments,
            } => write!(
                f,
                "{}({})",
                function,
                arguments
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix() {
        let infix = Expression::infix(Expression::Int(1), Expression::Int(2), InfixOperator::Add);
        assert_eq!(format!("{}", infix), "(1 + 2)");
    }

    #[test]
    fn test_prefix() {
        let prefix = Expression::prefix(Expression::Int(1), PrefixOperator::Negative);
        assert_eq!(format!("{}", prefix), "(-1)");
    }

    #[test]
    fn test_bool() {
        let bool_expr = Expression::Bool(true);
        assert_eq!(format!("{}", bool_expr), "true");
    }

    #[test]
    fn test_int() {
        let int_expr = Expression::Int(1);
        assert_eq!(format!("{}", int_expr), "1");
    }

    #[test]
    fn test_identifier() {
        let identifier_expr = Expression::identifier("foo");
        assert_eq!(format!("{}", identifier_expr), "foo");
    }

    #[test]
    fn test_condition() {
        let condition = Expression::If {
            condition: Box::new(Expression::Bool(true)),
            consequence: vec![Statement::Expression(Expression::Int(1))],
            alternative: Some(vec![Statement::Expression(Expression::Int(2))]),
        };
        assert_eq!(format!("{}", condition), "if true { 1 } else { 2 }");
    }

    #[test]
    fn test_function() {
        let function = Expression::Function {
            parameters: vec![Expression::identifier("foo")],
            body: vec![Statement::Expression(Expression::Int(1))],
        };
        assert_eq!(format!("{}", function), "fn(foo) { 1 }");
    }

    #[test]
    fn test_call() {
        let call = Expression::Call {
            function: Box::new(Expression::identifier("foo")),
            arguments: vec![Expression::Int(1)],
        };
        assert_eq!(format!("{}", call), "foo(1)");
    }
}
