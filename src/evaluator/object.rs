use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::ReturnValue(value) => write!(f, "{}", *value),
            Object::Null => write!(f, "null"),
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
