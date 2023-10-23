use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::object::environment::Environment;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Function {
    parameters: Vec<Identifier>,
    body: BlockStatement,
    env: Environment,
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement, env: Environment) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
    pub fn parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }

    pub fn env(&self) -> &Environment {
        &self.env
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn(")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{p}")?;
        }
        writeln!(f, ") {{")?;
        writeln!(f, "{}", self.body)?;
        write!(f, "}}")
    }
}

impl ObjectInterface for Function {
    fn object_type(&self) -> ObjectType {
        ObjectType::FunctionObj
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }
}

impl NodeInterface for Function {
    fn token_literal(&self) -> String {
        format!("{self}")
    }
}

impl TryFrom<Object> for Function {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Function(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
