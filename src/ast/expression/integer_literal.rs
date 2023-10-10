use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
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
            value: isize::default(),
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
    fn token_literal(&self) -> String {
        format!("{}", self.value)
    }
}

impl TryFrom<ExpressionStatement> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(expression_statement: ExpressionStatement) -> Result<Self, Self::Error> {
        let identifier = Identifier::try_from(expression_statement.expression)?;
        let value = identifier.value.parse::<isize>()?;

        Ok(Self {
            token: expression_statement.token,
            value,
        })
    }
}

impl TryFrom<Expression> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::IntegerLiteral(integ_exp) => Ok(integ_exp),
            Expression::Prefix(pref_exp) => match pref_exp.right() {
                Expression::IntegerLiteral(value) => Ok(value.clone()),
                unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
            },
            Expression::Identifier(ident) => Ok(IntegerLiteral {
                token: ident.token.clone(),
                value: ident.value.parse()?,
            }),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
