use crate::object::{Object, ObjectType};
use std::any::Any;


impl Object for () {
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
