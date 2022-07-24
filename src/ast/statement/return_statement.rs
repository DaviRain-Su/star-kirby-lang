use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

/// return statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: Box<Expression>,
}

impl Default for ReturnStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            return_value: Box::new(Expression::IntegerLiteralExpression(
                IntegerLiteral::default(),
            )),
        }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Statement> for ReturnStatement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::ReturnStatement(return_value) => return_value,
            _ => unimplemented!(),
        }
    }
}
