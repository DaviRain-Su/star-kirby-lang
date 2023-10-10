use crate::ast::expression::Expression;
use crate::ast::Identifier;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IndexExpression {
    token: Token, // '[' token word
    left: Box<Expression>,
    index: Box<Expression>,
}

impl IndexExpression {
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

impl NodeInterface for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

impl TryFrom<Expression> for IndexExpression {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Index(index_exp) => Ok(index_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
