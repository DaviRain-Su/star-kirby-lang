use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::{Identifier, NodeInterface};
use crate::error::Error;
use crate::object::environment::Environment;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::Join;

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
        let mut params = vec![];
        for p in self.parameters.iter() {
            params.push(p.to_string());
        }
        write!(f, "fn")?;
        write!(f, "(")?;
        write!(f, "{}", ", ".join(params))?;
        writeln!(f, ") {{")?;
        write!(f, "{}", self.body)?;
        write!(f, "\n}}")
    }
}

impl ObjectInterface for Function {
    fn r#type(&self) -> ObjectType {
        ObjectType::FunctionObj
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NodeInterface for Function {
    fn token_literal(&self) -> String {
        format!("{self}")
    }

    fn as_any(&self) -> &dyn Any {
        self
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
