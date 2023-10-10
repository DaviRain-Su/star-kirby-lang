use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct If {
    token: Token,
    condition: Box<Expression>,
    consequence: Option<BlockStatement>,
    alternative: Option<BlockStatement>,
}

impl If {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn alternative(&self) -> &Option<BlockStatement> {
        &self.alternative
    }

    pub fn consequence(&self) -> &Option<BlockStatement> {
        &self.consequence
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn alternative_mut(&mut self) -> &mut Option<BlockStatement> {
        &mut self.alternative
    }

    pub fn consequence_mut(&mut self) -> &mut Option<BlockStatement> {
        &mut self.consequence
    }

    pub fn condition_mut(&mut self) -> &mut Box<Expression> {
        &mut self.condition
    }
}

impl Default for If {
    fn default() -> Self {
        Self {
            token: Token::default(),
            condition: Box::new(Expression::IntegerLiteral(IntegerLiteral::default())),
            consequence: None,
            alternative: None,
        }
    }
}

impl Display for If {
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

impl NodeInterface for If {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }
}

impl TryFrom<Expression> for If {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::If(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
