pub mod expression;
pub mod statement;

#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::statement::Statement;

use crate::ast::expression::boolean::Boolean;
use crate::object::environment::Environment;

use crate::object::Object;
use crate::token::Token;
use derive_more::From;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, From)]
pub enum Node {
    Program(Program),
    Expression(Expression),
    Statement(Statement), // expression statement, return statement, let statement
    Object(Object),
}

impl From<&Expression> for Node {
    fn from(value: &Expression) -> Self {
        Self::Expression(value.clone())
    }
}

impl From<&Statement> for Node {
    fn from(value: &Statement) -> Self {
        Self::Statement(value.clone())
    }
}

impl From<&Object> for Node {
    fn from(value: &Object) -> Self {
        Self::Object(value.clone())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Expression(value) => write!(f, "{value}"),
            Node::Statement(value) => write!(f, "{value}"),
            Node::Object(value) => write!(f, "{value}"),
            Node::Program(value) => write!(f, "{value}"),
        }
    }
}

pub trait NodeInterface: Debug + Display {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> &str;
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
#[derive(Debug, Default, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)] // add debug trait for debug
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{statement}")?;
        }

        Ok(())
    }
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            ""
        } else {
            self.statements
                .first()
                .expect("never failed")
                .token_literal()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    #[tracing::instrument(level = "trace", name = "eval_program", skip(self, env))]
    pub fn eval_program(&self, env: &mut Environment) -> anyhow::Result<Object> {
        let null = crate::object::null::Null;
        let mut result: Object = null.into();

        for statement in self.statements.clone().into_iter() {
            let statement_node: Node = statement.into();
            result = statement_node.eval(env)?;

            match result {
                Object::ReturnValue(value) => {
                    tracing::error!("[eval_statement] ReturnValue is ({value:?})");
                    return Ok(value.value().clone());
                }
                _ => continue,
            }
        }

        Ok(result)
    }
}

impl NodeInterface for Program {
    fn token_literal(&self) -> &str {
        self.token_literal()
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier {
    pub token: Token, // token.IDENT 词法单元
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}

impl From<Token> for Identifier {
    fn from(token: Token) -> Self {
        Self {
            value: token.literal().into(),
            token,
        }
    }
}

impl From<Boolean> for Identifier {
    fn from(value: Boolean) -> Self {
        Self {
            value: value.value().to_string(),
            token: value.token().clone(),
        }
    }
}

impl NodeInterface for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl TryFrom<Expression> for Identifier {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Identifier(value) => Ok(value),
            Expression::IntegerLiteral(value) => Ok(Identifier {
                value: value.value().to_string(),
                token: value.token().clone(),
            }),
            Expression::Boolean(value) => Ok(Identifier {
                token: value.token().clone(),
                value: value.value().to_string(),
            }),
            v => {
                tracing::error!("Expression: {v}");
                Err(anyhow::anyhow!("Expression({}) is not Identifier", v))
            }
        }
    }
}

impl TryFrom<&Expression> for Identifier {
    type Error = anyhow::Error;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        Identifier::try_from(value.clone())
    }
}
