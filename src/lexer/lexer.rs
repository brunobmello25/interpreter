use std::iter::Peekable;
use std::str::Chars;

use super::location::Location;
use super::token::{Token, TokenType};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    ch: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars().peekable(),
            ch: None,
            line: 1,
            column: 0,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let location = Location::new(self.line, self.column);

        let token_type = match self.ch {
            Some(',') => TokenType::Comma,
            Some('(') => TokenType::LParen,
            Some(')') => TokenType::RParen,
            Some('{') => TokenType::LBrace,
            Some('}') => TokenType::RBrace,
            Some(';') => TokenType::Semicolon,
            Some('!') => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    TokenType::NotEq
                }
                _ => TokenType::Bang,
            },
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            Some('*') => TokenType::Asterisk,
            Some('/') => TokenType::Slash,
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some('<') => TokenType::LT,
            Some('>') => TokenType::GT,
            Some('0'..='9') => {
                let token_type = TokenType::integer(self.read_integer());
                return Token::new(token_type, location);
            }
            Some('a'..='z') | Some('A'..='Z') | Some('_') => {
                let word = self.read_word();

                let token_type = match word.as_str() {
                    "let" => TokenType::Let,
                    "fn" => TokenType::Function,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "return" => TokenType::Return,
                    _ => TokenType::identifier(word),
                };

                return Token::new(token_type, location);
            }
            Some('%') => TokenType::Modulo,
            Some(ch) => TokenType::Illegal(ch),
            None => TokenType::EOF,
        };

        self.read_char();
        return Token::new(token_type, location);
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn read_char(&mut self) {
        match self.chars.next() {
            Some(ch) => {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 0;
                } else {
                    self.column += 1;
                }

                self.ch = Some(ch);
            }
            None => {
                self.ch = None;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !([' ', '\t', '\n', '\r'].contains(&ch)) {
                break;
            }

            self.read_char();
        }
    }

    fn read_integer(&mut self) -> String {
        let mut number = String::new();

        while let Some(ch) = self.ch {
            if !Lexer::is_digit(ch) {
                break;
            }

            number.push(ch);
            self.read_char();
        }

        return number;
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();

        while let Some(ch) = self.ch {
            if !Lexer::is_letter(ch) {
                break;
            }

            word.push(ch);
            self.read_char();
        }

        return word;
    }

    fn is_letter(ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ('0'..='9').contains(&ch)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_read_equals_and_not_equals() {
        let mut lexer = Lexer::new("==\n!=");

        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::Eq, Location::new(1, 1))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::NotEq, Location::new(2, 1))
        );
    }

    #[test]
    fn test_read_word() {
        let mut lexer = Lexer::new("banana pera\nuva");

        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::identifier("banana"), Location::new(1, 1))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::identifier("pera"), Location::new(1, 8))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::identifier("uva"), Location::new(2, 1))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::EOF, Location::new(2, 3))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::EOF, Location::new(2, 3))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::EOF, Location::new(2, 3))
        );
    }

    #[test]
    fn test_read_integer() {
        let mut lexer = Lexer::new("1234\n6789");

        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::integer("1234"), Location::new(1, 1))
        );
        assert_eq!(
            lexer.next_token(),
            Token::new(TokenType::integer("6789"), Location::new(2, 1))
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("    ,\t\n\r.");

        assert_eq!(lexer.ch, Some(' '));
        assert_eq!(lexer.peek_char(), Some(&' '));
        lexer.skip_whitespace();
        assert_eq!(lexer.ch, Some(','));
        assert_eq!(lexer.peek_char(), Some(&'\t'));
        lexer.next_token();
        assert_eq!(lexer.ch, Some('\t'));
        assert_eq!(lexer.peek_char(), Some(&'\n'));
        lexer.skip_whitespace();
        assert_eq!(lexer.ch, Some('.'));
        assert_eq!(lexer.peek_char(), None);
    }

    #[test]
    fn test_peek_char_empty() {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.ch, None);
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.peek_char(), None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.peek_char(), None);
    }

    #[test]
    fn test_peek_char_single_char() {
        let mut lexer = Lexer::new(".");

        assert_eq!(lexer.ch, Some('.'));
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.peek_char(), None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.peek_char(), None);
    }

    #[test]
    fn test_peek_char() {
        let mut lexer = Lexer::new(".,*");

        assert_eq!(lexer.ch, Some('.'));
        assert_eq!(lexer.peek_char(), Some(&','));
        assert_eq!(lexer.peek_char(), Some(&','));
        lexer.next_token();
        assert_eq!(lexer.ch, Some(','));
        assert_eq!(lexer.peek_char(), Some(&'*'));
        assert_eq!(lexer.peek_char(), Some(&'*'));
        lexer.next_token();
        assert_eq!(lexer.ch, Some('*'));
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.peek_char(), None);
    }

    #[test]
    fn test_read_char_iterator_empty() {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
    }

    #[test]
    fn test_read_char_iterator() {
        let mut lexer = Lexer::new(",,,,");

        assert_eq!(lexer.ch, Some(','));
        lexer.next_token();
        assert_eq!(lexer.ch, Some(','));
        lexer.next_token();
        assert_eq!(lexer.ch, Some(','));
        lexer.next_token();
        assert_eq!(lexer.ch, Some(','));
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
        lexer.next_token();
        assert_eq!(lexer.ch, None);
    }

    #[test]
    fn test_next_token_empty() {
        let input = String::from("");

        let mut lexer = Lexer::new(&input);

        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_next_token() {
        let input = String::from(indoc! {"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            10 % 3;
        "});

        let expected_token_types = vec![
            TokenType::Let,
            TokenType::Identifier(String::from("five")),
            TokenType::Assign,
            TokenType::integer("5"),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("ten")),
            TokenType::Assign,
            TokenType::integer("10"),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("add")),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Identifier(String::from("x")),
            TokenType::Comma,
            TokenType::Identifier(String::from("y")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Identifier(String::from("x")),
            TokenType::Plus,
            TokenType::Identifier(String::from("y")),
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("result")),
            TokenType::Assign,
            TokenType::Identifier(String::from("add")),
            TokenType::LParen,
            TokenType::Identifier(String::from("five")),
            TokenType::Comma,
            TokenType::Identifier(String::from("ten")),
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::integer("5"),
            TokenType::Semicolon,
            TokenType::integer("5"),
            TokenType::LT,
            TokenType::integer("10"),
            TokenType::GT,
            TokenType::integer("5"),
            TokenType::Semicolon,
            TokenType::If,
            TokenType::LParen,
            TokenType::integer("5"),
            TokenType::LT,
            TokenType::integer("10"),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::integer("10"),
            TokenType::Eq,
            TokenType::integer("10"),
            TokenType::Semicolon,
            TokenType::integer("10"),
            TokenType::NotEq,
            TokenType::integer("9"),
            TokenType::Semicolon,
            TokenType::integer("10"),
            TokenType::Modulo,
            TokenType::integer("3"),
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let mut lexer = Lexer::new(&input);

        for expected_token in expected_token_types {
            let token = lexer.next_token();

            assert_eq!(token.token_type, expected_token);
        }
    }
}
