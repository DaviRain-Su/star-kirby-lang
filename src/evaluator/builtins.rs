use crate::object::built_in_function::Builtin;
use std::collections::HashMap;

lazy_static! {
    static ref BUILTINS: HashMap<String, Builtin> = {
        let mut m = HashMap::new();
        m.insert("len".to_string(), Builtin::new());
        m
    };
}


pub fn lookup_builtin(ident: &str) -> anyhow::Result<Builtin> {
    match BUILTINS.get(ident) {
        Some(value) => Ok(value.clone()),
        None => Err(anyhow::anyhow!("no found {}", ident)),
    }
}
