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
            Self::Expression(exp_s) => exp_s.token_literal(),
            Self::Let(let_s) => let_s.token_literal(),
            Self::Return(ret_s) => ret_s.token_literal(),
            Self::BlockStatement(block) => block.token_literal(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(exp_s) => write!(f, "{exp_s}"),
            Self::Let(let_s) => write!(f, "{let_s}"),
            Self::Return(ret_s) => write!(f, "{ret_s}"),
            Self::BlockStatement(block_s) => write!(f, "{block_s}"),
        }
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(exp_s: ExpressionStatement) -> Self {
        Self::Expression(exp_s)
    }
}

impl From<ExpressionStatement> for Node {
    fn from(expression_statement: ExpressionStatement) -> Self {
        Self::Statement(Statement::Expression(expression_statement))
    }
}

impl From<LetStatement> for Statement {
    fn from(let_s: LetStatement) -> Self {
        Self::Let(let_s)
    }
}

impl From<LetStatement> for Node {
    fn from(let_statement: LetStatement) -> Self {
        Self::Statement(Statement::Let(let_statement))
    }
}

impl From<ReturnStatement> for Statement {
    fn from(ret_s: ReturnStatement) -> Self {
        Self::Return(ret_s)
    }
}

impl From<ReturnStatement> for Node {
    fn from(return_statement: ReturnStatement) -> Self {
        Self::Statement(Statement::Return(return_statement))
    }
}

impl From<BlockStatement> for Statement {
    fn from(block_s: BlockStatement) -> Self {
        Self::BlockStatement(block_s)
    }
}

impl From<BlockStatement> for Node {
    fn from(block_s: BlockStatement) -> Self {
        Self::Statement(Statement::BlockStatement(block_s))
    }
}

impl AsRef<Statement> for &Statement {
    fn as_ref(&self) -> &Statement {
        self
    }
}
