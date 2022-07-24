use crate::ast::NodeInterface;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use string_join::display::Join;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Hash {
    pub pairs: BTreeMap<Object, Object>,
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pairs = vec![];
        for (key, value) in self.pairs.iter() {
            pairs.push(format!(r#""{}": "{}""#, key, value));
        }

        write!(f, "{{{}}}", ",".join(pairs))
    }
}

impl ObjectInterface for Hash {
    fn r#type(&self) -> ObjectType {
        ObjectType::HASH_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NodeInterface for Hash {
    fn token_literal(&self) -> String {
        "hash".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Object> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Hash(value) => Ok(value.clone()),
            _ => Err(anyhow::anyhow!("unknown Object type")),
        }
    }
}
