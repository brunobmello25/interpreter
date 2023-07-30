use crate::token::Token;

#[derive(PartialEq, PartialOrd)]
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
        match token {
            Token::Eq => Precedence::EQUALS,
            Token::NotEq => Precedence::EQUALS,
            Token::Plus => Precedence::SUM,
            Token::Minus => Precedence::SUM,
            Token::Slash => Precedence::PRODUCT,
            Token::Asterisk => Precedence::PRODUCT,
            Token::GT => Precedence::LESSGREATER,
            Token::LT => Precedence::LESSGREATER,
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
