use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct StringObj {
    value: String,
}

impl StringObj {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl Display for StringObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectInterface for StringObj {
    fn r#type(&self) -> ObjectType {
        ObjectType::StringObj
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }
}

impl NodeInterface for StringObj {
    fn token_literal(&self) -> String {
        self.value.clone()
    }
}

impl TryFrom<Object> for StringObj {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::String(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
