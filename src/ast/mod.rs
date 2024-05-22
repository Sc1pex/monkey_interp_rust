mod parser;
use crate::lexer::TokenType;
use std::fmt::Display;

pub use parser::Parser;

pub type Ident = String;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Let(LetStmt),
    Return(ReturnStmt),
    Expression(Expression),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetStmt {
    pub ident: Ident,
    pub expr: Expression,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmt {
    pub expr: Expression,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Ident(Ident),
    Number(i64),
    String(String),
    Prefix(PrefixExpr),
    Infix(InfixExpr),
    Bool(bool),
    If(IfExpr),
    Func(FuncExpr),
    Call(CallExpr),
    Array(ArrayExpr),
    Index(IndexExpr),
    Hash(HashExpr),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Ident(i) => write!(f, "{}", i),
            Expression::Number(x) => write!(f, "{}", x),
            Expression::String(s) => write!(f, "{}", s),
            Expression::Prefix(p) => write!(f, "{}", p),
            Expression::Infix(p) => write!(f, "{}", p),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::If(i) => write!(f, "{}", i),
            Expression::Func(i) => write!(f, "{}", i),
            Expression::Call(i) => write!(f, "{}", i),
            Expression::Array(i) => write!(f, "{}", i),
            Expression::Index(i) => write!(f, "{}", i),
            Expression::Hash(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrefixExpr {
    pub operator: TokenType,
    pub right: Box<Expression>,
}

impl Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InfixExpr {
    pub left: Box<Expression>,
    pub operator: TokenType,
    pub right: Box<Expression>,
}

impl Display for InfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfExpr {
    pub condition: Box<Expression>,
    pub if_branch: Vec<Statement>,
    pub else_branch: Option<Vec<Statement>>,
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "if ({}) {{", self.condition)?;
        for s in &self.if_branch {
            writeln!(f, "  {}", s)?;
        }
        write!(f, "}}")?;
        if let Some(else_branch) = &self.else_branch {
            writeln!(f, " else {{")?;
            for s in else_branch {
                writeln!(f, "  {}", s)?;
            }
            write!(f, "}}")?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncExpr {
    pub params: Vec<Ident>,
    pub body: Vec<Statement>,
}

impl Display for FuncExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn (")?;
        for p in &self.params {
            write!(f, "{}", p)?;
        }
        writeln!(f, ") {{")?;
        for s in &self.body {
            writeln!(f, "  {}", s)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallExpr {
    /// `Expression::Func` or `Expression::Ident`
    pub func: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.func)?;

        write!(f, "(")?;
        for (idx, s) in self.arguments.iter().enumerate() {
            if idx != self.arguments.len() - 1 {
                write!(f, "{}, ", s)?;
            } else {
                write!(f, "{}", s)?;
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayExpr {
    pub elements: Vec<Expression>,
}

impl Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (idx, s) in self.elements.iter().enumerate() {
            if idx != self.elements.len() - 1 {
                write!(f, "{}, ", s)?;
            } else {
                write!(f, "{}", s)?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndexExpr {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HashExpr {
    pub pairs: Vec<(Expression, Expression)>,
}

impl Display for HashExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (idx, (k, v)) in self.pairs.iter().enumerate() {
            if idx != self.pairs.len() - 1 {
                write!(f, "{}: {}, ", k, v)?;
            } else {
                write!(f, "{}: {}", k, v)?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod test;
