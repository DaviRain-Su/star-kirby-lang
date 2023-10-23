use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
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
    fn token_literal(&self) -> &str {
        self.value.token_literal()
    }
}

impl ObjectInterface for ReturnValue {
    fn object_type(&self) -> ObjectType {
        ObjectType::Return
    }

    fn inspect(&self) -> String {
        format!("{self}")
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
