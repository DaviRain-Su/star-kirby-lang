use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::ops::Index;
use string_join::display::Join;

const ARRAY: &str = "array";

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Array {
    elements: Vec<Box<Object>>,
}

impl Index<usize> for Array {
    type Output = Box<Object>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl Array {
    pub fn new(elements: Vec<Box<Object>>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &Vec<Box<Object>> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Box<Object>> {
        &mut self.elements
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl NodeInterface for Array {
    fn token_literal(&self) -> String {
        ARRAY.to_string()
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
        format!("{self}")
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
            Object::Array(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
