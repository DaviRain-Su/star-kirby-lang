use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::display::Join;
use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Box<Expression>>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut elements = vec![];

        for el in self.elements.iter() {
            elements.push(format!("{}", *el));
        }

        write!(f, "[")?;
        write!(f, "{}", ",".join(elements))?;
        write!(f, "]")
    }
}

impl TryFrom<Expression> for ArrayLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::ArrayLiteral(value) => Ok(value),
            _ => unimplemented!(),
        }
    }
}
