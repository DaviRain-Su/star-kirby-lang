use crate::ast::expression::Expression;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntegerLiteral {
    token: Token,
    value: isize,
}

impl IntegerLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            value: Default::default(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> isize {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut isize {
        &mut self.value
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal())
    }
}

impl NodeInterface for IntegerLiteral {
    fn token_literal(&self) -> &str {
        "integer_literal"
    }
}

impl TryFrom<ExpressionStatement> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(expression_statement: ExpressionStatement) -> Result<Self, Self::Error> {
        let identifier = Identifier::try_from(expression_statement.expression())?;
        let value = identifier.value.parse::<isize>()?;

        Ok(Self {
            token: expression_statement.token().clone(),
            value,
        })
    }
}

impl TryFrom<Expression> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::IntegerLiteral(value) => Ok(value),
            Expression::Prefix(value) => match value.right() {
                Expression::IntegerLiteral(value) => Ok(value.clone()),
                unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
            },
            Expression::Identifier(value) => Ok(IntegerLiteral {
                token: value.token.clone(),
                value: value.value.parse()?,
            }),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
