use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Call {
    token: Token,              // '('词法单元
    function: Box<Expression>, // 标识符或函数字面量
    arguments: Vec<Expression>,
}

impl Call {
    pub fn new(token: Token, function: Expression) -> Self {
        Self {
            token,
            function: Box::new(function),
            arguments: Vec::new(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<Expression> {
        &mut self.arguments
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let args = self
            .arguments
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({})", self.function, args)
    }
}

impl NodeInterface for Call {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for Call {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Call(call_exp) => Ok(call_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
