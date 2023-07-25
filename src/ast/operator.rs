use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator {
    Not,
    Negate,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InfixOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Modulo,
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::Not => write!(f, "!"),
            PrefixOperator::Negate => write!(f, "-"),
        }
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Add => write!(f, "+"),
            InfixOperator::Subtract => write!(f, "-"),
            InfixOperator::Divide => write!(f, "/"),
            InfixOperator::Multiply => write!(f, "*"),
            InfixOperator::Equal => write!(f, "=="),
            InfixOperator::NotEqual => write!(f, "!="),
            InfixOperator::LessThan => write!(f, "<"),
            InfixOperator::GreaterThan => write!(f, ">"),
            InfixOperator::LessThanOrEqual => write!(f, "<="),
            InfixOperator::GreaterThanOrEqual => write!(f, ">="),
            InfixOperator::Modulo => write!(f, "%"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prefix_operator_display() {
        use super::PrefixOperator::*;
        assert_eq!(format!("{}", Not), "!");
        assert_eq!(format!("{}", Negate), "-");
    }

    #[test]
    fn test_infix_operator_display() {
        use super::InfixOperator::*;
        assert_eq!(format!("{}", Add), "+");
        assert_eq!(format!("{}", Subtract), "-");
        assert_eq!(format!("{}", Divide), "/");
        assert_eq!(format!("{}", Multiply), "*");
        assert_eq!(format!("{}", Equal), "==");
        assert_eq!(format!("{}", NotEqual), "!=");
        assert_eq!(format!("{}", LessThan), "<");
        assert_eq!(format!("{}", GreaterThan), ">");
        assert_eq!(format!("{}", LessThanOrEqual), "<=");
        assert_eq!(format!("{}", GreaterThanOrEqual), ">=");
        assert_eq!(format!("{}", Modulo), "%");
    }
}
