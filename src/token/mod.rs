pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token {
    // identifier for token type
    r#type: TokenType,
    // identifier for token value
    literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, ch: char) -> Self {
        Self::from_char(r#type, ch)
    }

    pub fn from_char(r#type: TokenType, ch: char) -> Self {
        Self {
            r#type,
            literal: ch.into(),
        }
    }

    pub fn from_string(r#type: TokenType, literal: String) -> Self {
        Self { r#type, literal }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.r#type
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }

    pub fn token_type_mut(&mut self) -> &mut TokenType {
        &mut self.r#type
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
