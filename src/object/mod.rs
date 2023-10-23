use crate::ast::NodeInterface;
use crate::object::array::Array;
use crate::object::boolean::Boolean;
use crate::object::built_in_function::Builtin;
use crate::object::function::Function;
use crate::object::hash::Hash;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::r#macro::quote::Quote;
use crate::object::return_value::ReturnValue;
use crate::object::string::StringObj;
use std::fmt::{Debug, Display, Formatter};

pub mod array;
pub mod boolean;
pub mod built_in_function;
pub mod environment;
pub mod function;
pub mod hash;
pub mod integer;
pub mod r#macro;
pub mod null;
pub mod return_value;
pub mod string;

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    IntegerObj,
    BooleanObj,
    NullObj,
    ReturnObj,
    FunctionObj,
    StringObj,
    BuiltinObj,
    ArrayObj,
    HashObj,
    QueueObj,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntegerObj => write!(f, "INTEGER"),
            Self::BooleanObj => write!(f, "BOOLEAN"),
            Self::NullObj => write!(f, "NULL"),
            Self::ReturnObj => write!(f, "RETURN"),
            Self::FunctionObj => write!(f, "FUNCTION"),
            Self::StringObj => write!(f, "STRING"),
            Self::BuiltinObj => write!(f, "BUILTIN"),
            Self::ArrayObj => write!(f, "ARRAY"),
            Self::HashObj => write!(f, "HASH"),
            Self::QueueObj => write!(f, "QUOTE"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub enum Object {
    Boolean(Boolean),
    Integer(Integer),
    ReturnValue(ReturnValue),
    Function(Function),
    String(StringObj),
    Builtin(Builtin),
    Array(Array),
    Null(Null),
    Hash(Hash),
    Quote(Quote),
}

impl From<Boolean> for Object {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<once_cell::sync::Lazy<Boolean>> for Object {
    fn from(value: once_cell::sync::Lazy<Boolean>) -> Self {
        Self::Boolean(*value)
    }
}
impl From<Integer> for Object {
    fn from(integer: Integer) -> Self {
        Self::Integer(integer)
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

impl From<Null> for Object {
    fn from(_: Null) -> Self {
        Self::Null(Null)
    }
}

impl From<Hash> for Object {
    fn from(hash: Hash) -> Self {
        Self::Hash(hash)
    }
}

impl From<Quote> for Object {
    fn from(quote: Quote) -> Self {
        Self::Quote(quote)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::ReturnValue(value) => write!(f, "{value}"),
            Self::Function(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Builtin(value) => write!(f, "{value}"),
            Self::Array(value) => write!(f, "{value}"),
            Self::Null(value) => write!(f, "{value}"),
            Self::Hash(value) => write!(f, "{value}"),
            Self::Quote(value) => write!(f, "{value}"),
        }
    }
}

impl NodeInterface for Object {
    fn token_literal(&self) -> String {
        match self {
            Self::Boolean(value) => value.token_literal(),
            Self::Integer(value) => value.token_literal(),
            Self::ReturnValue(value) => value.token_literal(),
            Self::Function(value) => value.token_literal(),
            Self::String(value) => value.token_literal(),
            Self::Builtin(value) => value.token_literal(),
            Self::Array(value) => value.token_literal(),
            Self::Null(value) => value.token_literal(),
            Self::Hash(value) => value.token_literal(),
            Self::Quote(value) => value.token_literal(),
        }
    }
}

impl ObjectInterface for Object {
    fn object_type(&self) -> ObjectType {
        match self {
            Self::Boolean(value) => value.object_type(),
            Self::Integer(value) => value.object_type(),
            Self::ReturnValue(value) => value.object_type(),
            Self::Function(value) => value.object_type(),
            Self::String(value) => value.object_type(),
            Self::Builtin(value) => value.object_type(),
            Self::Array(value) => value.object_type(),
            Self::Null(value) => value.object_type(),
            Self::Hash(value) => value.object_type(),
            Self::Quote(value) => value.object_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Self::Boolean(value) => value.inspect(),
            Self::Integer(value) => value.inspect(),
            Self::ReturnValue(value) => value.inspect(),
            Self::Function(value) => value.inspect(),
            Self::String(value) => value.inspect(),
            Self::Builtin(value) => value.inspect(),
            Self::Array(value) => value.inspect(),
            Self::Null(value) => value.inspect(),
            Self::Hash(value) => value.inspect(),
            Self::Quote(value) => value.inspect(),
        }
    }
}

/// define object interface
pub trait ObjectInterface {
    fn object_type(&self) -> ObjectType;

    fn inspect(&self) -> String;
}
