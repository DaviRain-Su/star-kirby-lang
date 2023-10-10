use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ReturnValue {
    value: Box<Object>,
}

impl ReturnValue {
    pub fn new(value: Object) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    pub fn value(&self) -> &Object {
        &self.value
    }
}

impl Display for ReturnValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NodeInterface for ReturnValue {
    fn token_literal(&self) -> String {
        format!("{}", self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for ReturnValue {
    fn r#type(&self) -> ObjectType {
        ObjectType::ReturnObj
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Object> for ReturnValue {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::ReturnValue(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
