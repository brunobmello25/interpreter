use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::parser::ast::statement::Statement;

use super::environment::Environment;

#[derive(PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
        environment: Rc<RefCell<Environment>>,
    },
}

impl Object {
    pub fn return_value(value: Object) -> Self {
        Object::ReturnValue(Box::new(value))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::ReturnValue(value) => write!(f, "{}", *value),
            Object::Null => write!(f, "null"),
            Object::Function {
                body, parameters, ..
            } => {
                let mut result = String::new();
                result.push_str("fn");
                result.push('(');
                for (i, parameter) in parameters.iter().enumerate() {
                    result.push_str(&parameter);
                    if i != parameters.len() - 1 {
                        result.push_str(", ");
                    }
                }
                result.push(')');
                result.push_str(" {\n");
                for statement in body {
                    result.push_str(&format!("{}\n", statement));
                }
                result.push_str("}");
                write!(f, "{}", result)
            }
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "Integer({})", value),
            Object::Boolean(value) => write!(f, "Boolean({})", value),
            Object::ReturnValue(value) => write!(f, "ReturnValue({:?})", *value),
            Object::Null => write!(f, "Null"),
            Object::Function {
                parameters, body, ..
            } => {
                write!(
                    f,
                    "Function {{ parameters: {:?}, body: {:?}, environment: ... }}",
                    parameters, body
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        use super::Object;
        assert_eq!(format!("{}", Object::Integer(1)), "1");
        assert_eq!(format!("{}", Object::Boolean(true)), "true");
        assert_eq!(format!("{}", Object::Boolean(false)), "false");
        assert_eq!(
            format!("{}", Object::ReturnValue(Box::new(Object::Integer(1)))),
            "1"
        );
        assert_eq!(format!("{}", Object::Null), "null");
    }
}
