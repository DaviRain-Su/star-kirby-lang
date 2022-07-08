use std::any::Any;
use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

impl Node for Boolean {
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
                _ => unimplemented!(),
            },
            Expression::IdentifierExpression(ident) => Ok(Boolean {
                token: ident.token.clone(),
                value: ident.value.parse()?,
            }),
            _ => {
                println!("Expression: {:#?}", value);
                unimplemented!()
            }
        }
    }
}
