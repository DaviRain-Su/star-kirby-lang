use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// return statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReturnStatement {
    token: Token, //  return 词法单元
    return_value: Box<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn return_value(&self) -> &Expression {
        &self.return_value
    }

    pub fn return_value_mut(&mut self) -> &mut Box<Expression> {
        &mut self.return_value
    }

    pub fn return_value_into(self) -> Expression {
        *self.return_value
    }
}

impl Default for ReturnStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            return_value: Box::new(Expression::IntegerLiteral(IntegerLiteral::default())),
        }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

impl NodeInterface for ReturnStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<Statement> for ReturnStatement {
    type Error = anyhow::Error;

    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Return(value) => Ok(value),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Statement> for ReturnStatement {
    type Error = anyhow::Error;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Return(value) => Ok(value.clone()),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}
