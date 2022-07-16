use std::collections::HashMap;
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl  Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<&Object> {
        self.store.get(&name)
    }

    pub fn store(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }
}
