mod parser;
pub use parser::Parser;

type Ident = String;

pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
enum Statement {
    Let(LetStmt),
}

#[derive(Debug)]
enum Expression {
    Ident(Ident),
    Todo,
}

#[derive(Debug)]
struct LetStmt {
    ident: Ident,
    expr: Expression,
}

#[cfg(test)]
mod test;
