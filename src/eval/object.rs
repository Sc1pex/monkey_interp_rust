use super::{builtin::Builtin, Environment};
use crate::ast::FuncExpr;
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Bool(bool),
    String(String),

    Return(Box<Object>),
    Func(FuncObj),
    Builtin(Builtin),
    Array(ArrayObj),

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
            Object::Builtin(_) => "BUILTIN",
            Object::Array(_) => "ARRAY",
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
            Object::Builtin(_) => write!(f, "builtin"),
            Object::Array(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncObj {
    pub expr: FuncExpr,
    pub env: Rc<RefCell<Environment>>,
}

impl Display for FuncObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayObj {
    pub elements: Vec<Object>,
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
