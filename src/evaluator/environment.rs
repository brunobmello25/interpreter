use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use super::object::Object;

#[derive(PartialEq, Clone)]
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

// custom debug implementation to avoid infinite recursion
impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Environment")
            .field("store", &self.store)
            .field("outer", &self.outer.is_some()) // Simply print whether outer is Some or None
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_nest_level() {
        let env = Environment::new();
        let env2 = Environment::with_outer(Rc::clone(&env));
        let env3 = Environment::with_outer(Rc::clone(&env2));
        assert_eq!(env3.borrow().get("a"), None);
        assert_eq!(env2.borrow().get("a"), None);
        assert_eq!(env.borrow().get("a"), None);
        assert_eq!(env.borrow_mut().set("a", Object::Integer(1)), None);
        assert_eq!(env3.borrow().get("a"), Some(Object::Integer(1)));
        assert_eq!(env2.borrow().get("a"), Some(Object::Integer(1)));
        assert_eq!(env.borrow().get("a"), Some(Object::Integer(1)));
    }

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
