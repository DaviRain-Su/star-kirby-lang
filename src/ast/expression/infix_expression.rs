use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InfixExpression {
    token: Token,
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

impl InfixExpression {
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

    pub fn right_mut(&mut self) -> &mut Box<Expression> {
        &mut self.right
    }
}

impl Default for InfixExpression {
    fn default() -> Self {
        Self {
            token: Token::default(),
            left: Box::new(IntegerLiteral::default().into()),
            operator: String::default(),
            right: Box::new(IntegerLiteral::default().into()),
        }
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl NodeInterface for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<ExpressionStatement> for InfixExpression {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        match value.expression {
            Expression::InfixExpression(infix_exp) => Ok(infix_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<Expression> for InfixExpression {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::InfixExpression(infix_exp) => Ok(infix_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for InfixExpression {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::InfixExpression(infix_exp) => Ok(infix_exp.clone()),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
