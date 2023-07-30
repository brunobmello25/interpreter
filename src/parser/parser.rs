use crate::{
    ast::{
        expression::Expression,
        operator::{InfixOperator, PrefixOperator},
        program::Program,
        statement::Statement,
    },
    lexer::Lexer,
    parser::precedence::Precedence,
    token::Token,
};

pub struct ParserError {}

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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token != Token::EOF {
            let stmt = match self.current_token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            };

            match stmt {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => self.errors.push(err),
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peeking_token == Token::Semicolon {
            self.next_token();
        };

        Ok(Statement::expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_prefix()?;

        while self.peeking_token != Token::Semicolon
            && precedence < Precedence::from(&self.peeking_token)
        {
            self.next_token();

            lhs = self.parse_infix(lhs)?;
        }

        Ok(lhs)
    }

    fn advance_tokens(&mut self) {
        while self.current_token != Token::Semicolon && self.current_token != Token::EOF {
            self.next_token();
        }

        if self.current_token == Token::Semicolon {
            self.next_token();
        }
    }

    fn parse_prefix(&mut self) -> Result<Expression, ParserError> {
        match &self.current_token {
            Token::Identifier(identifier) => Ok(Expression::identifier(identifier)),
            Token::Integer(integer_literal) => self.parse_integer(integer_literal),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            _ => Err(ParserError {}),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = match &self.current_token {
            Token::Bang => PrefixOperator::Not,
            Token::Minus => PrefixOperator::Negative,
            _ => return Err(ParserError {}),
        };

        self.next_token();

        self.parse_expression(Precedence::PREFIX)
            .map(|expression| Expression::prefix(expression, operator))
            .map_err(|_| ParserError {})
    }

    fn parse_infix(&mut self, lhs: Expression) -> Result<Expression, ParserError> {
        let precedence = Precedence::from(&self.current_token);
        let operator = InfixOperator::from(&self.current_token);

        self.next_token();

        let rhs = self.parse_expression(precedence);

        match rhs {
            Ok(rhs) => Ok(Expression::infix(lhs, rhs, operator)),
            Err(_) => Err(ParserError {}),
        }
    }

    fn parse_integer(&self, literal: &String) -> Result<Expression, ParserError> {
        literal
            .parse()
            .map(Expression::Int)
            .map_err(|_| ParserError {})
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let let_token = self.current_token.clone();

        self.next_token();

        let identifier = match &self.current_token {
            Token::Identifier(identifier) => identifier.clone(),
            _ => return Err(ParserError {}),
        };

        self.next_token();

        // TODO: parse expression here
        while self.current_token != Token::Semicolon && self.current_token != Token::EOF {
            self.next_token();
        }

        Ok(Statement::Let { name: identifier })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let return_token = self.current_token.clone();

        self.next_token();

        // TODO: parse expressions here
        while self.current_token != Token::Semicolon && self.current_token != Token::EOF {
            self.next_token();
        }

        Ok(Statement::r#return(return_token))
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peeking_token);
        self.peeking_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{
        ast::{
            expression::Expression,
            operator::{InfixOperator, PrefixOperator},
            statement::Statement,
        },
        lexer::Lexer,
        token::Token,
    };

    use super::Parser;

    #[test]
    fn test_parsing_infix_expressions_with_integers() {
        let mut parser = make_parser(indoc! {"
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 8);

        macro_rules! infix_assert {
            ($index:expr, $op:expr) => {
                assert_eq!(
                    program.statements[$index],
                    Statement::expression(Expression::infix(
                        Expression::Int(5),
                        Expression::Int(5),
                        $op,
                    ))
                )
            };
        }
        infix_assert!(0, InfixOperator::Add);
        infix_assert!(1, InfixOperator::Sub);
        infix_assert!(2, InfixOperator::Mult);
        infix_assert!(3, InfixOperator::Div);
        infix_assert!(4, InfixOperator::GreaterThan);
        infix_assert!(5, InfixOperator::LessThan);
        infix_assert!(6, InfixOperator::Equal);
        infix_assert!(7, InfixOperator::NotEqual);
    }

    #[test]
    fn test_parsing_infix_with_multiple_expressions() {
        let mut parser = make_parser(indoc! {"
            5 + 7 * 10;
            1 - 2 + 3;
            5 * 7 + 10;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::infix(
                Expression::Int(5),
                Expression::infix(Expression::Int(7), Expression::Int(10), InfixOperator::Mult),
                InfixOperator::Add
            ))
        );
        assert_eq!(
            program.statements[1],
            Statement::Expression(Expression::infix(
                Expression::infix(Expression::Int(1), Expression::Int(2), InfixOperator::Sub),
                Expression::Int(3),
                InfixOperator::Add
            ))
        );
        assert_eq!(
            program.statements[2],
            Statement::Expression(Expression::infix(
                Expression::infix(Expression::Int(5), Expression::Int(7), InfixOperator::Mult),
                Expression::Int(10),
                InfixOperator::Add
            ))
        );
    }

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
        assert_eq!(parser.peeking_token, Token::integer("5"));

        parser.next_token();
        assert_eq!(parser.current_token, Token::integer("5"));
        assert_eq!(parser.peeking_token, Token::Semicolon);
    }

    #[test]
    fn test_parse_single_let_statement() {
        let mut parser = make_parser("let x = banana;");

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);
        assert_eq!(parser.errors.len(), 0);

        assert_eq!(program.statements[0], Statement::r#let(Token::Let, "x"))
    }

    #[test]
    fn test_parse_let_statement() {
        let mut parser = make_parser(indoc! {"
            let x = 5;
            let y = 10;
            let banana = 123456;
        "});

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);
        assert_eq!(parser.errors.len(), 0);

        //TODO: assert that the statement expressions are correct
        assert_eq!(program.statements[0], Statement::r#let(Token::Let, "x"));
        assert_eq!(program.statements[1], Statement::r#let(Token::Let, "y"));
        assert_eq!(
            program.statements[2],
            Statement::r#let(Token::Let, "banana")
        );
    }

    #[test]
    fn test_parse_return_statement() {
        let mut parser = make_parser(indoc! {"
            return banana;
            return 69 + 420;
        "});

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(parser.errors.len(), 0);

        // TODO: assert that the statement expressions are correct
        assert_eq!(program.statements[0], Statement::r#return(Token::Return));
        assert_eq!(program.statements[1], Statement::r#return(Token::Return));
    }

    #[test]
    fn test_identifier_expression() {
        let mut parser = make_parser(indoc! {"
            banana;
            apple;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements[0],
            Statement::expression(Expression::identifier("banana"))
        );
        assert_eq!(
            program.statements[1],
            Statement::expression(Expression::identifier("apple"))
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let mut parser = make_parser(indoc! {"
            123;
            456;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements[0],
            Statement::expression(Expression::Int(123))
        );
        assert_eq!(
            program.statements[1],
            Statement::expression(Expression::Int(456))
        );
    }

    #[test]
    fn test_prefix_operators() {
        let mut parser = make_parser(indoc! {"
            !5;
            -15;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements[0],
            Statement::expression(Expression::prefix(Expression::Int(5), PrefixOperator::Not))
        );
        assert_eq!(
            program.statements[1],
            Statement::expression(Expression::prefix(
                Expression::Int(15),
                PrefixOperator::Negative
            ))
        );
    }

    fn make_parser(input: impl Into<String>) -> Parser {
        let input = input.into();
        let lexer = Lexer::new(&input);
        let parser = Parser::new(lexer);
        return parser;
    }
}
