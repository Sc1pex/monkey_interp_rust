use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Bool(bool),
    Null,
    Return(Box<Object>),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Integer(0) => false,
            Object::Integer(_) => true,
            Object::Bool(b) => *b,
            Object::Null => false,
            Object::Return(o) => o.is_truthy(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", x),
            Object::Null => write!(f, "null"),
            Object::Return(o) => write!(f, "{}", o),
        }
    }
}