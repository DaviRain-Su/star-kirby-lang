use crate::object::{Object, ObjectType};
use std::any::Any;

#[derive(Debug)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn r#type(&self) -> ObjectType {
        ObjectType::INTEGER_OBJ
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
