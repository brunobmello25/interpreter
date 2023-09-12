use crate::config::Config;

mod cli;
mod config;
mod evaluator;
mod lexer;
mod parser;
mod repl;

fn main() {
    let config = Config::new(&mut std::env::args());

    println!("{:?}", config);

    // println!("Monkey repl! enter empty string to exit");

    // let repl = Repl::new(std::io::stdin());
    // repl.start();
}
