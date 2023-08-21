use std::{
    io::{self, Stdin, Write},
    rc::Rc,
};

use crate::{
    evaluator::{
        environment::Environment,
        evaluator::{EvaluationError, Evaluator},
        object::Object,
    },
    lexer::lexer::Lexer,
    parser::{
        ast::{node::Node, program::Program},
        parser::Parser,
    },
};

pub struct Repl {
    stdin: Stdin,
}

const PROMPT: &'static str = ">> ";

impl Repl {
    pub fn new(stdin: Stdin) -> Self {
        Repl { stdin }
    }

    pub fn start(&self) {
        let mut line = String::new();
        self.read_input(&mut line, &self.stdin);

        while !line.trim().is_empty() {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            if parser.errors.len() == 0 {
                match self.evaluate_program(program) {
                    Ok(object) => println!("{}", object),
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("Woops! parser got {} errors!", parser.errors.len());
                for error in parser.errors {
                    println!("{}", error);
                }
            }

            self.read_input(&mut line, &self.stdin);
        }
    }

    fn evaluate_program(&self, program: Program) -> Result<Object, EvaluationError> {
        let environment = Environment::new();
        let mut evaluator = Evaluator::new();

        evaluator.eval(Node::Program(program), Rc::clone(&environment))
    }

    #[allow(dead_code)]
    fn print_program(&self, program: Program) {
        for statement in program.statements {
            println!("{:?}", statement);
        }
    }

    fn read_input(&self, input: &mut String, stdin: &Stdin) {
        input.clear();
        print!("{PROMPT}");
        io::stdout().flush().expect("failed to flush stdout");
        stdin.read_line(input).expect("failed to read line");
    }
}
