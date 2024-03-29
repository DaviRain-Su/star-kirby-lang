use crate::ast::NodeInterface;
use crate::error::Error;

use crate::object::{Object, ObjectInterface, ObjectType};
use std::fmt::{Display, Formatter};
use std::ops::Index;

const ARRAY: &str = "array";

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Array {
    elements: Vec<Object>,
}

impl Index<usize> for Array {
    type Output = Object;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl Array {
    pub fn new(elements: Vec<Object>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &Vec<Object> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Object> {
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
    fn token_literal(&self) -> &str {
        ARRAY
    }
}

impl ObjectInterface for Array {
    fn object_type(&self) -> ObjectType {
        ObjectType::Array
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "[{}]", elements)
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
