use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol {
    pub scope: Scope,
    pub index: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Global,
    Local,
    Builtin,
}

pub type SymbolTableRef = Rc<RefCell<SymbolTable>>;

#[derive(Clone)]
pub struct SymbolTable {
    pub outer: Option<SymbolTableRef>,
    store: HashMap<String, Symbol>,
    stored: usize,
}

impl SymbolTable {
    pub fn empty() -> SymbolTableRef {
        Rc::new(RefCell::new(Self {
            outer: None,
            store: HashMap::default(),
            stored: 0,
        }))
    }

    pub fn new_enclosed(outer: &SymbolTableRef) -> SymbolTableRef {
        Rc::new(RefCell::new(Self {
            outer: Some(outer.clone()),
            store: HashMap::default(),
            stored: 0,
        }))
    }

    pub fn define(&mut self, name: &str) -> Symbol {
        let scope = if self.outer.is_some() {
            Scope::Local
        } else {
            Scope::Global
        };

        let sym = Symbol {
            scope,
            index: self.stored as u16,
        };
        self.stored += 1;
        self.store.insert(name.to_string(), sym);
        self.store[name]
    }

    pub fn define_builtin(&mut self, name: &str) -> Symbol {
        let sym = Symbol {
            scope: Scope::Builtin,
            index: self.store.len() as u16,
        };
        self.store.insert(name.to_string(), sym);
        self.store[name]
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        self.store
            .get(name)
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|o| o.borrow().resolve(name)))
    }

    pub fn symbols(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_local() {
        let glob = SymbolTable::empty();
        glob.borrow_mut().define("a");
        glob.borrow_mut().define("b");

        let local1 = SymbolTable::new_enclosed(&glob);
        local1.borrow_mut().define("c");
        local1.borrow_mut().define("d");

        let local2 = SymbolTable::new_enclosed(&glob);
        local2.borrow_mut().define("e");
        local2.borrow_mut().define("f");

        let expected: &[(SymbolTableRef, &[(&'static str, Scope, u16)])] = &[
            (
                local1,
                &[
                    ("a", Scope::Global, 0),
                    ("b", Scope::Global, 1),
                    ("c", Scope::Local, 0),
                    ("d", Scope::Local, 1),
                ],
            ),
            (
                local2,
                &[
                    ("a", Scope::Global, 0),
                    ("b", Scope::Global, 1),
                    ("e", Scope::Local, 0),
                    ("f", Scope::Local, 1),
                ],
            ),
        ];

        for (l, e) in expected {
            for e in *e {
                let r = l
                    .borrow()
                    .resolve(e.0)
                    .expect(&format!("Symbol {} not found", e.0));
                assert_eq!(
                    r,
                    Symbol {
                        scope: e.1,
                        index: e.2
                    },
                    "Symbol {} is wrong",
                    e.0
                );
            }
        }
    }
}
