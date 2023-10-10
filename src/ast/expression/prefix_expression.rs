use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PrefixExpression {
    token: Token, // 前缀词法单元，如!
    operator: String,
    right: Box<Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String) -> Self {
        Self {
            token,
            operator,
            ..Default::default()
        }
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

impl Default for PrefixExpression {
    fn default() -> Self {
        Self {
            token: Token::default(),
            operator: String::default(),
            right: Box::new(IntegerLiteral::default().into()),
        }
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl NodeInterface for PrefixExpression {
    fn token_literal(&self) -> String {
        self.right.token_literal()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<ExpressionStatement> for PrefixExpression {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        match value.expression {
            Expression::PrefixExpression(pre_exp) => Ok(pre_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
