use crate::error::Error;
use crate::object::built_in_function::Builtin;
use crate::object::built_in_function::{
    array_first_element, array_last_element, array_push_element, array_rest_element, process_len,
    puts,
};
use std::collections::HashMap;

lazy_static! {
    static ref BUILTINS: HashMap<String, Builtin> = {
        let mut m = HashMap::new();
        m.insert("len".into(), Builtin::new(process_len));
        m.insert("first".into(), Builtin::new(array_first_element));
        m.insert("last".into(), Builtin::new(array_last_element));
        m.insert("rest".into(), Builtin::new(array_rest_element));
        m.insert("push".into(), Builtin::new(array_push_element));
        m.insert("puts".into(), Builtin::new(puts));
        m
    };
}

pub fn lookup_builtin(ident: &str) -> anyhow::Result<Builtin> {
    match BUILTINS.get(ident) {
        Some(value) => Ok(value.clone()),
        None => Err(Error::NoFoundBuildInFunction(ident.into()).into()),
    }
}
