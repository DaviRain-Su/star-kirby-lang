use crate::ast::Node;
use crate::object::integer::Integer;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Builtin {
    pub built_in_function: Box<fn(Vec<Object>) -> anyhow::Result<Object>>,
}

impl Builtin {
    pub fn new() -> Self {
        Self {
            built_in_function: Box::new(process_len),
        }
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "built_in_function")
    }
}

fn process_len(args: Vec<Object>) -> anyhow::Result<Object> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }

    match args[0].clone() {
        Object::String(string_obj) => Ok(Object::Integer(Integer {
            value: string_obj.value.len() as i64,
        })),
        _ => Err(anyhow::anyhow!(format!(
            "argument to `len` not supported, got {}",
            args[0].r#type()
        ))),
    }
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

impl From<Builtin> for Object {
    fn from(value: Builtin) -> Self {
        Object::Builtin(value)
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
