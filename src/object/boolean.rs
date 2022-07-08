use crate::object::{Object, ObjectType};

#[derive(Debug)]
pub struct  Boolean {
    pub value: bool,
}


impl Object for Boolean {
    fn r#type(&self) -> ObjectType {
        ObjectType::BOOLEAN_OBJ
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}