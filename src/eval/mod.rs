#![allow(dead_code)]

use crate::{
    ast::{Expression, Ident, Program, Statement},
    lexer::TokenType,
};
use builtin::Builtin;
pub use env::Environment;
use object::*;
use std::{cell::RefCell, rc::Rc};

mod builtin;
mod env;
mod object;

pub fn eval_program(prog: Program, env: &Rc<RefCell<Environment>>) -> EvalResult {
    let mut res = Object::Null;
    for stmt in prog.statements {
        res = eval_stmt(stmt, env)?;

        if let Object::Return(val) = res {
            return Ok(*val);
        }
    }
    Ok(res)
}

pub fn eval_stmt(stmt: Statement, env: &Rc<RefCell<Environment>>) -> EvalResult {
    match stmt {
        Statement::Let(l) => {
            let val = eval_expr(l.expr, env)?;
            env.borrow_mut().set(l.ident, val);
            Ok(Object::Null)
        }
        Statement::Return(r) => {
            let val = eval_expr(r.expr, env)?;
            Ok(Object::Return(Box::new(val)))
        }
        Statement::Expression(e) => eval_expr(e.expr, env),
    }
}

pub fn eval_expr(e: Expression, env: &Rc<RefCell<Environment>>) -> EvalResult {
    match e {
        Expression::Ident(i) => eval_ident(i, env),
        Expression::Number(x) => Ok(Object::Integer(x)),
        Expression::String(s) => Ok(Object::String(s)),
        Expression::Prefix(p) => {
            let right = eval_expr(*p.right, env)?;
            eval_prefix(p.operator, right)
        }
        Expression::Infix(i) => {
            let left = eval_expr(*i.left, env)?;
            let right = eval_expr(*i.right, env)?;
            eval_infix(left, i.operator, right)
        }
        Expression::Bool(b) => Ok(Object::Bool(b)),
        Expression::If(i) => {
            let cond = eval_expr(*i.condition, env)?;

            if cond.is_truthy() {
                eval_block(i.if_branch, env)
            } else {
                match i.else_branch {
                    Some(b) => eval_block(b, env),
                    None => Ok(Object::Null),
                }
            }
        }
        Expression::Func(f) => Ok(Object::Func(FuncObj {
            expr: f,
            env: env.clone(),
        })),
        Expression::Call(c) => {
            let func = eval_expr(*c.func, env)?;
            let args = eval_exprs(c.arguments, env)?;

            apply_func(func, args)
        }
    }
}

fn eval_ident(ident: Ident, env: &Rc<RefCell<Environment>>) -> EvalResult {
    if let Some(r) = env.borrow().get(&ident) {
        Ok(r)
    } else if let Some(b) = Builtin::from_ident(&ident) {
        Ok(b)
    } else {
        Err(format!("identifier not found: {}", ident))
    }
}

fn eval_exprs(
    expr: Vec<Expression>,
    env: &Rc<RefCell<Environment>>,
) -> Result<Vec<Object>, String> {
    expr.into_iter().map(|e| eval_expr(e, env)).collect()
}

pub fn eval_block(block: Vec<Statement>, env: &Rc<RefCell<Environment>>) -> EvalResult {
    let mut res = Object::Null;
    for stmt in block {
        res = eval_stmt(stmt, env)?;

        if matches!(res, Object::Return(_)) {
            return Ok(res);
        }
    }
    Ok(res)
}

fn eval_prefix(op: TokenType, right: Object) -> EvalResult {
    match op {
        TokenType::Bang => eval_bang_op(right),
        TokenType::Minus => eval_minus_op(right),
        _ => unreachable!(),
    }
}

fn eval_infix(left: Object, op: TokenType, right: Object) -> EvalResult {
    match (left, op, right) {
        (Object::Integer(left), _, Object::Integer(right)) => {
            eval_integer_infix_op(left, op, right)
        }
        (Object::String(left), _, Object::String(right)) => eval_string_infix_op(left, op, right),
        (left, TokenType::Eq, right) => Ok(Object::Bool(left == right)),
        (left, TokenType::NotEq, right) => Ok(Object::Bool(left != right)),
        (left, op, right) if left.kind() != right.kind() => Err(format!(
            "type mismatch: {} {} {}",
            left.kind(),
            op,
            right.kind()
        )),
        (left, op, right) => Err(format!(
            "unknown operator: {} {} {}",
            left.kind(),
            op,
            right.kind()
        )),
    }
}

fn eval_bang_op(value: Object) -> EvalResult {
    Ok(Object::Bool(!value.is_truthy()))
}

fn eval_minus_op(value: Object) -> EvalResult {
    match value {
        Object::Integer(x) => Ok(Object::Integer(-x)),
        _ => Err(format!("unknown operator: -{}", value.kind())),
    }
}

fn eval_integer_infix_op(left: i64, op: TokenType, right: i64) -> EvalResult {
    match op {
        TokenType::Plus => Ok(Object::Integer(left + right)),
        TokenType::Minus => Ok(Object::Integer(left - right)),
        TokenType::Star => Ok(Object::Integer(left * right)),
        TokenType::Slash => Ok(Object::Integer(left / right)),

        TokenType::Lt => Ok(Object::Bool(left < right)),
        TokenType::Gt => Ok(Object::Bool(left > right)),
        TokenType::Eq => Ok(Object::Bool(left == right)),
        TokenType::NotEq => Ok(Object::Bool(left != right)),
        _ => unreachable!(),
    }
}

fn eval_string_infix_op(left: String, op: TokenType, right: String) -> EvalResult {
    match op {
        TokenType::Plus => Ok(Object::String(left + &right)),

        TokenType::Eq => Ok(Object::Bool(left == right)),
        TokenType::NotEq => Ok(Object::Bool(left != right)),

        _ => Err(format!("unknown operator: STRING {} STRING", op)),
    }
}

fn apply_func(func: Object, args: Vec<Object>) -> EvalResult {
    let func = match func {
        Object::Func(f) => f,
        Object::Builtin(b) => return b.call(args),
        _ => return Err(format!("not a function: {}", func.kind())),
    };

    let env = Rc::new(RefCell::new(Environment::new_enclosed(func.env)));
    if args.len() != func.expr.params.len() {
        return Err(format!(
            "function expects {} arguments but {} were given",
            func.expr.params.len(),
            args.len()
        ));
    }

    for (arg, param) in args.iter().zip(func.expr.params.into_iter()) {
        env.borrow_mut().set(param, arg.clone())
    }
    let res = eval_block(func.expr.body, &env)?;

    match res {
        Object::Return(r) => Ok(*r),
        _ => Ok(res),
    }
}

type EvalResult = Result<Object, String>;

#[cfg(test)]
mod test;
