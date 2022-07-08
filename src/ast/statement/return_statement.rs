use crate::ast::statement::Statement;
use crate::ast::{Identifier, Node};
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// return statement
#[derive(Debug, Default, Clone)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: Identifier,
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
}

impl From<Statement> for ReturnStatement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::ReturnStatement(return_value) => return_value.clone(),
            _ => unimplemented!(),
        }
    }
}
