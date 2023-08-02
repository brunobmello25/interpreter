#[macro_export]
macro_rules! expect_peek {
    ($self: ident, $token: ident) => {
        match $self.peeking_token.token_type {
            TokenType::$token => {
                $self.next_token();
                Ok(())
            }
            _ => Err(ParserError::new(format!(
                "unexpected token {} in {}",
                $self.peeking_token.token_type, $self.peeking_token.location,
            ))),
        }
    };
}
