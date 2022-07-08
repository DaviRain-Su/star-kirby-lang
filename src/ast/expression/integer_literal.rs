use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::{Identifier, Node};
use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            value: i64::default(),
        }
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal.clone())
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        format!("{}", self.value)
    }
}

impl TryFrom<ExpressionStatement> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(expression_statement: ExpressionStatement) -> Result<Self, Self::Error> {
        let identifier = Identifier::try_from(expression_statement.expression)?;
        let value = identifier.value.parse::<i64>()?;

        Ok(Self {
            token: expression_statement.token.clone(),
            value,
        })
    }
}

impl TryFrom<Expression> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::IntegerLiteralExpression(integ_exp) => Ok(integ_exp.clone()),
            Expression::PrefixExpression(pref_exp) => match *pref_exp.right {
                Expression::IntegerLiteralExpression(value) => Ok(value.clone()),
                _ => unimplemented!(),
            },
            Expression::IdentifierExpression(ident) => Ok(IntegerLiteral {
                token: ident.token.clone(),
                value: ident.value.parse()?,
            }),
            _ => {
                println!("Expression: {:#?}", value);
                unimplemented!()
            }
        }
    }
}
