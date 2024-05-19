#![allow(dead_code)]

use crate::{
    ast::{Expression, Program, Statement},
    lexer::TokenType,
};
use object::Object;

mod object;

pub fn eval_program(prog: Program) -> EvalResult {
    let mut res = Object::Null;
    for stmt in prog.statements {
        res = eval_stmt(stmt)?;

        if let Object::Return(val) = res {
            return Ok(*val);
        }
    }
    Ok(res)
}

pub fn eval_stmt(stmt: Statement) -> EvalResult {
    match stmt {
        Statement::Let(_) => todo!(),
        Statement::Return(r) => {
            let val = eval_expr(r.expr)?;
            Ok(Object::Return(Box::new(val)))
        }
        Statement::Expression(e) => eval_expr(e.expr),
    }
}

pub fn eval_expr(e: Expression) -> EvalResult {
    match e {
        Expression::Ident(_) => todo!(),
        Expression::Number(x) => Ok(Object::Integer(x)),
        Expression::Prefix(p) => {
            let right = eval_expr(*p.right)?;
            eval_prefix(p.operator, right)
        }
        Expression::Infix(i) => {
            let left = eval_expr(*i.left)?;
            let right = eval_expr(*i.right)?;
            eval_infix(left, i.operator, right)
        }
        Expression::Bool(b) => Ok(Object::Bool(b)),
        Expression::If(i) => {
            let cond = eval_expr(*i.condition)?;

            if cond.is_truthy() {
                eval_block(i.if_branch)
            } else {
                match i.else_branch {
                    Some(b) => eval_block(b),
                    None => Ok(Object::Null),
                }
            }
        }
        Expression::Func(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

pub fn eval_block(block: Vec<Statement>) -> EvalResult {
    let mut res = Object::Null;
    for stmt in block {
        res = eval_stmt(stmt)?;

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
