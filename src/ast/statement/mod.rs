pub mod block_statement;
pub mod expression_statement;
pub mod let_statement;
pub mod return_statement;

use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::Node;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::ExpressionStatement(exp_s) => exp_s.token_literal(),
            Self::LetStatement(let_s) => let_s.token_literal(),
            Self::ReturnStatement(ret_s) => ret_s.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::ExpressionStatement(exp_s) => write!(f, "{}", exp_s),
            Statement::LetStatement(let_s) => write!(f, "{}", let_s),
            Statement::ReturnStatement(ret_s) => write!(f, "{}", ret_s),
        }
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(exp_s: ExpressionStatement) -> Self {
        Self::ExpressionStatement(exp_s)
    }
}

impl From<LetStatement> for Statement {
    fn from(let_s: LetStatement) -> Self {
        Self::LetStatement(let_s)
    }
}

impl From<ReturnStatement> for Statement {
    fn from(ret_s: ReturnStatement) -> Self {
        Self::ReturnStatement(ret_s)
    }
}

impl AsRef<Statement> for &Statement {
    fn as_ref(&self) -> &Statement {
        self
    }
}
