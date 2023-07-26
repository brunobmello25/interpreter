#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Assign,
    Integer(usize),
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
}

impl Token {
    pub fn identifier(ident: impl Into<String>) -> Token {
        Token::Identifier(ident.into())
    }

    pub fn token_literal(&self) -> &str {
        match self {
            Token::Let => "let",
            Token::Identifier(_) => "identifier",
            Token::Assign => "assign",
            Token::Integer(_) => "integer",
            Token::Comma => "comma",
            Token::Function => "function",
            Token::LParen => "lparen",
            Token::RParen => "rparen",
            Token::LBrace => "lbrace",
            Token::RBrace => "rbrace",
            Token::Semicolon => "semicolon",
            Token::Illegal(_) => "illegal",
            Token::EOF => "eof",
            Token::Plus => "plus",
            Token::Minus => "minus",
            Token::Bang => "bang",
            Token::Asterisk => "asterisk",
            Token::Slash => "slash",
            Token::LT => "lt",
            Token::GT => "gt",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::Eq => "eq",
            Token::NotEq => "noteq",
        }
    }
}
