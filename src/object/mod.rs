use crate::ast::Node;
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::return_value::ReturnValue;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

pub mod boolean;
pub mod environment;
pub mod integer;
pub mod return_value;
pub mod unit;

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    NULL_OBJ,
    UNIT_OBJ,
    RETURN_OBJ,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INTEGER_OBJ => write!(f, "INTEGER"),
            Self::BOOLEAN_OBJ => write!(f, "BOOLEAN"),
            Self::NULL_OBJ => write!(f, "NULL"),
            Self::UNIT_OBJ => write!(f, "UNIT"),
            Self::RETURN_OBJ => write!(f, "RETURN"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Boolean(Boolean),
    Integer(Integer),
    Unit(()),
    ReturnValue(ReturnValue),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Integer(value) => write!(f, "{}", value),
            Object::Unit(value) => write!(f, "{:?}", value),
            Object::ReturnValue(value) => write!(f, "{:?}", value),
        }
    }
}

impl Node for Object {
    fn token_literal(&self) -> String {
        match self {
            Object::Boolean(value) => value.token_literal(),
            Object::Integer(value) => value.token_literal(),
            Object::Unit(value) => value.token_literal(),
            Object::ReturnValue(value) => value.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        match self {
            Object::Boolean(value) => Node::as_any(&*value),
            Object::Integer(value) => Node::as_any(&*value),
            Object::Unit(value) => Node::as_any(&*value),
            Object::ReturnValue(value) => Node::as_any(&*value),
        }
    }
}

impl ObjectInterface for Object {
    fn r#type(&self) -> ObjectType {
        match self {
            Object::Boolean(value) => value.r#type(),
            Object::Integer(value) => value.r#type(),
            Object::Unit(value) => value.r#type(),
            Object::ReturnValue(value) => value.r#type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Object::Boolean(value) => value.inspect(),
            Object::Integer(value) => value.inspect(),
            Object::Unit(value) => value.inspect(),
            Object::ReturnValue(value) => value.inspect(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        match self {
            Object::Boolean(value) => ObjectInterface::as_any(&*value),
            Object::Integer(value) => ObjectInterface::as_any(&*value),
            Object::Unit(value) => ObjectInterface::as_any(&*value),
            Object::ReturnValue(value) => ObjectInterface::as_any(&*value),
        }
    }
}
/// define object interface
pub trait ObjectInterface {
    fn r#type(&self) -> ObjectType;

    fn inspect(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}
