use std::io::{self, Stdin, Write};

use crate::{
    lexer::lexer::Lexer,
    parser::{ast::program::Program, parser::Parser},
};

pub struct Repl {}

const PROMPT: &'static str = ">> ";

impl Repl {
    pub fn start(stdin: Stdin) {
        let mut line = String::new();
        Self::read_input(&mut line, &stdin);

        while !line.trim().is_empty() {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();

            if parser.errors.len() == 0 {
                Self::print_program(&program);
            } else {
                println!("Woops! parser got {} errors!", parser.errors.len());
                for error in parser.errors {
                    println!("{}", error);
                }
            }

            Self::read_input(&mut line, &stdin);
        }
    }

    fn print_program(program: &Program) {
        for stmt in &program.statements {
            println!("{:?}", stmt);
        }
        println!("");
    }

    fn read_input(input: &mut String, stdin: &Stdin) {
        input.clear();
        print!("{PROMPT}");
        io::stdout().flush().expect("failed to flush stdout");
        stdin.read_line(input).expect("failed to read line");
    }
}
