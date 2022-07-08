use std::fmt::{Display, Formatter};

pub mod integer;
pub mod boolean;
pub mod null;

#[derive(Debug)]
pub enum  ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    NULL_OBJ,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INTEGER_OBJ => write!(f, "INTEGER"),
            Self::BOOLEAN_OBJ => write!(f, "BOOLEAN"),
            Self::NULL_OBJ => write!(f, "NULL"),
        }
    }
}

/// define object interface
pub trait Object {
    fn r#type(&self) -> ObjectType;

    fn inspect(&self) -> String;
}

