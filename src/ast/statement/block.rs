use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::ast::NodeInterface;
use crate::object::environment::Environment;
use crate::object::null::Null;
use crate::object::Object;
use crate::object::ObjectInterface;
use crate::object::ObjectType;
use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockStatement {
    token: Token, // '{' 词法单元
    statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn statements_len(&self) -> usize {
        self.statements.len()
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    #[tracing::instrument(name = "eval_block_statement", skip(self),fields(env = %env))]
    pub fn eval_block_statement(&self, env: &mut Environment) -> anyhow::Result<Object> {
        let mut result: Object = Null.into();

        for statement in self.statements.clone().into_iter() {
            tracing::trace!("[eval_block_statement] statement is ({:#?})", statement);
            let node: Node = statement.into();
            result = node.eval(env)?;

            tracing::trace!("[eval_block_statement] result is ({:?})", result);
            match result.clone() {
                Object::ReturnValue(value) => {
                    if value.object_type() == ObjectType::Return {
                        return Ok(value.into());
                    }
                }
                _ => continue,
            }
        }

        Ok(result)
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

impl NodeInterface for BlockStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
