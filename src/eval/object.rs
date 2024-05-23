use super::{builtin::Builtin, Environment};
use crate::ast::FuncExpr;
use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Obj {
    Integer(i64),
    Bool(bool),
    String(String),

    Return(Rc<Obj>),
    Func(FuncObj),
    Builtin(Builtin),
    Array(ArrayObj),
    Hash(HashObj),

    Null,
}

impl Obj {
    pub fn is_truthy(&self) -> bool {
        match self {
            Obj::Integer(0) => false,
            Obj::Integer(_) => true,
            Obj::Bool(b) => *b,
            Obj::Null => false,
            Obj::Return(o) => o.is_truthy(),
            _ => false,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Obj::Integer(_) => "INTEGER",
            Obj::Bool(_) => "BOOL",
            Obj::String(_) => "STRING",
            Obj::Null => "NULL",
            Obj::Return(_) => "RETURN",
            Obj::Func(_) => "FUNCTION",
            Obj::Builtin(_) => "BUILTIN",
            Obj::Array(_) => "ARRAY",
            Obj::Hash(_) => "HASH",
        }
    }
}

impl Hash for Obj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Obj::Integer(v) => v.hash(state),
            Obj::String(v) => v.hash(state),
            Obj::Bool(v) => v.hash(state),
            _ => panic!("Cannot hash object of type {}", self.kind()),
        }
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::Integer(x) => write!(f, "{}", x),
            Obj::Bool(x) => write!(f, "{}", x),
            Obj::String(s) => write!(f, "{}", s),
            Obj::Null => write!(f, "null"),
            Obj::Return(o) => write!(f, "{}", o),
            Obj::Func(o) => write!(f, "{}", o),
            Obj::Builtin(_) => write!(f, "builtin"),
            Obj::Array(a) => write!(f, "{}", a),
            Obj::Hash(h) => write!(f, "{}", h),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncObj {
    pub expr: FuncExpr,
    pub env: Rc<RefCell<Environment>>,
}

impl Display for FuncObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayObj {
    pub elements: Vec<Rc<Obj>>,
}

impl Display for ArrayObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (idx, s) in self.elements.iter().enumerate() {
            if idx != self.elements.len() - 1 {
                write!(f, "{}, ", s)?;
            } else {
                write!(f, "{}", s)?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashObj {
    pub map: HashMap<Rc<Obj>, Rc<Obj>>,
}

impl Display for HashObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (idx, (k, v)) in self.map.iter().enumerate() {
            if idx != self.map.len() - 1 {
                write!(f, "{}: {}, ", k, v)?;
            } else {
                write!(f, "{}: {}", k, v)?;
            }
        }
        write!(f, "}}")
    }
}
