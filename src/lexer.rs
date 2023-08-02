use crate::{
    location::Location,
    token::{Token, TokenType},
};

pub struct Lexer {
    input: String,
    position: usize,
    reading_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        let mut lexer = Lexer {
            input: input.into(),
            position: 0,
            reading_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token_type = match self.ch {
            ',' => TokenType::Comma,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            ';' => TokenType::Semicolon,
            '\0' => TokenType::EOF,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::NotEq
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '<' => TokenType::LT,
            '>' => TokenType::GT,
            '0'..='9' => {
                let token_type = TokenType::integer(self.read_number());
                return Token::new(token_type, self.location());
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let word = self.read_word();

                let token_type = match word.as_str() {
                    "let" => TokenType::Let,
                    "fn" => TokenType::Function,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "return" => TokenType::Return,
                    _ => TokenType::Identifier(word),
                };

                return Token::new(token_type, self.location());
            }
            '%' => TokenType::Modulo,
            ch => TokenType::Illegal(ch),
        };

        self.read_char();
        return Token::new(token_type, self.location());
    }

    fn location(&self) -> Location {
        Location::default()
    }

    fn peek_char(&self) -> char {
        match self.input.chars().nth(self.reading_position) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn read_char(&mut self) {
        if let Some(c) = self.input.chars().nth(self.reading_position) {
            self.ch = c;
        } else {
            self.ch = '\0';
        }

        self.position = self.reading_position;
        self.reading_position += 1;
    }

    fn read_word(&mut self) -> String {
        let initial_pos = self.position;

        while self.is_letter(self.ch) {
            self.read_char();
        }

        let result = self.input[initial_pos..self.position].to_string();

        return result;
    }

    fn read_number(&mut self) -> String {
        let initial_pos = self.position;

        while self.is_number(self.ch) {
            self.read_char();
        }

        self.input[initial_pos..self.position].to_string()
    }

    fn is_letter(&self, ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    fn is_number(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn skip_whitespace(&mut self) {
        while [' ', '\t', '\n', '\r'].contains(&self.ch) {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::token::TokenType;

    use super::*;

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
