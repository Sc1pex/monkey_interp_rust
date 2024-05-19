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

        if let Object::Return(val) = res {
            return *val;
        }
    }
    res
}

pub fn eval_stmt(stmt: Statement) -> Object {
    match stmt {
        Statement::Let(_) => todo!(),
        Statement::Return(r) => {
            let val = eval_expr(r.expr);
            Object::Return(Box::new(val))
        }
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
        Expression::If(i) => {
            let cond = eval_expr(*i.condition);

            if cond.is_truthy() {
                eval_block(i.if_branch)
            } else {
                match i.else_branch {
                    Some(b) => eval_block(b),
                    None => Object::Null,
                }
            }
        }
        Expression::Func(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

pub fn eval_block(block: Vec<Statement>) -> Object {
    let mut res = Object::Null;
    for stmt in block {
        res = eval_stmt(stmt);

        if matches!(res, Object::Return(_)) {
            return res;
        }
    }
    res
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
    Object::Bool(!value.is_truthy())
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
