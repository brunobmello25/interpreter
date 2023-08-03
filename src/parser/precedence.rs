use crate::lexer::token::{Token, TokenType};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST = 1,
    EQUALS = 2,
    LESSGREATER = 3,
    SUM = 4,
    PRODUCT = 5,
    PREFIX = 6,
    CALL = 7,
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Self {
        match token.token_type {
            TokenType::Eq => Precedence::EQUALS,
            TokenType::NotEq => Precedence::EQUALS,
            TokenType::Plus => Precedence::SUM,
            TokenType::Minus => Precedence::SUM,
            TokenType::Slash => Precedence::PRODUCT,
            TokenType::Asterisk => Precedence::PRODUCT,
            TokenType::GT => Precedence::LESSGREATER,
            TokenType::LT => Precedence::LESSGREATER,
            TokenType::LParen => Precedence::CALL,
            TokenType::Modulo => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::precedence::Precedence;

    #[test]
    fn test_precedence() {
        assert!(Precedence::LOWEST < Precedence::EQUALS);
        assert!(Precedence::EQUALS < Precedence::LESSGREATER);
        assert!(Precedence::LESSGREATER < Precedence::SUM);
        assert!(Precedence::SUM < Precedence::PRODUCT);
        assert!(Precedence::PRODUCT < Precedence::PREFIX);
        assert!(Precedence::PREFIX < Precedence::CALL);
    }
}
