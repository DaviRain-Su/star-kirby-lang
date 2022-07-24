use crate::ast::Node;
use crate::object::{ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct Boolean {
    pub value: bool,
}

impl ObjectInterface for Boolean {
    fn r#type(&self) -> ObjectType {
        ObjectType::BOOLEAN_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
