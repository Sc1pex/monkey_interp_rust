use super::{EvalResult, Object};
use crate::ast::Ident;

#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Len,
}

impl Builtin {
    pub fn from_ident(ident: &Ident) -> Option<Object> {
        match ident.as_str() {
            "len" => Some(Object::Builtin(Builtin::Len)),
            _ => None,
        }
    }

    pub fn call(&self, args: Vec<Object>) -> EvalResult {
        match self {
            Builtin::Len => len(args),
        }
    }
}

fn len(args: Vec<Object>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i64)),
        _ => Err(format!(
            "argument to `len` not supported, got {}",
            args[0].kind()
        )),
    }
}
