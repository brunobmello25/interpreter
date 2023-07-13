// config test

#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Identifier(String),
    Assign,
    Integer(usize),
    Plus,
    Comma,
    Function,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Illegal,
    EOF,
}

struct Lexer {
    input: String,
    position: usize,
    reading_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
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
            _ => todo!(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let input = String::from("=+(){},;");
        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }
}
