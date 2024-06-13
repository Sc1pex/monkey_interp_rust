use crate::{
    ast::Ident,
    eval::{ArrayObj, Object},
};
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

impl Builtin {
    pub fn from_ident_obj(ident: &Ident) -> Option<Rc<Object>> {
        Self::from_ident(ident).map(|s| Rc::new(Object::Builtin(s)))
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        if value > std::mem::variant_count::<Self>() as u8 {
            None
        } else {
            unsafe { Some(std::mem::transmute(value)) }
        }
    }

    pub fn from_ident(ident: &Ident) -> Option<Self> {
        match ident.as_str() {
            "len" => Some(Builtin::Len),
            "first" => Some(Builtin::First),
            "last" => Some(Builtin::Last),
            "rest" => Some(Builtin::Rest),
            "push" => Some(Builtin::Push),
            "puts" => Some(Builtin::Puts),
            _ => None,
        }
    }

    pub fn call<T: From<Object> + Display>(&self, args: Vec<&Object>) -> Result<T, String> {
        match self {
            Builtin::Len => len(args).map(Into::into),
            Builtin::First => first(args).map(Into::into),
            Builtin::Last => last(args).map(Into::into),
            Builtin::Rest => rest(args).map(Into::into),
            Builtin::Push => push(args).map(Into::into),
            Builtin::Puts => puts(args).map(Into::into),
        }
    }
}

fn len(args: Vec<&Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. expected 1, got {}",
            args.len()
        ));
    }

    match &*args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i64).into()),
        Object::Array(a) => Ok(Object::Integer(a.elements.len() as i64).into()),
        _ => Err(format!(
            "argument to `len` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn first(args: Vec<&Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. expected 1, got {}",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let f = a
                .elements
                .first()
                .cloned()
                .map(|r| (*r).clone())
                .unwrap_or(Object::Null);
            Ok(f.into())
        }
        _ => Err(format!(
            "argument to `first` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn last(args: Vec<&Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. expected 1, got {}",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let l = a
                .elements
                .last()
                .cloned()
                .map(|r| (*r).clone())
                .unwrap_or(Object::Null);
            Ok(l.into())
        }
        _ => Err(format!(
            "argument to `last` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn rest(args: Vec<&Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments. expected 1, got {}",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let elements = a.elements.clone().into_iter().skip(1).collect();
            Ok(Object::Array(ArrayObj { elements }).into())
        }
        _ => Err(format!(
            "argument to `rest` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn push(args: Vec<&Object>) -> Result<Object, String> {
    if args.len() != 2 {
        return Err(format!(
            "wrong number of arguments. expected 2, got {}",
            args.len()
        ));
    }

    match &*args[0] {
        Object::Array(a) => {
            let mut elements = a.elements.clone();
            elements.push(args[1].clone().into());
            Ok(Object::Array(ArrayObj { elements }).into())
        }
        _ => Err(format!(
            "argument to `push` not supported, got {}",
            args[0].kind()
        )),
    }
}

fn puts(args: Vec<&Object>) -> Result<Object, String> {
    for arg in args {
        println!("{}", arg);
    }
    Ok(Object::Null.into())
}
