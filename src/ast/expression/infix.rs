use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Infix {
    token: Token,
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

impl Infix {
    pub fn new(token: Token, left: Expression, operator: String) -> Self {
        Self {
            token,
            left: Box::new(left),
            operator,
            ..Default::default()
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> &str {
        self.operator.as_str()
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }

    pub fn update_expression(&mut self, right: Expression) {
        self.right = Box::new(right);
    }
}

impl Default for Infix {
    fn default() -> Self {
        Self {
            token: Token::default(),
            left: Box::new(IntegerLiteral::default().into()),
            operator: String::default(),
            right: Box::new(IntegerLiteral::default().into()),
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl NodeInterface for Infix {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<ExpressionStatement> for Infix {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        match value.expression() {
            Expression::Infix(value) => Ok(value.clone()),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<Expression> for Infix {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Infix(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for Infix {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Infix(value) => Ok(value.clone()),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
