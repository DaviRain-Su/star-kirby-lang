use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::block::BlockStatement;
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

    pub fn update_alternative(&mut self, alternative: BlockStatement) {
        self.alternative = Some(alternative);
    }

    pub fn update_consequence(&mut self, consequence: BlockStatement) {
        self.consequence = Some(consequence);
    }

    pub fn update_expression(&mut self, expression: Expression) {
        self.condition = Box::new(expression);
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
    fn token_literal(&self) -> &str {
        self.token.literal()
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

impl TryFrom<&Expression> for If {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        If::try_from(value.clone())
    }
}
