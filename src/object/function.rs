use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::Identifier;
use crate::object::environment::Environment;
use crate::object::{ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};
use string_join::Join;

#[derive(Debug, Clone)]
struct Function {
    parameters: Vec<Identifier>,
    body: BlockStatement,
    env: Environment,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![];
        for p in self.parameters.iter() {
            params.push(format!("{}", p));
        }
        write!(f, "fn")?;
        write!(f, "(")?;
        write!(f, "{}", ", ".join(params))?;
        write!(f, ") {{\n")?;
        write!(f, "{}", self.body)?;
        write!(f, "\n}}")
    }
}

impl ObjectInterface for Function {
    fn r#type(&self) -> ObjectType {
        ObjectType::FUNCTION_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
