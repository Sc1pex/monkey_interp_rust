#![allow(dead_code)]

mod parser;
use std::fmt::Display;

pub use parser::Parser;

type Ident = String;

pub struct Program {
    statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Statement {
    Let(LetStmt),
    Return(ReturnStmt),
    Expression(ExpressionStmt),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(s) => write!(f, "{}", s),
            Statement::Return(s) => write!(f, "{}", s),
            Statement::Expression(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
struct LetStmt {
    ident: Ident,
    expr: Expression,
}
#[derive(Debug)]
struct ReturnStmt {
    expr: Expression,
}
#[derive(Debug)]
struct ExpressionStmt {
    expr: Expression,
}

impl Display for LetStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.ident, self.expr)
    }
}
impl Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.expr)
    }
}
impl Display for ExpressionStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug)]
enum Expression {
    Ident(Ident),
    Number(i64),
    Todo,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Ident(i) => write!(f, "{}", i),
            Expression::Number(x) => write!(f, "{}", x),
            Expression::Todo => write!(f, "TODO"),
        }
    }
}

#[cfg(test)]
mod test;
