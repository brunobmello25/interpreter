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
        }
    }
}

impl Token {
    pub fn identifier(ident: impl Into<String>) -> Token {
        Token::Identifier(ident.into())
    }

    pub fn token_literal(&self) -> String {
        let result = match self {
            Token::Let => "let".to_string(),
            Token::Identifier(identifier) => identifier.to_string(),
            Token::Assign => "=".to_string(),
            Token::Integer(integer) => integer.to_string(),
            Token::Comma => ",".to_string(),
            Token::Function => "fn".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Illegal(token) => token.to_string(),
            Token::EOF => "\0".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Return => "return".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
        };

        return result.into();
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn test_token_literal() {
        assert_eq!(Token::Let.token_literal(), "let");
        assert_eq!(Token::Identifier("a".to_string()).token_literal(), "a");
        assert_eq!(Token::Assign.token_literal(), "=");
        assert_eq!(Token::Integer(1).token_literal(), "1");
        assert_eq!(Token::Comma.token_literal(), ",");
        assert_eq!(Token::Function.token_literal(), "fn");
        assert_eq!(Token::LParen.token_literal(), "(");
        assert_eq!(Token::RParen.token_literal(), ")");
        assert_eq!(Token::LBrace.token_literal(), "{");
        assert_eq!(Token::RBrace.token_literal(), "}");
        assert_eq!(Token::Semicolon.token_literal(), ";");
        assert_eq!(Token::Illegal('a').token_literal(), "a");
        assert_eq!(Token::EOF.token_literal(), "\0");
        assert_eq!(Token::Plus.token_literal(), "+");
        assert_eq!(Token::Minus.token_literal(), "-");
        assert_eq!(Token::Bang.token_literal(), "!");
        assert_eq!(Token::Asterisk.token_literal(), "*");
        assert_eq!(Token::Slash.token_literal(), "/");
        assert_eq!(Token::LT.token_literal(), "<");
        assert_eq!(Token::GT.token_literal(), ">");
        assert_eq!(Token::True.token_literal(), "true");
        assert_eq!(Token::False.token_literal(), "false");
        assert_eq!(Token::If.token_literal(), "if");
        assert_eq!(Token::Else.token_literal(), "else");
        assert_eq!(Token::Return.token_literal(), "return");
        assert_eq!(Token::Eq.token_literal(), "==");
        assert_eq!(Token::NotEq.token_literal(), "!=");
    }
}
