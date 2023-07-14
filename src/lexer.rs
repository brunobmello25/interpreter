use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    reading_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
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
            '=' => Token::Assign,
            '+' => Token::Plus,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semicolon,
            '\0' => Token::EOF,
            '0'..='9' => {
                return Token::Integer(self.read_number());
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier();

                let token = match identifier.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    _ => Token::Identifier(identifier),
                };

                return token;
            }
            ch => Token::Illegal(ch),
        };

        self.read_char();
        return token;
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

    fn read_identifier(&mut self) -> String {
        let initial_pos = self.position;

        while self.is_letter(self.ch) {
            self.read_char();
        }

        let result = self.input[initial_pos..self.position].to_string();

        println!(
            "Finished reading identifier. read '{}'. next char will be '{}'",
            result, self.ch
        );

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

        println!(
            "Finished reading identifier. read '{}'. next char will be '{}'",
            result, self.ch
        );

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
    fn test_next_token_word() {
        let input = String::from("let");
        let expected_tokens = vec![Token::Let];

        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_next_token_with_space() {
        let input = String::from("let    let");
        let expected_tokens = vec![Token::Let, Token::Let];

        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_read_number() {
        let input = String::from("5 10 15");
        let expected_tokens = vec![Token::Integer(5), Token::Integer(10), Token::Integer(15)];

        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
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
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }
}
