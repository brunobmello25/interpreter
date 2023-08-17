use std::fmt::Display;

use crate::{
    expect_peek,
    lexer::{
        lexer::Lexer,
        location::Location,
        token::{Token, TokenType},
    },
};

use super::{
    ast::{
        expression::Expression,
        operator::{InfixOperator, PrefixOperator},
        program::Program,
        statement::Statement,
    },
    precedence::Precedence,
};

#[derive(Debug)]
pub struct ParserError {
    msg: String,
    location: Location,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ParserError:{}] {} ", self.location, self.msg)
    }
}

impl ParserError {
    fn new(msg: impl Into<String>, location: &Location) -> ParserError {
        ParserError {
            msg: msg.into(),
            location: location.clone(),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peeking_token: Token,
    pub errors: Vec<ParserError>,
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();

            match stmt {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => self.errors.push(err),
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peeking_token.token_type == TokenType::Semicolon {
            self.next_token();
        };

        Ok(Statement::expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_prefix()?;

        while self.peeking_token.token_type != TokenType::Semicolon
            && precedence < Precedence::from(&self.peeking_token)
        {
            self.next_token();

            lhs = self.parse_infix(lhs)?;
        }

        Ok(lhs)
    }

    fn advance_tokens(&mut self) {
        while self.current_token.token_type != TokenType::Semicolon
            && self.current_token.token_type != TokenType::EOF
        {
            self.next_token();
        }

        if self.current_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
    }

    fn parse_prefix(&mut self) -> Result<Expression, ParserError> {
        match &self.current_token.token_type {
            TokenType::Identifier(identifier) => Ok(Expression::identifier(identifier)),
            TokenType::Integer(integer_literal) => self.parse_integer(integer_literal),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::True | TokenType::False => self.parse_boolean(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_literal(),
            TokenType::Null => Ok(Expression::Null),
            token_type => Err(ParserError::new(
                format!("Expected prefix expression, got {:?}", token_type),
                &self.current_token.location,
            )),
        }
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_call_arguments()?;
        Ok(Expression::call(function, arguments))
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut arguments = vec![];

        if self.peeking_token.token_type == TokenType::RParen {
            self.next_token();
            return Ok(arguments);
        }

        self.next_token();

        arguments.push(self.parse_expression(Precedence::LOWEST)?);

        while self.peeking_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(Precedence::LOWEST)?);
        }

        expect_peek!(self, RParen)?;

        Ok(arguments)
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParserError> {
        expect_peek!(self, LParen)?;

        let parameters = self.parse_function_params()?;

        expect_peek!(self, LBrace)?;

        let body = self.parse_block_statement()?;

        Ok(Expression::function(parameters, body))
    }

    fn parse_function_params(&mut self) -> Result<Vec<String>, ParserError> {
        let mut params = vec![];

        if self.peeking_token.token_type == TokenType::RParen {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        while let TokenType::Identifier(identifier) = &self.current_token.token_type {
            params.push(identifier.clone());

            self.next_token();
            if let TokenType::Comma = self.current_token.token_type {
                self.next_token();
            }
        }

        Ok(params)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        expect_peek!(self, LParen)?;

        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST)?;

        expect_peek!(self, RParen)?;

        expect_peek!(self, LBrace)?;

        let consequence = self.parse_block_statement()?;

        let mut alternative: Option<Vec<Statement>> = None;

        if self.peeking_token.token_type == TokenType::Else {
            self.next_token();

            expect_peek!(self, LBrace)?;

            alternative = Some(self.parse_block_statement()?);
        }

        Ok(Expression::r#if(condition, consequence, alternative))
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Statement>, ParserError> {
        self.next_token();

        let mut statements = vec![];

        while self.current_token.token_type != TokenType::RBrace
            && self.current_token.token_type != TokenType::EOF
        {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }

        Ok(statements)
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST);

        expect_peek!(self, RParen)?;

        expression
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = match &self.current_token.token_type {
            TokenType::Bang => PrefixOperator::Not,
            TokenType::Minus => PrefixOperator::Negative,
            token_type => {
                return Err(ParserError::new(
                    format!("unexpected token {}", token_type),
                    &self.current_token.location,
                ))
            }
        };

        self.next_token();

        let expression = self.parse_expression(Precedence::PREFIX)?;
        Ok(Expression::prefix(expression, operator))
    }

    fn parse_infix(&mut self, lhs: Expression) -> Result<Expression, ParserError> {
        let precedence = Precedence::from(&self.current_token);

        let operator = match &self.current_token.token_type {
            TokenType::Eq => InfixOperator::Equal,
            TokenType::NotEq => InfixOperator::NotEqual,
            TokenType::Plus => InfixOperator::Add,
            TokenType::Minus => InfixOperator::Sub,
            TokenType::Asterisk => InfixOperator::Mult,
            TokenType::Slash => InfixOperator::Div,
            TokenType::GT => InfixOperator::GreaterThan,
            TokenType::LT => InfixOperator::LessThan,
            TokenType::Modulo => InfixOperator::Modulo,
            TokenType::LParen => return self.parse_call_expression(lhs),
            token_type => {
                return Err(ParserError::new(
                    format!("unexpected token {}", token_type),
                    &self.current_token.location,
                ))
            }
        };

        self.next_token();

        let rhs = self.parse_expression(precedence)?;
        Ok(Expression::infix(lhs, rhs, operator))
    }

    fn parse_boolean(&self) -> Result<Expression, ParserError> {
        match &self.current_token.token_type {
            TokenType::True => Ok(Expression::Bool(true)),
            TokenType::False => Ok(Expression::Bool(false)),
            _ => Err(ParserError::new(
                format!("expected boolean, got {}", self.current_token.token_type),
                &self.current_token.location,
            )),
        }
    }

    fn parse_integer(&self, literal: &String) -> Result<Expression, ParserError> {
        literal.parse().map(Expression::Int).map_err(|_| {
            ParserError::new(
                format!("failed to parse integer {}", literal),
                &self.current_token.location,
            )
        })
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let identifier = match &self.current_token.token_type {
            TokenType::Identifier(identifier) => identifier.clone(),
            _ => {
                return Err(ParserError::new(
                    format!("expected identifier, got {}", self.current_token.token_type),
                    &self.current_token.location.clone(),
                ))
            }
        };

        expect_peek!(self, Assign)?;

        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peeking_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::r#let(identifier, expression))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peeking_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::r#return(expression))
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
        lexer::{lexer::Lexer, token::TokenType},
        parser::ast::{
            expression::Expression,
            operator::{InfixOperator, PrefixOperator},
            statement::Statement,
        },
    };

    use super::Parser;

    #[test]
    fn test_if_with_multiple_statements() {
        let mut parser = make_parser(indoc! {"
            if (x < y) {
                let z = x + 20;
                return x + y;
            }
        "});
        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::r#if(
                Expression::infix(
                    Expression::identifier("x"),
                    Expression::identifier("y"),
                    InfixOperator::LessThan
                ),
                vec![
                    Statement::r#let(
                        "z",
                        Expression::infix(
                            Expression::identifier("x"),
                            Expression::Int(20),
                            InfixOperator::Add
                        )
                    ),
                    Statement::r#return(Expression::infix(
                        Expression::identifier("x"),
                        Expression::identifier("y"),
                        InfixOperator::Add
                    ))
                ],
                None
            ))
        )
    }

    #[test]
    fn test_multiple_statements() {
        let mut parser = make_parser(indoc! {"
            let z = x + 20;
            return x + y;
        "});
        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements[0],
            Statement::r#let(
                "z",
                Expression::infix(
                    Expression::identifier("x"),
                    Expression::Int(20),
                    InfixOperator::Add
                )
            )
        );
        assert_eq!(
            program.statements[1],
            Statement::r#return(Expression::infix(
                Expression::identifier("x"),
                Expression::identifier("y"),
                InfixOperator::Add
            ))
        );
    }

    #[test]
    fn test_complex_expressions_with_conditions() {
        let mut parser = make_parser(indoc! {"
            if (x < y) {
                let z = x + 20;
                return x + y;
            } else {
                if (x > y) {
                    let z = x - y;
                    return x - y;
                } else {
                    return x * y;
                }
            }
        "});
        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
        let first_condition = Expression::infix(
            Expression::identifier("x"),
            Expression::identifier("y"),
            InfixOperator::LessThan,
        );
        let first_let = Statement::r#let(
            "z",
            Expression::infix(
                Expression::identifier("x"),
                Expression::Int(20),
                InfixOperator::Add,
            ),
        );
        let first_return = Statement::r#return(Expression::infix(
            Expression::identifier("x"),
            Expression::identifier("y"),
            InfixOperator::Add,
        ));
        let second_condition = Expression::infix(
            Expression::identifier("x"),
            Expression::identifier("y"),
            InfixOperator::GreaterThan,
        );
        let second_let = Statement::r#let(
            "z",
            Expression::infix(
                Expression::identifier("x"),
                Expression::identifier("y"),
                InfixOperator::Sub,
            ),
        );
        let second_return = Statement::r#return(Expression::infix(
            Expression::identifier("x"),
            Expression::identifier("y"),
            InfixOperator::Sub,
        ));
        let third_return = Statement::r#return(Expression::infix(
            Expression::identifier("x"),
            Expression::identifier("y"),
            InfixOperator::Mult,
        ));
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::r#if(
                first_condition,
                vec![first_let, first_return],
                Some(vec![Statement::Expression(Expression::r#if(
                    second_condition,
                    vec![second_let, second_return],
                    Some(vec![third_return])
                ))])
            ))
        );
    }

    #[test]
    fn test_fn_with_if_else() {
        let mut parser = make_parser(indoc! {"
            let counter = fn(x) {
                if (x > 100) {
                    return true;
                } else {
                    let foobar = 9999;
                    foobar
                }
            };
        "});
        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::r#let(
                "counter",
                Expression::function(
                    vec!["x"],
                    vec![Statement::Expression(Expression::r#if(
                        Expression::infix(
                            Expression::identifier("x"),
                            Expression::Int(100),
                            InfixOperator::GreaterThan
                        ),
                        vec![Statement::r#return(Expression::Bool(true))],
                        Some(vec![
                            Statement::r#let("foobar", Expression::Int(9999)),
                            Statement::Expression(Expression::identifier("foobar"))
                        ])
                    ))]
                )
            )
        )
    }

    #[test]
    fn test_nested_if_parsing() {
        let mut parser = make_parser(indoc! {"
            if(x < y) {
                if(x > y) {
                    return x;
                } else {
                    return y;
                }
            } else {
                return y;
            }
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::r#if(
                Expression::infix(
                    Expression::identifier("x"),
                    Expression::identifier("y"),
                    InfixOperator::LessThan
                ),
                vec![Statement::Expression(Expression::r#if(
                    Expression::infix(
                        Expression::identifier("x"),
                        Expression::identifier("y"),
                        InfixOperator::GreaterThan
                    ),
                    vec![Statement::r#return(Expression::identifier("x"))],
                    Some(vec![Statement::r#return(Expression::identifier("y"))])
                ))],
                Some(vec![Statement::r#return(Expression::identifier("y")),])
            ))
        );
    }

    #[test]
    fn test_call_expression_parsing() {
        let mut parser = make_parser("add(1, 2 * 3, 4 + 5, 6 / 2);");
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::call(
                Expression::identifier("add"),
                vec![
                    Expression::Int(1),
                    Expression::infix(Expression::Int(2), Expression::Int(3), InfixOperator::Mult),
                    Expression::infix(Expression::Int(4), Expression::Int(5), InfixOperator::Add),
                    Expression::infix(Expression::Int(6), Expression::Int(2), InfixOperator::Div)
                ]
            ))
        )
    }

    #[test]
    fn test_function_literal_parsing() {
        let mut parser = make_parser(indoc! {"
            fn(x, y) {
                x + y;
            }
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::function(
                vec!["x", "y"],
                vec![Statement::expression(Expression::infix(
                    Expression::identifier("x"),
                    Expression::identifier("y"),
                    InfixOperator::Add,
                ))]
            ))
        );
    }

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
            5 % 5;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 9);

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
        infix_assert!(8, InfixOperator::Modulo);
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

        assert_eq!(parser.current_token.token_type, TokenType::EOF);
        assert_eq!(parser.peeking_token.token_type, TokenType::EOF);
    }

    #[test]
    fn test_new_with_single_token_input() {
        let parser = make_parser(";");

        assert_eq!(parser.current_token.token_type, TokenType::Semicolon);
        assert_eq!(parser.peeking_token.token_type, TokenType::EOF);
    }

    #[test]
    fn test_new_with_multiple_tokens_input() {
        let parser = make_parser("let five = 5;");

        assert_eq!(parser.current_token.token_type, TokenType::Let);
        assert_eq!(
            parser.peeking_token.token_type,
            TokenType::Identifier(String::from("five"))
        );
    }

    #[test]
    fn test_next_token() {
        let mut parser = make_parser("let five = 5;");

        assert_eq!(parser.current_token.token_type, TokenType::Let);
        assert_eq!(
            parser.peeking_token.token_type,
            TokenType::identifier("five")
        );

        parser.next_token();
        assert_eq!(
            parser.current_token.token_type,
            TokenType::identifier("five")
        );
        assert_eq!(parser.peeking_token.token_type, TokenType::Assign);

        parser.next_token();
        assert_eq!(parser.current_token.token_type, TokenType::Assign);
        assert_eq!(parser.peeking_token.token_type, TokenType::integer("5"));

        parser.next_token();
        assert_eq!(parser.current_token.token_type, TokenType::integer("5"));
        assert_eq!(parser.peeking_token.token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_if_expression() {
        let mut parser = make_parser(indoc! {"
            if (x < y) { x }
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::r#if(
                Expression::infix(
                    Expression::identifier("x"),
                    Expression::identifier("y"),
                    InfixOperator::LessThan,
                ),
                vec![Statement::Expression(Expression::identifier("x"))],
                None
            ))
        )
    }

    #[test]
    fn test_if_else_expression() {
        let mut parser = make_parser(indoc! {"
            if (x < y) { x } else { y }
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::r#if(
                Expression::infix(
                    Expression::identifier("x"),
                    Expression::identifier("y"),
                    InfixOperator::LessThan,
                ),
                vec![Statement::Expression(Expression::identifier("x"))],
                Some(vec![Statement::Expression(Expression::identifier("y"))])
            ))
        )
    }

    #[test]
    fn test_parse_let_statement() {
        let mut parser = make_parser(indoc! {"
            let x = 5;
            let y = 10;
            let banana = 123456;
        "});

        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);

        assert_eq!(
            program.statements[0],
            Statement::r#let("x", Expression::Int(5))
        );
        assert_eq!(
            program.statements[1],
            Statement::r#let("y", Expression::Int(10))
        );
        assert_eq!(
            program.statements[2],
            Statement::r#let("banana", Expression::Int(123456))
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

        assert_eq!(
            program.statements[0],
            Statement::r#return(Expression::identifier("banana"))
        );
        assert_eq!(
            program.statements[1],
            Statement::r#return(Expression::infix(
                Expression::Int(69),
                Expression::Int(420),
                InfixOperator::Add
            ))
        );
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
    fn test_parse_null() {
        let mut parser = make_parser(indoc! {"
            null;
            let x = null;
            x == null;
        "});
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
        assert_eq!(
            program.statements[0],
            Statement::expression(Expression::Null)
        );
        assert_eq!(
            program.statements[1],
            Statement::r#let("x", Expression::Null)
        );
        assert_eq!(
            program.statements[2],
            Statement::expression(Expression::infix(
                Expression::identifier("x"),
                Expression::Null,
                InfixOperator::Equal
            ))
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

    #[test]
    fn test_precedences() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for test in tests {
            let mut parser = make_parser(test.0);
            let program = parser.parse_program();
            assert_eq!(parser.errors.len(), 0);
            assert_eq!(program.to_string().trim(), test.1);
        }
    }

    fn make_parser<'a>(input: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        return parser;
    }
}
