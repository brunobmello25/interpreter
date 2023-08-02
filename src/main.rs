use repl::Repl;

mod ast;
mod lexer;
mod location;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Monkey repl! enter empty string to exit");

    Repl::start(std::io::stdin());
}
