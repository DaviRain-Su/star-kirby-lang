pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token {
    // identifier for token type
    token_type: TokenType,
    // identifier for token value
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        Self::from_char(token_type, ch)
    }

    pub fn from_char(token_type: TokenType, ch: char) -> Self {
        Self {
            token_type,
            literal: ch.into(),
        }
    }

    pub fn from_string(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }

    pub fn token_type_mut(&mut self) -> &mut TokenType {
        &mut self.token_type
    }

    pub fn literal_mut(&mut self) -> &mut String {
        &mut self.literal
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::from_string(TokenType::EOF, "\0".into())
    }
}
