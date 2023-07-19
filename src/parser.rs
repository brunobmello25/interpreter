use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peeking_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peeking_token = lexer.next_token();

        let parser = Parser {
            lexer,
            current_token,
            peeking_token,
        };

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        todo!()
    }

    fn next_token(&mut self) {
        // set current_token to peeking_token and advance peeking_token to lexer.next_token
        let next = self.lexer.next_token();
        self.current_token = std::mem::replace(&mut self.peeking_token, next);
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    use super::Parser;

    #[test]
    fn test_initial_tokens() {
        let input = String::from("let banana");
        let lexer = Lexer::new(&input);
        let parser = Parser::new(lexer);

        assert_eq!(parser.current_token, Token::Let);
        assert_eq!(
            parser.peeking_token,
            Token::Identifier("banana".to_string())
        );
    }

    #[test]
    fn test_next_token() {
        let input = String::from("let banana =");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        parser.next_token();

        assert_eq!(
            parser.current_token,
            Token::Identifier("banana".to_string())
        );
        assert_eq!(parser.peeking_token, Token::Assign);
    }

    #[test]
    fn test_next_token_with_eof() {
        let input = String::from("let");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        assert_eq!(parser.current_token, Token::Let);
        assert_eq!(parser.peeking_token, Token::EOF);

        parser.next_token();

        assert_eq!(parser.current_token, Token::EOF);
        assert_eq!(parser.peeking_token, Token::EOF);
    }
}
