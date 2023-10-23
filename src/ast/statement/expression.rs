use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// expression statement
/// ExpressionStatement 类型具有两个字段，分别是每个节点都具有的token字段
/// 和保存表达的expression字段。
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExpressionStatement {
    pub token: Token, // 该表达式中的第一个词法单元
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }
}

impl Default for ExpressionStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            expression: Expression::IntegerLiteral(IntegerLiteral::default()),
        }
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl NodeInterface for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression.token_literal()
    }
}

impl TryFrom<Statement> for ExpressionStatement {
    type Error = anyhow::Error;

    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Expression(exp_s) => Ok(exp_s),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Statement> for ExpressionStatement {
    type Error = anyhow::Error;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Expression(exp_s) => Ok(exp_s.clone()),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}
