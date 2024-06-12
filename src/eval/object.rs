use super::{builtin::Builtin, Environment};
use crate::{ast::FuncExpr, compiler::Bytes};
use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(i64),
    Bool(bool),
    String(String),

    Return(Rc<Object>),
    Func(FuncObj),
    CompiledFunc(Rc<CompiledFuncObj>),
    Builtin(Builtin),
    Array(ArrayObj),
    Hash(HashObj),

    Null,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Integer(0) => false,
            Object::Integer(_) => true,
            Object::Bool(b) => *b,
            Object::Null => false,
            Object::Return(o) => o.is_truthy(),
            _ => false,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Bool(_) => "BOOL",
            Object::String(_) => "STRING",
            Object::Null => "NULL",
            Object::Return(_) => "RETURN",
            Object::Func(_) => "FUNCTION",
            Object::CompiledFunc(_) => "COMPILED FUNCTION",
            Object::Builtin(_) => "BUILTIN",
            Object::Array(_) => "ARRAY",
            Object::Hash(_) => "HASH",
        }
    }
}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer(v) => v.hash(state),
            Object::String(v) => v.hash(state),
            Object::Bool(v) => v.hash(state),
            _ => panic!("Cannot hash object of type {}", self.kind()),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", x),
            Object::String(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
            Object::Return(o) => write!(f, "{}", o),
            Object::Func(o) => write!(f, "{}", o),
            Object::CompiledFunc(o) => write!(f, "{}", o),
            Object::Builtin(_) => write!(f, "builtin"),
            Object::Array(a) => write!(f, "{}", a),
            Object::Hash(h) => write!(f, "{}", h),
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
pub struct CompiledFuncObj {
    pub instructions: Bytes,
    pub locals: usize,
    pub params: usize,
}

impl CompiledFuncObj {
    pub fn new(instructions: Bytes, locals: usize, params: usize) -> Self {
        Self {
            instructions,
            locals,
            params,
        }
    }
}

impl Display for CompiledFuncObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} {} locals", self.instructions, self.locals)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayObj {
    pub elements: Vec<Rc<Object>>,
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
    pub map: HashMap<Rc<Object>, Rc<Object>>,
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
