use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ArrayLiteral {
    token: Token, // '[' token word
    elements: Vec<Expression>,
}

impl ArrayLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            elements: Default::default(),
        }
    }

    pub fn elements(&self) -> &[Expression] {
        &self.elements
    }

    pub fn update_elements(&mut self, elements: Vec<Expression>) {
        self.elements = elements;
    }
}

impl NodeInterface for ArrayLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{elements}]")
    }
}

impl TryFrom<Expression> for ArrayLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::ArrayLiteral(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for ArrayLiteral {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        ArrayLiteral::try_from(value.clone())
    }
}
