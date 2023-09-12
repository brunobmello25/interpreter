use std::env::Args;

#[derive(Debug)]
enum Mode {
    Repl,
    File { path: String },
}

#[derive(Debug)]
pub struct Config {
    mode: Mode,
}

impl Config {
    pub fn new(args: &mut Args) -> Self {
        let mode = match args.len() {
            0 | 1 => Mode::Repl,
            _ => {
                args.next();
                Mode::File {
                    path: args.next().unwrap(),
                }
            }
        };

        Config { mode }
    }
}
