use crate::ast::Node;
use crate::object::Object;

#[cfg(test)]
pub mod tests;



fn eval(node: Box<dyn Node>) -> Box<dyn Object> {
    todo!()
}

