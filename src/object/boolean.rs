use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl ObjectInterface for Boolean {
    fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NodeInterface for Boolean {
    fn token_literal(&self) -> String {
        self.value.to_string()
    }
}

impl TryFrom<Object> for Boolean {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Boolean(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self { value }
    }
}
