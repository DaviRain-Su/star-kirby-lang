use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl Default for IfExpression {
    fn default() -> Self {
        Self {
            token: Token::default(),
            condition: Box::new(Expression::IntegerLiteralExpression(
                IntegerLiteral::default(),
            )),
            consequence: None,
            alternative: None,
        }
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "if ")?;
        write!(f, "{}", self.condition)?;
        write!(f, " ")?;
        if self.consequence.is_some() {
            write!(f, "{}", self.consequence.as_ref().unwrap())?;
        }
        if self.alternative.is_some() {
            write!(f, "else ")?;
            write!(f, "{}", self.alternative.as_ref().unwrap())?;
        }
        Ok(())
    }
}

impl NodeInterface for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for IfExpression {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::IfExpression(value) => Ok(value),
            _ => unimplemented!(),
        }
    }
}
