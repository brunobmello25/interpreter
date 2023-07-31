#[macro_export]
macro_rules! expect_peek {
    ($self: ident, $token: expr) => {
        match $self.peeking_token {
            $token => {
                $self.next_token();
                Ok(())
            }
            _ => Err(ParserError {}),
        }?;
    };
}
