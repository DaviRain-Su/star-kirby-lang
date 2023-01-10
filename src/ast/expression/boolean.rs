use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use log::trace;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

impl NodeInterface for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for Boolean {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::BooleanExpression(boolean) => Ok(boolean),
            Expression::PrefixExpression(prefix_expression) => match *prefix_expression.right {
                Expression::BooleanExpression(value) => Ok(value),
                value => Err(Error::UnknownExpression(value.to_string()).into()),
            },
            Expression::IdentifierExpression(ident) => Ok(Boolean {
                token: ident.token.clone(),
                value: ident.value.parse()?,
            }),
            unknow => {
                trace!("[try_from] Expression is ({})", unknow);
                Err(Error::UnknownExpression(unknow.to_string()).into())
            }
        }
    }
}
