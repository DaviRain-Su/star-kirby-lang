use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::ast::NodeInterface;
use crate::object::environment::Environment;
use crate::object::null::Null;
use crate::object::Object;
use crate::object::ObjectInterface;
use crate::object::ObjectType;
use crate::token::Token;
use log::trace;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockStatement {
    pub token: Token, // '{' 词法单元
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn eval_block_statement(&self, env: &mut Environment) -> anyhow::Result<Object> {
        trace!("[eval_block_statement]  BlockStatement is ({})", self);
        let mut result: Object = Null.into();

        for statement in self.statements.clone().into_iter() {
            trace!("[eval_block_statement] statement is ({:#?})", statement);
            let node: Node = statement.into();
            result = node.eval(env)?;

            trace!("[eval_block_statement] result is ({:?})", result);
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
