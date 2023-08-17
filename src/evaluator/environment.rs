use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn with_outer(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(&outer)),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(val) => Some(val.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str, val: Object) -> Option<Object> {
        self.store.insert(name.to_string(), val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let env = Environment::new();
        assert_eq!(env.borrow().get("a"), None);
        assert_eq!(env.borrow_mut().set("a", Object::Integer(1)), None);
        assert_eq!(env.borrow().get("a"), Some(Object::Integer(1)));
    }

    #[test]
    fn test_env_outer() {
        let env = Environment::new();
        let env2 = Environment::with_outer(Rc::clone(&env));
        assert_eq!(env2.borrow().get("a"), None);
        assert_eq!(env.borrow_mut().set("a", Object::Integer(1)), None);
        assert_eq!(env2.borrow().get("a"), Some(Object::Integer(1)));
    }
}
