use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::{Identifier, Node};
use crate::object::{ObjectInterface, ObjectType};
use crate::object::ObjectType::QUOTE_OBJ;


#[derive(Debug)]
pub struct Quote {
    pub node: &'static dyn Node,
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUOTE({})", self.node)
    }
}


impl Node for Quote {
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


#[test]
fn test_create_quote() {
    let identitier = Identifier::default();

    let quote = Quote {
        node: Box::new(identitier),
    };

    println!("Quote = {:?}", quote);
}