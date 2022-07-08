use std::any::{Any, TypeId};
use std::fmt::{Display, Formatter};
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::null::Null;

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

    fn as_any(&self) -> &dyn Any;
}


pub fn parser_object(obj: Box<dyn Object>) -> anyhow::Result<String> {
    let type_id = obj.as_any().type_id();

    if TypeId::of::<Boolean>() == type_id {
        let value = obj
            .as_any()
            .downcast_ref::<Boolean>().ok_or_else(|| anyhow::anyhow!("downcast_ref boolean error"))?;

        return Ok(value.inspect());
    } else if TypeId::of::<Integer>() == type_id {
        let value = obj
            .as_any()
            .downcast_ref::<Integer>().ok_or_else(|| anyhow::anyhow!("downcast_ref Integer error"))?;

        return Ok(value.inspect());
    } else if TypeId::of::<Null>() == type_id {
        let value = obj
            .as_any()
            .downcast_ref::<Null>().ok_or_else(|| anyhow::anyhow!("downcast_ref Null error"))?;

        return Ok(value.inspect());
    }

    Err(anyhow::anyhow!("parser object error"))
}
