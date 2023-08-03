use repl::Repl;

mod evaluator;
mod lexer;
mod parser;
mod repl;

fn main() {
    println!("Monkey repl! enter empty string to exit");

    let repl = Repl::new(std::io::stdin());
    repl.start();
}
