use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Integer {
    value: isize,
}

impl Integer {
    pub fn new(value: isize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> isize {
        self.value
    }
}

impl ObjectInterface for Integer {
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NodeInterface for Integer {
    fn token_literal(&self) -> &str {
        "integer"
    }
}

impl TryFrom<Object> for Integer {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Integer(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        Self { value }
    }
}
