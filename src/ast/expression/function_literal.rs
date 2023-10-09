use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::Join;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionLiteral {
    pub token: Token, // 'fn' 词法单元
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            parameters: Default::default(),
            body: BlockStatement::default(),
        }
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(|value| format!("{value}"))
            .collect::<Vec<_>>();

        let parameters = ",".join(parameters);
        write!(f, "{}({}){}", self.token_literal(), parameters, self.body)
    }
}

impl NodeInterface for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
