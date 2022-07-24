use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, Node};
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

/// let statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LetStatement {
    pub token: Token, // token.LET 词法单元
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            name: Identifier::default(),
            value: Box::new(Expression::IntegerLiteralExpression(
                IntegerLiteral::default(),
            )),
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
        )
    }
}

impl From<Statement> for LetStatement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::LetStatement(let_s) => let_s,
            _ => unimplemented!(),
        }
    }
}

impl From<&Statement> for LetStatement {
    fn from(value: &Statement) -> Self {
        match value {
            Statement::LetStatement(let_s) => let_s.clone(),
            _ => unimplemented!(),
        }
    }
}
