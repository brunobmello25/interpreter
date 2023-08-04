use std::collections::HashMap;

use super::object::Object;

pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, val: Object) -> Option<Object> {
        self.store.insert(name.to_string(), val)
    }
}
