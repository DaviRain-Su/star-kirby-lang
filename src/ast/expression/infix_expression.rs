use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::Node;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
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

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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
            _ => unimplemented!(),
        }
    }
}

impl TryFrom<Expression> for InfixExpression {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::InfixExpression(infix_exp) => Ok(infix_exp),
            _ => unimplemented!(),
        }
    }
}
