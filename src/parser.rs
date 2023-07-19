use crate::{
    ast::{LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug)]
pub struct ParserError {
    msg: String,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peeking_token: Token,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
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

        while self.current_token != Token::EOF {
            let statement = self.parse_statement();

            if let Some(statement) = statement {
                program.statements.push(statement);
            }

            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let identifier = match self.peeking_token {
            Token::Identifier(ref identifier) => identifier.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Identifier(identifier.clone())) {
            return None;
        }

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        // TODO: parse expression here
        while self.peeking_token != Token::Semicolon && self.peeking_token != Token::EOF {
            self.next_token();
        }

        Some(Statement::Let(LetStatement { identifier }))
    }

    fn next_token(&mut self) {
        // set current_token to peeking_token and advance peeking_token to lexer.next_token
        let next = self.lexer.next_token();
        self.current_token = std::mem::replace(&mut self.peeking_token, next);
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peeking_token == token {
            self.next_token();
            return true;
        } else {
            self.peek_error(&token);
            return false;
        }
    }

    fn peek_error(&mut self, expected_token: &Token) {
        let msg = format!(
            "expected next token to be {}, got {}",
            expected_token.token_literal(),
            self.peeking_token.token_literal()
        );
        self.errors.push(ParserError { msg });
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{
        ast::{LetStatement, Statement},
        lexer::Lexer,
        token::Token,
    };

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

    #[test]
    fn test_parse_let_statements() {
        let input = String::from(indoc! {"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "});
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("failed to parse program");

        let expected_statements = vec![
            LetStatement {
                identifier: "x".to_string(),
            },
            LetStatement {
                identifier: "y".to_string(),
            },
            LetStatement {
                identifier: "foobar".to_string(),
            },
        ];

        assert_eq!(parser.errors.len(), 0, "parser has errors");

        assert_eq!(program.statements.len(), expected_statements.len());
        for (statement, expected_statement) in program.statements.iter().zip(&expected_statements) {
            if let Statement::Let(s) = statement {
                assert_eq!(s, expected_statement);
            } else {
                panic!("expected let statement");
            }
        }
    }

    #[test]
    fn test_peek_error() {
        let input = String::from(indoc! {"
            let x 5;
            let y = 10;
            let foobar = 838383;
        "});
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        parser.parse_program().expect("failed to parse program");

        assert_eq!(parser.errors.len(), 1, "parser should have one error");
        assert_eq!(
            parser.errors[0].msg,
            "expected next token to be assign, got integer"
        )
    }
}
