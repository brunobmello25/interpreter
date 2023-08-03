use repl::Repl;

mod lexer;
mod parser;
mod repl;

fn main() {
    println!("Monkey repl! enter empty string to exit");

    Repl::start(std::io::stdin());
}
