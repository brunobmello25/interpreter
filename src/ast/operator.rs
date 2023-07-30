use std::fmt::Display;

use crate::token::Token;

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
}

impl From<&Token> for InfixOperator {
    fn from(token: &Token) -> Self {
        match token {
            Token::Eq => InfixOperator::Equal,
            Token::NotEq => InfixOperator::NotEqual,
            Token::Plus => InfixOperator::Add,
            Token::Minus => InfixOperator::Sub,
            Token::Asterisk => InfixOperator::Mult,
            Token::Slash => InfixOperator::Div,
            Token::GT => InfixOperator::GreaterThan,
            Token::LT => InfixOperator::LessThan,
            Token::Let => todo!(),
            Token::Identifier(_) => todo!(),
            Token::Assign => todo!(),
            Token::Integer(_) => todo!(),
            Token::Comma => todo!(),
            Token::Function => todo!(),
            Token::LParen => todo!(),
            Token::RParen => todo!(),
            Token::LBrace => todo!(),
            Token::RBrace => todo!(),
            Token::Semicolon => todo!(),
            Token::Illegal(_) => todo!(),
            Token::EOF => todo!(),
            Token::Bang => todo!(),
            Token::True => todo!(),
            Token::False => todo!(),
            Token::If => todo!(),
            Token::Else => todo!(),
            Token::Return => todo!(),
        }
    }
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
