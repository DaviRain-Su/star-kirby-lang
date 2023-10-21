use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionLiteral {
    token: Token, // 'fn' 词法单元
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            parameters: Default::default(),
            body: Default::default(),
        }
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut BlockStatement {
        &mut self.body
    }

    pub fn parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut Vec<Identifier> {
        &mut self.parameters
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "{}({parameters}){}", self.token_literal(), self.body)
    }
}

impl NodeInterface for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }
}

impl TryFrom<Expression> for FunctionLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::FunctionLiteral(fun_xp) => Ok(fun_xp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
