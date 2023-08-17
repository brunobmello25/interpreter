use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum PrefixOperator {
    Not,
    Negative,
}

#[derive(PartialEq, Debug, Clone)]
pub enum InfixOperator {
    Add,
    Sub,
    Mult,
    Div,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InfixOperator::Add => write!(f, "+"),
            InfixOperator::Sub => write!(f, "-"),
            InfixOperator::Mult => write!(f, "*"),
            InfixOperator::Div => write!(f, "/"),
            InfixOperator::Modulo => write!(f, "%"),
            InfixOperator::Equal => write!(f, "=="),
            InfixOperator::NotEqual => write!(f, "!="),
            InfixOperator::GreaterThan => write!(f, ">"),
            InfixOperator::LessThan => write!(f, "<"),
        }
    }
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PrefixOperator::Not => write!(f, "!"),
            PrefixOperator::Negative => write!(f, "-"),
        }
    }
}
