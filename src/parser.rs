use crate::{ast::program::Program, lexer::Lexer, token::Token};

#[derive(Debug)]
pub struct ParserError {
    msg: String,
}

impl ParserError {
    fn new(msg: impl Into<String>) -> Self {
        ParserError { msg: msg.into() }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peeking_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peeking_token = lexer.next_token();

        let parser = Parser {
            lexer,
            current_token,
            peeking_token,
            errors: vec![],
        };

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::new();

        Ok(program)
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peeking_token);
        self.peeking_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    use super::Parser;

    #[test]
    fn test_new_with_empty_input() {
        let parser = make_parser("");

        assert_eq!(parser.current_token, Token::EOF);
        assert_eq!(parser.peeking_token, Token::EOF);
    }

    #[test]
    fn test_new_with_single_token_input() {
        let parser = make_parser(";");

        assert_eq!(parser.current_token, Token::Semicolon);
        assert_eq!(parser.peeking_token, Token::EOF);
    }

    #[test]
    fn test_new_with_multiple_tokens_input() {
        let parser = make_parser("let five = 5;");

        assert_eq!(parser.current_token, Token::Let);
        assert_eq!(
            parser.peeking_token,
            Token::Identifier(String::from("five"))
        );
    }

    #[test]
    fn test_next_token() {
        let mut parser = make_parser("let five = 5;");

        assert_eq!(parser.current_token, Token::Let);
        assert_eq!(parser.peeking_token, Token::identifier("five"));

        parser.next_token();
        assert_eq!(parser.current_token, Token::identifier("five"));
        assert_eq!(parser.peeking_token, Token::Assign);

        parser.next_token();
        assert_eq!(parser.current_token, Token::Assign);
        assert_eq!(parser.peeking_token, Token::Integer(5));

        parser.next_token();
        assert_eq!(parser.current_token, Token::Integer(5));
        assert_eq!(parser.peeking_token, Token::Semicolon);
    }

    fn make_parser(input: impl Into<String>) -> Parser {
        let input = input.into();
        let lexer = Lexer::new(&input);
        let parser = Parser::new(lexer);
        return parser;
    }
}
