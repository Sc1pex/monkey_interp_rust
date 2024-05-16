mod parser;
pub use parser::Parser;

type Ident = String;

pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
enum Statement {
    Let(LetStmt),
    Return(ReturnStmt),
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
enum Expression {
    Ident(Ident),
    Todo,
}

#[cfg(test)]
mod test;
