use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};

const NULL: &str = "null";

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Null;

impl Display for Null {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NULL)
    }
}

impl NodeInterface for Null {
    fn token_literal(&self) -> String {
        format!("{self}")
    }
}

impl ObjectInterface for Null {
    fn r#type(&self) -> ObjectType {
        ObjectType::NullObj
    }

    fn inspect(&self) -> String {
        NULL.to_string()
    }
}

impl TryFrom<Object> for Null {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Null(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
