use std::io::{self, Stdin, Write};

use crate::{
    evaluator::{
        evaluator::{EvaluationError, Evaluator},
        object::Object,
    },
    lexer::lexer::Lexer,
    parser::{
        ast::{node::Node, program::Program},
        parser::Parser,
    },
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
                match Self::evaluate_program(program) {
                    Ok(object) => println!("{}", object),
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("Woops! parser got {} errors!", parser.errors.len());
                for error in parser.errors {
                    println!("{}", error);
                }
            }

            Self::read_input(&mut line, &stdin);
        }
    }

    fn evaluate_program(program: Program) -> Result<Object, EvaluationError> {
        let evaluator = Evaluator::new();

        evaluator.eval(Node::Program(program))
    }

    fn read_input(input: &mut String, stdin: &Stdin) {
        input.clear();
        print!("{PROMPT}");
        io::stdout().flush().expect("failed to flush stdout");
        stdin.read_line(input).expect("failed to read line");
    }
}
