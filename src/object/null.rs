use std::any::Any;
use crate::object::{Object, ObjectType};

#[derive(Debug)]
pub struct Null;


impl  Object for Null {
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