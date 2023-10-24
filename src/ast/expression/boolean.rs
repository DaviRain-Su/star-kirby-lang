use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use log::trace;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Boolean {
    token: Token,
    value: bool,
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal())
    }
}

impl NodeInterface for Boolean {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<Expression> for Boolean {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Boolean(value) => Ok(value),
            Expression::Prefix(value) => match value.right() {
                Expression::Boolean(value) => Ok(value.clone()),
                value => Err(Error::UnknownExpression(value.to_string()).into()),
            },
            Expression::Identifier(value) => Ok(Boolean {
                token: value.token.clone(),
                value: value.value.parse()?,
            }),
            unknow => {
                trace!("[try_from] Expression is ({unknow})");
                Err(Error::UnknownExpression(unknow.to_string()).into())
            }
        }
    }
}
