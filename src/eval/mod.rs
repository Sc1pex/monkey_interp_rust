#![allow(dead_code)]

use crate::{
    ast::{Expression, Program, Statement},
    lexer::TokenType,
};
use object::Object;

mod object;

pub fn eval_program(prog: Program) -> Object {
    let mut res = Object::Null;
    for stmt in prog.statements {
        res = eval_stmt(stmt);
    }
    res
}

pub fn eval_stmt(stmt: Statement) -> Object {
    match stmt {
        Statement::Let(_) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Expression(e) => eval_expr(e.expr),
    }
}

pub fn eval_expr(e: Expression) -> Object {
    match e {
        Expression::Ident(_) => todo!(),
        Expression::Number(x) => Object::Integer(x),
        Expression::Prefix(p) => {
            let right = eval_expr(*p.right);
            eval_prefix(p.operator, right)
        }
        Expression::Infix(_) => todo!(),
        Expression::Bool(b) => Object::Bool(b),
        Expression::If(_) => todo!(),
        Expression::Func(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

fn eval_prefix(op: TokenType, right: Object) -> Object {
    match op {
        TokenType::Bang => eval_bang_op(right),
        TokenType::Minus => eval_minus_op(right),
        _ => unreachable!(),
    }
}

fn eval_bang_op(value: Object) -> Object {
    match value {
        Object::Integer(0) => Object::Bool(true),
        Object::Integer(_) => Object::Bool(false),
        Object::Bool(x) => Object::Bool(!x),
        Object::Null => Object::Bool(true),
    }
}

fn eval_minus_op(value: Object) -> Object {
    match value {
        Object::Integer(x) => Object::Integer(-x),
        _ => Object::Null,
    }
}

#[cfg(test)]
mod test;
