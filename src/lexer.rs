use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a String,
    position: usize,
    reading_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            reading_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semicolon,
            '\0' => Token::EOF,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '<' => Token::LT,
            '>' => Token::GT,
            '0'..='9' => {
                return Token::Integer(self.read_number());
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let word = self.read_word();

                let token = match word.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Identifier(word),
                };

                return token;
            }
            ch => Token::Illegal(ch),
        };

        self.read_char();
        return token;
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

    fn read_number(&mut self) -> usize {
        let initial_pos = self.position;

        while self.is_number(self.ch) {
            self.read_char();
        }

        let result = self.input[initial_pos..self.position]
            .to_string()
            .parse()
            .expect("failed to parse integer");

        return result;
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

    use super::*;

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
        "});

        let expected_tokens = vec![
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::LParen,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::LT,
            Token::Integer(10),
            Token::GT,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer(5),
            Token::LT,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer(10),
            Token::Eq,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEq,
            Token::Integer(9),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(&input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }
}
