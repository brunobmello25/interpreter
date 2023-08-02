#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
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

impl Clone for Token {
    fn clone(&self) -> Self {
        let token_type = match &self.token_type {
            TokenType::Let => TokenType::Let,
            TokenType::Identifier(identifier) => TokenType::identifier(identifier),
            TokenType::Assign => TokenType::Assign,
            TokenType::Integer(integer) => TokenType::Integer(integer.clone()),
            TokenType::Comma => TokenType::Comma,
            TokenType::Function => TokenType::Function,
            TokenType::LParen => TokenType::LParen,
            TokenType::RParen => TokenType::RParen,
            TokenType::LBrace => TokenType::LBrace,
            TokenType::RBrace => TokenType::RBrace,
            TokenType::Semicolon => TokenType::Semicolon,
            TokenType::Illegal(illegal) => TokenType::Illegal(illegal.clone()),
            TokenType::EOF => TokenType::EOF,
            TokenType::Plus => TokenType::Plus,
            TokenType::Minus => TokenType::Minus,
            TokenType::Bang => TokenType::Bang,
            TokenType::Asterisk => TokenType::Asterisk,
            TokenType::Slash => TokenType::Slash,
            TokenType::LT => TokenType::LT,
            TokenType::GT => TokenType::GT,
            TokenType::True => TokenType::True,
            TokenType::False => TokenType::False,
            TokenType::If => TokenType::If,
            TokenType::Else => TokenType::Else,
            TokenType::Return => TokenType::Return,
            TokenType::Eq => TokenType::Eq,
            TokenType::NotEq => TokenType::NotEq,
            TokenType::Modulo => TokenType::Modulo,
        };

        Token { token_type }
    }
}

impl TokenType {
    pub fn identifier(ident: impl Into<String>) -> TokenType {
        TokenType::Identifier(ident.into())
    }

    pub fn integer(integer: impl Into<String>) -> TokenType {
        TokenType::Integer(integer.into())
    }
}
