use crate::ast::Node;
use crate::object::{ObjectInterface, ObjectType};
use std::any::Any;

impl ObjectInterface for () {
    fn r#type(&self) -> ObjectType {
        ObjectType::NULL_OBJ
    }

    fn inspect(&self) -> String {
        "unit".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for () {
    fn token_literal(&self) -> String {
        "()".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
