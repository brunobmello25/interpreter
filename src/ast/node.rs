use std::fmt::Display;

pub trait Node: Display {
    fn token_literal() -> String;
}
