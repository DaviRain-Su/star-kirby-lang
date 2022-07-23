use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::Node;
use crate::object::{ObjectInterface, ObjectType};

#[derive(Debug, Clone, Copy)]
pub struct Null;


impl Display for Null {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl Node for Null {
    fn token_literal(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Null {
    fn r#type(&self) -> ObjectType {
        ObjectType::NULL_OBJ
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
