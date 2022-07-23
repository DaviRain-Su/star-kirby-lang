use crate::ast::Node;
use crate::object::boolean::Boolean;
use crate::object::built_in_function::Builtin;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::return_value::ReturnValue;
use crate::object::string::StringObj;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use crate::object::array::Array;

pub mod boolean;
pub mod built_in_function;
pub mod environment;
pub mod function;
pub mod integer;
pub mod return_value;
pub mod string;
pub mod unit;
pub mod array;

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    NULL_OBJ,
    UNIT_OBJ,
    RETURN_OBJ,
    FUNCTION_OBJ,
    STRING_OBJ,
    BUILTIN_OBJ,
    ARRAY_OBJ,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INTEGER_OBJ => write!(f, "INTEGER"),
            Self::BOOLEAN_OBJ => write!(f, "BOOLEAN"),
            Self::NULL_OBJ => write!(f, "NULL"),
            Self::UNIT_OBJ => write!(f, "UNIT"),
            Self::RETURN_OBJ => write!(f, "RETURN"),
            Self::FUNCTION_OBJ => write!(f, "FUNCTION"),
            Self::STRING_OBJ => write!(f, "STRING"),
            Self::BUILTIN_OBJ => write!(f, "BUILTIN"),
            Self::ARRAY_OBJ => write!(f, "ARRAY"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Boolean(Boolean),
    Integer(Integer),
    Unit(()),
    ReturnValue(ReturnValue),
    Function(Function),
    String(StringObj),
    Builtin(Builtin),
    Array(Array),
}

impl From<Boolean> for Object {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<Integer> for Object {
    fn from(integer: Integer) -> Self {
        Self::Integer(integer)
    }
}

impl From<()> for Object {
    fn from(_: ()) -> Self {
        Self::Unit(())
    }
}

impl From<ReturnValue> for Object {
    fn from(value: ReturnValue) -> Self {
        Self::ReturnValue(value)
    }
}

impl From<Function> for Object {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}


impl From<StringObj> for Object {
    fn from(value: StringObj) -> Self {
        Self::String(value)
    }
}


impl From<Builtin> for Object {
    fn from(value: Builtin) -> Self {
        Self::Builtin(value)
    }
}

impl From<Array> for Object {
    fn from(array: Array) -> Self {
        Self::Array(array)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Unit(value) => write!(f, "{:?}", value),
            Self::ReturnValue(value) => write!(f, "{:?}", value),
            Self::Function(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Builtin(value) => write!(f, "{}", value),
            Self::Array(value) => write!(f, "{}", value),
        }
    }
}

impl Node for Object {
    fn token_literal(&self) -> String {
        match self {
            Self::Boolean(value) => value.token_literal(),
            Self::Integer(value) => value.token_literal(),
            Self::Unit(value) => value.token_literal(),
            Self::ReturnValue(value) => value.token_literal(),
            Self::Function(value) => value.token_literal(),
            Self::String(value) => value.token_literal(),
            Self::Builtin(value) => value.token_literal(),
            Self::Array(value) => value.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        match self {
            Self::Boolean(value) => Node::as_any(value),
            Self::Integer(value) => Node::as_any(value),
            Self::Unit(value) => Node::as_any(value),
            Self::ReturnValue(value) => Node::as_any(value),
            Self::Function(value) => Node::as_any(value),
            Self::String(value) => Node::as_any(value),
            Self::Builtin(value) => Node::as_any(value),
            Self::Array(value) => Node::as_any(value),
        }
    }
}

impl ObjectInterface for Object {
    fn r#type(&self) -> ObjectType {
        match self {
            Self::Boolean(value) => value.r#type(),
            Self::Integer(value) => value.r#type(),
            Self::Unit(value) => value.r#type(),
            Self::ReturnValue(value) => value.r#type(),
            Self::Function(value) => value.r#type(),
            Self::String(value) => value.r#type(),
            Self::Builtin(value) => value.r#type(),
            Self::Array(value) => value.r#type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Self::Boolean(value) => value.inspect(),
            Self::Integer(value) => value.inspect(),
            Self::Unit(value) => value.inspect(),
            Self::ReturnValue(value) => value.inspect(),
            Self::Function(value) => value.inspect(),
            Self::String(value) => value.inspect(),
            Self::Builtin(value) => value.inspect(),
            Self::Array(value) => value.inspect(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        match self {
            Self::Boolean(value) => ObjectInterface::as_any(value),
            Self::Integer(value) => ObjectInterface::as_any(value),
            Self::Unit(value) => ObjectInterface::as_any(value),
            Self::ReturnValue(value) => ObjectInterface::as_any(value),
            Self::Function(value) => ObjectInterface::as_any(value),
            Self::String(value) => ObjectInterface::as_any(value),
            Self::Builtin(value) => ObjectInterface::as_any(value),
            Self::Array(value) => ObjectInterface::as_any(value),
        }
    }
}


/// define object interface
pub trait ObjectInterface {
    fn r#type(&self) -> ObjectType;

    fn inspect(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}
