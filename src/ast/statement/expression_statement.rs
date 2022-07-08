use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// expression statement
/// ExpressionStatement 类型具有两个字段，分别是每个节点都具有的token字段
/// 和保存表达的expression字段。
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token, // 该表达式中的第一个词法单元
    pub expression: Expression,
}

impl Default for ExpressionStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            expression: Expression::IntegerLiteralExpression(IntegerLiteral::default()),
        }
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expression)
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression.token_literal()
    }
}

impl From<Statement> for ExpressionStatement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::ExpressionStatement(exp_s) => exp_s,
            _ => unimplemented!(),
        }
    }
}

impl From<&Statement> for ExpressionStatement {
    fn from(value: &Statement) -> Self {
        match value {
            Statement::ExpressionStatement(exp_s) => exp_s.clone(),
            _ => unimplemented!(),
        }
    }
}
