#![allow(dead_code)]

use crate::{
    ast::{Expression, Program, Statement},
    lexer::TokenType,
};
pub use env::Environment;
use object::Object;

mod env;
mod object;

pub fn eval_program(prog: Program, env: &mut Environment) -> EvalResult {
    let mut res = Object::Null;
    for stmt in prog.statements {
        res = eval_stmt(stmt, env)?;

        if let Object::Return(val) = res {
            return Ok(*val);
        }
    }
    Ok(res)
}

pub fn eval_stmt(stmt: Statement, env: &mut Environment) -> EvalResult {
    match stmt {
        Statement::Let(l) => {
            let val = eval_expr(l.expr, env)?;
            env.set(l.ident, val);
            Ok(Object::Null)
        }
        Statement::Return(r) => {
            let val = eval_expr(r.expr, env)?;
            Ok(Object::Return(Box::new(val)))
        }
        Statement::Expression(e) => eval_expr(e.expr, env),
    }
}

pub fn eval_expr(e: Expression, env: &mut Environment) -> EvalResult {
    match e {
        Expression::Ident(i) => env
            .get(&i)
            .cloned()
            .ok_or(format!("identifier not found: {}", i)),
        Expression::Number(x) => Ok(Object::Integer(x)),
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
        Expression::Func(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

pub fn eval_block(block: Vec<Statement>, env: &mut Environment) -> EvalResult {
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

type EvalResult = Result<Object, String>;

#[cfg(test)]
mod test;
