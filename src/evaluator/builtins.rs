use crate::object::built_in_function::Builtin;
use std::collections::HashMap;
use crate::object::built_in_function::{
    process_len,
    array_first_element,
    array_last_element,
    array_rest_element,
    array_push_element
};

lazy_static! {
    static ref BUILTINS: HashMap<String, Builtin> = {
        let mut m = HashMap::new();
        m.insert("len".to_string(), Builtin::new(process_len));
        m.insert("first".to_string(), Builtin::new(array_first_element));
         m.insert("last".to_string(), Builtin::new(array_last_element));
         m.insert("rest".to_string(), Builtin::new(array_rest_element));
         m.insert("push".to_string(), Builtin::new(array_push_element));
        m
    };
}

pub fn lookup_builtin(ident: &str) -> anyhow::Result<Builtin> {
    match BUILTINS.get(ident) {
        Some(value) => Ok(value.clone()),
        None => Err(anyhow::anyhow!("no found {}", ident)),
    }
}
