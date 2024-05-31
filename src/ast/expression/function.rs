use crate::ast::expression::Expression;
use crate::ast::statement::block::BlockStatement;
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

    pub fn update_body(&mut self, body: BlockStatement) {
        self.body = body;
    }

    pub fn parameters(&self) -> &[Identifier] {
        &self.parameters
    }

    pub fn update_parameters(&mut self, parameters: Vec<Identifier>) {
        self.parameters = parameters;
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
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<Expression> for FunctionLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::FunctionLiteral(value) => Ok(value),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}

impl TryFrom<&Expression> for FunctionLiteral {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        FunctionLiteral::try_from(value.clone())
    }
}
