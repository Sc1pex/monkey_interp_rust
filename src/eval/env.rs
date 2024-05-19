use super::object::Object;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: &Environment) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer.clone())),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj),
            None => {
                if let Some(outer) = &self.outer {
                    outer.get(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}
