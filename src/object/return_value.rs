use std::any::Any;
use crate::ast::Node;
use crate::object::{Object, ObjectInterface, ObjectType};

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Box<Object>,
}

impl Node for ReturnValue {
    fn token_literal(&self) -> String {
        "ReturnValue".to_string()
    }

    fn as_any(&self) -> &dyn Any {
       self
    }
}

impl ObjectInterface for ReturnValue {
    fn r#type(&self) -> ObjectType {
        ObjectType::RETURN_OBJ
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}