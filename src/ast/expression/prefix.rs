use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prefix {
    token: Token, // 前缀词法单元，如!
    operator: String,
    right: Box<Expression>,
}

impl Prefix {
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

impl Default for Prefix {
    fn default() -> Self {
        Self {
            token: Token::default(),
            operator: String::default(),
            right: Box::new(IntegerLiteral::default().into()),
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl NodeInterface for Prefix {
    fn token_literal(&self) -> &str {
        self.right.token_literal()
    }
}

impl TryFrom<ExpressionStatement> for Prefix {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        match value.expression() {
            Expression::Prefix(value) => Ok(value.clone()),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
