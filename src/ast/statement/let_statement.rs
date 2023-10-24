use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// let statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LetStatement {
    token: Token, // token.LET 词法单元
    name: Identifier,
    value: Box<Expression>,
}

impl LetStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn construct(token: Token, name: Identifier, value: Expression) -> Self {
        Self {
            token,
            name,
            value: Box::new(value),
        }
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Expression {
        &mut self.value
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut Identifier {
        &mut self.name
    }
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            name: Identifier::default(),
            value: Box::new(Expression::IntegerLiteral(IntegerLiteral::default())),
        }
    }
}

impl NodeInterface for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
        )
    }
}

impl TryFrom<Statement> for LetStatement {
    type Error = anyhow::Error;

    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Let(value) => Ok(value),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Statement> for LetStatement {
    type Error = anyhow::Error;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Let(value) => Ok(value.clone()),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}
