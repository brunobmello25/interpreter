use repl::Repl;

mod lexer;
mod repl;
mod token;

fn main() {
    // let input = "let a".to_string();
    // let mut lexer = Lexer::new(&input);
    //
    // let token = lexer.next_token();
    // println!("{:?}", token);

    Repl::start(std::io::stdin());
}
