use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Symbol {
    pub scope: Scope,
    pub index: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Global,
}

#[derive(Default, Clone)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn define(&mut self, name: &str) -> Symbol {
        let sym = Symbol {
            scope: Scope::Global,
            index: self.store.len() as u16,
        };
        self.store.insert(name.to_string(), sym);
        self.store[name]
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.store.get(name)
    }
}
