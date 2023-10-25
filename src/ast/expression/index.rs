use crate::ast::expression::Expression;
use crate::ast::Identifier;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index {
    token: Token, // '[' token word
    left: Box<Expression>,
    index: Box<Expression>,
}

impl Index {
    pub fn new(token: Token, left: Expression) -> Self {
        Self {
            token,
            left: Box::new(left),
            index: Box::new(Expression::Identifier(Identifier::default())),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn index(&self) -> &Expression {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut Box<Expression> {
        &mut self.index
    }
}

impl NodeInterface for Index {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

impl TryFrom<Expression> for Index {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Index(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for Index {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        Index::try_from(value.clone())
    }
}
