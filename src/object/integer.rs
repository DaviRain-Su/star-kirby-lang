use crate::object::{Object, ObjectType};

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

}