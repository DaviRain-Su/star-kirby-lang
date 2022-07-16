use crate::ast::Node;
use crate::object::{ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl ObjectInterface for Integer {
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

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Integer({})", self.value)
    }
}

impl Node for Integer {
    fn token_literal(&self) -> String {
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
