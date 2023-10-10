use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

/// let statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LetStatement {
    pub token: Token, // token.LET 词法单元
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl LetStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            name: Identifier::default(),
            value: Box::new(Expression::IntegerLiteralExpression(
                IntegerLiteral::default(),
            )),
        }
    }
}

impl NodeInterface for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
            Statement::Let(let_s) => Ok(let_s),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Statement> for LetStatement {
    type Error = anyhow::Error;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Let(let_s) => Ok(let_s.clone()),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}
