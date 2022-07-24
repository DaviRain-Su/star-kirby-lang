use crate::ast::Node;
use crate::object::array::Array;
use crate::object::integer::Integer;
use crate::object::ObjectType::ARRAY_OBJ;
use crate::object::{Object, ObjectInterface, ObjectType};
use crate::NULL;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct Builtin {
    pub built_in_function: Box<fn(Vec<Object>) -> anyhow::Result<Object>>,
}

impl Builtin {
    pub fn new(func: fn(Vec<Object>) -> anyhow::Result<Object>) -> Self {
        Self {
            built_in_function: Box::new(func),
        }
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "built_in_function")
    }
}

pub fn process_len(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }

    match args[0].clone() {
        Object::String(string_obj) => Ok(Integer {
            value: string_obj.value.len() as i64,
        }
        .into()),
        Object::Array(array) => Ok(Integer {
            value: array.elements.len() as i64,
        }
        .into()),
        _ => Err(anyhow::anyhow!(format!(
            "argument to `len` not supported, got {}",
            args[0].r#type()
        ))),
    }
}

pub fn array_first_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }

    if args[0].r#type() != ARRAY_OBJ {
        return Err(anyhow::anyhow!(
            "argument to `first` must ARRAY, got {}",
            args[0].r#type()
        ));
    }

    match args[0].clone() {
        Object::Array(array) if array.elements.len() > 0 => Ok(*array.elements[0].clone().clone()),
        _ => Ok(NULL.into()),
    }
}

pub fn array_last_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }

    if args[0].r#type() != ARRAY_OBJ {
        return Err(anyhow::anyhow!(
            "argument to `first` must ARRAY, got {}",
            args[0].r#type()
        ));
    }

    match args[0].clone() {
        Object::Array(array) if array.elements.len() > 0 => {
            let length = array.elements.len();
            Ok(*array.elements[length - 1].clone().clone())
        }
        _ => Ok(NULL.into()),
    }
}

pub fn array_rest_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }

    if args[0].r#type() != ARRAY_OBJ {
        return Err(anyhow::anyhow!(
            "argument to `first` must ARRAY, got {}",
            args[0].r#type()
        ));
    }

    match args[0].clone() {
        Object::Array(array) if array.elements.len() > 0 => {
            let mut new_elements = array.elements.clone();
            new_elements.remove(0);
            Ok(Array {
                elements: new_elements,
            }
            .into())
        }
        _ => Ok(NULL.into()),
    }
}

pub fn array_push_element(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 2 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        )));
    }

    if args[0].r#type() != ARRAY_OBJ {
        return Err(anyhow::anyhow!(
            "argument to `first` must ARRAY, got {}",
            args[0].r#type()
        ));
    }

    match args[0].clone() {
        Object::Array(array) => {
            let mut array = array.elements.clone();
            array.push(Box::new(args[1].clone()));
            return Ok(Array { elements: array }.into());
        }
        _ => Ok(NULL.into()),
    }
}

pub fn puts(args: Vec<Object>) -> anyhow::Result<Object> {
    for arg in args {
        println!("{}", arg);
    }
    return Ok(NULL.into());
}

impl ObjectInterface for Builtin {
    fn r#type(&self) -> ObjectType {
        ObjectType::BUILTIN_OBJ
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for Builtin {
    fn token_literal(&self) -> String {
        "builtin function".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Object> for Builtin {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Builtin(value) => Ok(value.clone()),
            _ => Err(anyhow::anyhow!("unknown Object type")),
        }
    }
}
