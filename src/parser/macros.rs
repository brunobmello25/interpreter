#[macro_export]
macro_rules! expect_peek {
    ($self: ident, $token: ident) => {
        match $self.peeking_token {
            Token::$token => {
                $self.next_token();
                Ok(())
            }
            _ => Err(ParserError {}),
        }
    };
}
