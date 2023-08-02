#[derive(Debug, PartialEq)]
pub enum Token {
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

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Let => Self::Let,
            Self::Identifier(identifier) => Self::Identifier(identifier.clone()),
            Self::Assign => Self::Assign,
            Self::Integer(integer) => Self::Integer(integer.clone()),
            Self::Comma => Self::Comma,
            Self::Function => Self::Function,
            Self::LParen => Self::LParen,
            Self::RParen => Self::RParen,
            Self::LBrace => Self::LBrace,
            Self::RBrace => Self::RBrace,
            Self::Semicolon => Self::Semicolon,
            Self::Illegal(illegal) => Self::Illegal(illegal.clone()),
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

impl Token {
    pub fn identifier(ident: impl Into<String>) -> Token {
        Token::Identifier(ident.into())
    }

    pub fn integer(integer: impl Into<String>) -> Token {
        Token::Integer(integer.into())
    }
}
