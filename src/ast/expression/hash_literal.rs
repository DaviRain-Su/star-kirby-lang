use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use string_join::display::Join;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashLiteral {
    pub token: Token, // token '{'
    pub pair: BTreeMap<Expression, Expression>,
}

impl Display for HashLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pair = vec![];
        for (key, value) in self.pair.iter() {
            pair.push(format!("{}:{}", key, value));
        }

        write!(f, "{{{}}}", ",".join(pair))
    }
}

impl NodeInterface for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for HashLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::HashLiteral(value) => Ok(value),
            _ => unimplemented!(),
        }
    }
}
