use std::io::{self, Stdin, Write};

use crate::{lexer::Lexer, token::Token};

pub struct Repl {}

const PROMPT: &'static str = ">> ";

impl Repl {
    pub fn start(stdin: Stdin) {
        println!("Monkey repl! enter empty string to exit");

        let mut line = String::new();

        Self::read_input(&mut line, &stdin);

        while !line.trim().is_empty() {
            let mut lexer = Lexer::new(&line);

            let mut token = lexer.next_token();
            while token != Token::EOF {
                println!("{:?}", token);
                token = lexer.next_token();
            }

            Self::read_input(&mut line, &stdin);
        }
    }

    fn read_input(input: &mut String, stdin: &Stdin) {
        input.clear();
        print!("{PROMPT}");
        io::stdout().flush().expect("failed to flush stdout");
        stdin.read_line(input).expect("failed to read line");
    }
}
