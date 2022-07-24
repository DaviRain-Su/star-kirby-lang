use crate::ast::Node;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::display::Join;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct Array {
    pub elements: Vec<Box<Object>>,
}

impl Node for Array {
    fn token_literal(&self) -> String {
        "array".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Array {
    fn r#type(&self) -> ObjectType {
        ObjectType::ARRAY_OBJ
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
