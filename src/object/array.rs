use crate::ast::NodeInterface;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::display::Join;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Array {
    pub elements: Vec<Box<Object>>,
}

impl NodeInterface for Array {
    fn token_literal(&self) -> String {
        "array".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Array {
    fn r#type(&self) -> ObjectType {
        ObjectType::ArrayObj
    }

    fn inspect(&self) -> String {
        format!("{}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut elements = vec![];
        for e in self.elements.iter() {
            elements.push(format!("{}", *e));
        }

        write!(f, "[{}]", ",".join(elements))
    }
}

impl TryFrom<Object> for Array {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Array(value) => Ok(value.clone()),
            _ => Err(anyhow::anyhow!("unknown Object type")),
        }
    }
}
