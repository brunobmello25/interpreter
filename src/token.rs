use std::fmt::Display;

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenType::Let => write!(f, "let"),
            TokenType::Identifier(identifier) => write!(f, "identifier {}", identifier),
            TokenType::Assign => write!(f, "assign"),
            TokenType::Integer(integer) => write!(f, "integer {}", integer),
            TokenType::Comma => write!(f, ","),
            TokenType::Function => write!(f, "function"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Illegal(illegal) => write!(f, "illegal {}", illegal),
            TokenType::EOF => write!(f, "end of file"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::LT => write!(f, "<"),
            TokenType::GT => write!(f, ">"),
            TokenType::True => write!(f, "boolean true"),
            TokenType::False => write!(f, "boolean false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),
            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
            TokenType::Modulo => write!(f, "%"),
        }
    }
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        match self {
            Self::Let => Self::Let,
            Self::Identifier(arg0) => Self::Identifier(arg0.clone()),
            Self::Assign => Self::Assign,
            Self::Integer(arg0) => Self::Integer(arg0.clone()),
            Self::Comma => Self::Comma,
            Self::Function => Self::Function,
            Self::LParen => Self::LParen,
            Self::RParen => Self::RParen,
            Self::LBrace => Self::LBrace,
            Self::RBrace => Self::RBrace,
            Self::Semicolon => Self::Semicolon,
            Self::Illegal(arg0) => Self::Illegal(arg0.clone()),
            Self::EOF => Self::EOF,
            Self::Plus => Self::Plus,
            Self::Minus => Self::Minus,
            Self::Bang => Self::Bang,
            Self::Asterisk => Self::Asterisk,
            Self::Slash => Self::Slash,
            Self::LT => Self::LT,
            Self::GT => Self::GT,
            Self::True => Self::True,
            Self::False => Self::False,
            Self::If => Self::If,
            Self::Else => Self::Else,
            Self::Return => Self::Return,
            Self::Eq => Self::Eq,
            Self::NotEq => Self::NotEq,
            Self::Modulo => Self::Modulo,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            token_type: self.token_type.clone(),
            location: self.location.clone(),
        }
    }
}
