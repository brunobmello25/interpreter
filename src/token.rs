use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, location: Location) -> Self {
        Self {
            token_type,
            location,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Let,
    Identifier(String),
    Assign,
    Integer(String),
    Comma,
    Function,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Illegal(char),
    EOF,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
    Modulo,
}

impl TokenType {
    pub fn identifier(ident: impl Into<String>) -> TokenType {
        TokenType::Identifier(ident.into())
    }

    pub fn integer(integer: impl Into<String>) -> TokenType {
        TokenType::Integer(integer.into())
    }
}
