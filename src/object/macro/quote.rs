use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::{NodeInterface, Node};
use crate::object::{Object, ObjectInterface, ObjectType};
use crate::object::ObjectType::QUOTE_OBJ;


#[derive(Debug, Clone, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct Quote {
    pub node: Box<Node>,
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUOTE({})", self.node)
    }
}


impl NodeInterface for Quote {
    fn token_literal(&self) -> String {
        "quote".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Quote {
    fn r#type(&self) -> ObjectType {
        QUOTE_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Object> for Quote {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Quote(value) => Ok(value.clone()),
            _ => Err(anyhow::anyhow!("unknown Object type")),
        }
    }
}


#[test]
fn test_create_quote() {
    let identitier = Identifier::default();

    let quote = Quote {
        node: Box::new(Node::Expression(Expression::IdentifierExpression(identitier))),
    };

    println!("Quote = {:?}", quote);
}