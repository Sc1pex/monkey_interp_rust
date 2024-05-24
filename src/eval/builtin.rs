use super::{EvalResult, Object};
use crate::ast::Ident;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

impl Builtin {
    pub fn from_ident(ident: &Ident) -> Option<Rc<Object>> {
        match ident.as_str() {
            "len" => Some(Rc::new(Object::Builtin(Builtin::Len))),
            "first" => Some(Rc::new(Object::Builtin(Builtin::First))),
            "last" => Some(Rc::new(Object::Builtin(Builtin::Last))),
            "rest" => Some(Rc::new(Object::Builtin(Builtin::Rest))),
            "push" => Some(Rc::new(Object::Builtin(Builtin::Push))),
            "puts" => Some(Rc::new(Object::Builtin(Builtin::Puts))),
            _ => None,
        }
    }

    pub fn call(&self, args: Vec<Rc<Object>>) -> EvalResult {
        match self {
            Builtin::Len => len(args),
            Builtin::First => first(args),
            Builtin::Last => last(args),
            Builtin::Rest => rest(args),
            Builtin::Push => push(args),
            Builtin::Puts => puts(args),
        }
    }
}

fn len(args: Vec<Rc<Object>>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &*args[0] {
        Object::String(s) => Ok(Rc::new(Object::Integer(s.len() as i64))),
        Object::Array(a) => Ok(Rc::new(Object::Integer(a.elements.len() as i64))),
        _ => Err(format!(
            "argument to `len` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn first(args: Vec<Rc<Object>>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => Ok(a.elements.first().cloned().unwrap_or(Rc::new(Object::Null))),
        _ => Err(format!(
            "argument to `first` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn last(args: Vec<Rc<Object>>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => Ok(a.elements.last().cloned().unwrap_or(Rc::new(Object::Null))),
        _ => Err(format!(
            "argument to `last` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn rest(args: Vec<Rc<Object>>) -> EvalResult {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let elements = a.elements.clone().into_iter().skip(1).collect();
            Ok(Rc::new(Object::Array(super::ArrayObj { elements })))
        }
        _ => Err(format!(
            "argument to `rest` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn push(args: Vec<Rc<Object>>) -> EvalResult {
    if args.len() != 2 {
        return Err(format!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let mut elements = a.elements.clone();
            elements.push(args[1].clone());
            Ok(Rc::new(Object::Array(super::ArrayObj { elements })))
        }
        _ => Err(format!(
            "argument to `push` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn puts(args: Vec<Rc<Object>>) -> EvalResult {
    for arg in args {
        println!("{}", arg);
    }
    Ok(Rc::new(Object::Null))
}
