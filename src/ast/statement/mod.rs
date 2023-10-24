pub mod block;
pub mod expression;
pub mod let_statement;
pub mod return_statement;

use crate::ast::statement::block::BlockStatement;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::NodeInterface;
use std::fmt::{Debug, Display, Formatter};

use super::Node;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
    Return(ReturnStatement),
    BlockStatement(BlockStatement),
}

impl NodeInterface for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Self::Expression(value) => value.token_literal(),
            Self::Let(value) => value.token_literal(),
            Self::Return(value) => value.token_literal(),
            Self::BlockStatement(value) => value.token_literal(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(value) => write!(f, "{value}"),
            Self::Let(value) => write!(f, "{value}"),
            Self::Return(value) => write!(f, "{value}"),
            Self::BlockStatement(value) => write!(f, "{value}"),
        }
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(value: ExpressionStatement) -> Self {
        Self::Expression(value)
    }
}

impl From<ExpressionStatement> for Node {
    fn from(value: ExpressionStatement) -> Self {
        Self::Statement(Statement::Expression(value))
    }
}

impl From<LetStatement> for Statement {
    fn from(value: LetStatement) -> Self {
        Self::Let(value)
    }
}

impl From<LetStatement> for Node {
    fn from(value: LetStatement) -> Self {
        Self::Statement(Statement::Let(value))
    }
}

impl From<ReturnStatement> for Statement {
    fn from(value: ReturnStatement) -> Self {
        Self::Return(value)
    }
}

impl From<ReturnStatement> for Node {
    fn from(value: ReturnStatement) -> Self {
        Self::Statement(Statement::Return(value))
    }
}

impl From<BlockStatement> for Statement {
    fn from(value: BlockStatement) -> Self {
        Self::BlockStatement(value)
    }
}

impl From<BlockStatement> for Node {
    fn from(value: BlockStatement) -> Self {
        Self::Statement(Statement::BlockStatement(value))
    }
}

impl AsRef<Statement> for &Statement {
    fn as_ref(&self) -> &Statement {
        self
    }
}
