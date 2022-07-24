pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token {
    // identifier for token type
    pub(crate) r#type: TokenType,
    // identifier for token value
    pub(crate) literal: String,
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
}

impl Default for Token {
    fn default() -> Self {
        Token::from_string(TokenType::EOF, "\0".into())
    }
}

#[test]
fn test_token_struct() {
    let temp_struct = Token {
        r#type: TokenType::LET,
        literal: String::from("let"),
    };

    println!("token = {:?}", temp_struct);
}
