use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::array::Array;
use crate::object::integer::Integer;
use crate::object::Null;
use crate::object::ObjectType;
use crate::object::{Object, ObjectInterface};
use std::fmt::{Display, Formatter};

const BUILD_FUNC: &str = "builtin function";

type BuildBoxFuncType = Box<fn(Vec<Object>) -> anyhow::Result<Object>>;
type BuildFuncType = fn(Vec<Object>) -> anyhow::Result<Object>;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Builtin {
    built_in_function: BuildBoxFuncType,
}

impl Builtin {
    pub fn new(func: BuildFuncType) -> Self {
        Self {
            built_in_function: Box::new(func),
        }
    }

    pub fn value(&self) -> &BuildBoxFuncType {
        &self.built_in_function
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BUILD_FUNC}")
    }
}

pub fn process_len(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(Error::WrongNumberOfArguments {
            got: args.len(),
            want: 1,
        }
        .into());
    }

    match args[0].clone() {
        Object::String(string_obj) => Ok(Integer::new(string_obj.value().len() as isize).into()),
        Object::Array(array) => Ok(Integer::new(array.len() as isize).into()),
        _ => Err(Error::ArgumentNotSupported {
            got: args[0].object_type().to_string(),
        }
        .into()),
    }
}

pub fn array_first_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(Error::WrongNumberOfArguments {
            got: args.len(),
            want: 1,
        }
        .into());
    }

    if args[0].object_type() != ObjectType::Array {
        return Err(Error::ArgumentFirstMustArray {
            got: args[0].object_type().to_string(),
        }
        .into());
    }

    match args[0].clone() {
        Object::Array(array) if !array.is_empty() => Ok(array.elements()[0].clone()),
        _ => Ok(Null.into()),
    }
}

pub fn array_last_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(Error::WrongNumberOfArguments {
            got: args.len(),
            want: 1,
        }
        .into());
    }

    if args[0].object_type() != ObjectType::Array {
        return Err(Error::ArgumentFirstMustArray {
            got: args[0].object_type().to_string(),
        }
        .into());
    }

    match args[0].clone() {
        Object::Array(array) if !array.is_empty() => {
            let length = array.len();
            Ok(array.elements()[length - 1].clone())
        }
        _ => Ok(Null.into()),
    }
}

pub fn array_rest_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(Error::WrongNumberOfArguments {
            got: args.len(),
            want: 1,
        }
        .into());
    }

    if args[0].object_type() != ObjectType::Array {
        return Err(Error::ArgumentFirstMustArray {
            got: args[0].object_type().to_string(),
        }
        .into());
    }

    match args[0].clone() {
        Object::Array(mut array) if !array.is_empty() => {
            let new_elements = array.elements_mut();
            new_elements.remove(0);
            Ok(Array::new(new_elements.clone()).into())
        }
        _ => Ok(Null.into()),
    }
}

pub fn array_push_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 2 {
        return Err(Error::WrongNumberOfArguments {
            got: args.len(),
            want: 2,
        }
        .into());
    }

    if args[0].object_type() != ObjectType::Array {
        return Err(Error::ArgumentFirstMustArray {
            got: args[0].object_type().to_string(),
        }
        .into());
    }

    match args[0].clone() {
        Object::Array(mut array) => {
            let array = array.elements_mut();
            array.push(args[1].clone());
            Ok(Array::new(array.clone()).into())
        }
        _ => Ok(Null.into()),
    }
}

pub fn puts(args: Vec<Object>) -> anyhow::Result<Object> {
    for arg in args {
        println!("{arg}");
    }
    Ok(Null.into())
}

impl ObjectInterface for Builtin {
    fn object_type(&self) -> ObjectType {
        ObjectType::Array
    }

    fn inspect(&self) -> String {
        BUILD_FUNC.to_string()
    }
}

impl NodeInterface for Builtin {
    fn token_literal(&self) -> &str {
        BUILD_FUNC
    }
}

impl TryFrom<Object> for Builtin {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Builtin(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
