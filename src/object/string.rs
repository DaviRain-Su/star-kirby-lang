use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::Node;
use crate::object::{ObjectInterface, ObjectType};

#[derive(Debug, Clone)]
pub struct StringObj {
    pub value: String,
}

impl Display for StringObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectInterface for StringObj {
    fn r#type(&self) -> ObjectType {
        ObjectType::STRING_OBJ
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for StringObj {
    fn token_literal(&self) -> String {
        self.value.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}