#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Assign,
    Integer(usize),
    Plus,
    Comma,
    Function,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Illegal(char),
    EOF,
}

