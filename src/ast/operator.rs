#[derive(PartialEq, Debug)]
pub enum PrefixOperator {
    Not,
    Negative,
}

#[derive(PartialEq, Debug)]
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
    GreaterThanOrEqual,
    LessThanOrEqual,
}
