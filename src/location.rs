use std::fmt::Display;

#[derive(Debug, PartialEq, Default)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}:{}", self.line, self.column)
    }
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Self {
            line: self.line,
            column: self.column,
        }
    }
}
