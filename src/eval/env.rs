use super::Obj;
use crate::ast::Ident;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Environment {
    store: HashMap<Ident, Rc<Obj>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &Ident) -> Option<Rc<Obj>> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => {
                if let Some(outer) = &self.outer {
                    outer.borrow().get(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: &Ident, value: Rc<Obj>) {
        self.store.insert(name.into(), value);
    }
}
