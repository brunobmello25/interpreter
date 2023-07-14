use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let mut lexer = Lexer::new("let a".to_string());

    let token = lexer.next_token();
    println!("{:?}", token);
}
