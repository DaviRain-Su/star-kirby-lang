use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashLiteral {
    token: Token, // token '{'
    pair: BTreeMap<Expression, Expression>,
}

impl HashLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            pair: Default::default(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn pair(&self) -> &BTreeMap<Expression, Expression> {
        &self.pair
    }

    pub fn pair_mut(&mut self) -> &mut BTreeMap<Expression, Expression> {
        &mut self.pair
    }
}

impl Display for HashLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self
            .pair
            .iter()
            .map(|(key, value)| format!("{key}:{value}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{{{pairs}}}")
    }
}

impl NodeInterface for HashLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<Expression> for HashLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::HashLiteral(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for HashLiteral {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        HashLiteral::try_from(value.clone())
    }
}
