#[derive(Debug, PartialEq, Default)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Self {
            line: self.line,
            column: self.column,
        }
    }
}
