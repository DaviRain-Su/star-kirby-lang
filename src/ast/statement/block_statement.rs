use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockStatement {
    pub token: Token, // '{' 词法单元
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
