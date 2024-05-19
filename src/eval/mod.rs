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
        Expression::Infix(i) => {
            let left = eval_expr(*i.left);
            let right = eval_expr(*i.right);
            eval_infix(left, i.operator, right)
        }
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

fn eval_infix(left: Object, op: TokenType, right: Object) -> Object {
    match (left, op, right) {
        (Object::Integer(left), _, Object::Integer(right)) => {
            eval_integer_infix_op(left, op, right)
        }
        (left, TokenType::Eq, right) => Object::Bool(left == right),
        (left, TokenType::NotEq, right) => Object::Bool(left != right),
        _ => Object::Null,
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

fn eval_integer_infix_op(left: i64, op: TokenType, right: i64) -> Object {
    match op {
        TokenType::Plus => Object::Integer(left + right),
        TokenType::Minus => Object::Integer(left - right),
        TokenType::Star => Object::Integer(left * right),
        TokenType::Slash => Object::Integer(left / right),

        TokenType::Lt => Object::Bool(left < right),
        TokenType::Gt => Object::Bool(left > right),
        TokenType::Eq => Object::Bool(left == right),
        TokenType::NotEq => Object::Bool(left != right),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test;
