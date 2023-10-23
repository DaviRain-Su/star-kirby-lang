use crate::ast::NodeInterface;
use crate::error::Error;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub const HASH: &str = "hash";

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Hash {
    pairs: BTreeMap<Object, Object>,
}

impl Hash {
    pub fn new(pairs: BTreeMap<Object, Object>) -> Self {
        Self { pairs }
    }

    pub fn pairs(&self) -> &BTreeMap<Object, Object> {
        &self.pairs
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.pairs.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, r#""{key}": "{value}""#)?;
        }

        write!(f, "}}")
    }
}

impl ObjectInterface for Hash {
    fn object_type(&self) -> ObjectType {
        ObjectType::Hash
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }
}

impl NodeInterface for Hash {
    fn token_literal(&self) -> &str {
        HASH.into()
    }
}

impl TryFrom<Object> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Hash(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}
