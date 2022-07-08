use std::any::Any;
use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::token::Token;
use std::fmt::{Display, Formatter};
use string_join::display::Join;

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,              // '('词法单元
    pub function: Box<Expression>, // 标识符或函数字面量
    pub arguments: Vec<Box<Expression>>,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut args = vec![];
        for a in self.arguments.iter() {
            args.push(format!("{}", a));
        }

        let args = ",".join(args);
        write!(f, "{}", self.function)?;
        write!(f, "(")?;
        write!(f, "{}", args)?;
        write!(f, ")")
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for CallExpression {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::CallExpression(call_exp) => Ok(call_exp),
            _ => unimplemented!(),
        }
    }
}
