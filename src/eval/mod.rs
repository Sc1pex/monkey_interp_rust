#![allow(dead_code)]

use crate::ast::{Expression, Program, Statement};
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
        Expression::Prefix(_) => todo!(),
        Expression::Infix(_) => todo!(),
        Expression::Bool(b) => Object::Bool(b),
        Expression::If(_) => todo!(),
        Expression::Func(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

#[cfg(test)]
mod test;
