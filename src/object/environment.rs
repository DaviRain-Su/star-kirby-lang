use crate::object::Object;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Environment {
    store: BTreeMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Environment) -> Self {
        let mut env = Environment::new();
        env.outer = Some(Box::new(outer));
        env
    }

    pub fn get(&self, name: String) -> Option<&Object> {
        let ret = self.store.get(&name);
        if ret.is_none() && self.outer.is_some() {
            return self.outer.as_ref().unwrap().get(name);
        }

        ret
    }

    pub fn store(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }
}
