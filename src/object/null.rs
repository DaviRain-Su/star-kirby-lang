use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Null;

impl Display for Null {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl NodeInterface for Null {
    fn token_literal(&self) -> String {
        format!("{self}")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Null {
    fn r#type(&self) -> ObjectType {
        ObjectType::NullObj
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
