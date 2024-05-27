#![allow(dead_code)]

use crate::{ast::*, eval::Object, lexer::TokenType};

pub use code::Bytes;
pub use instructions::{Instruction, OpCode};

mod code;
mod instructions;

#[derive(Default)]
pub struct Compiler {
    instructions: Bytes,
    constants: Vec<Object>,
}

#[derive(Default)]
pub struct Bytecode {
    pub instructions: Bytes,
    pub constants: Vec<Object>,
}

impl Compiler {
    pub fn compile(&mut self, program: Program) -> CompileResult {
        for stmt in program.statements {
            self.compile_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn bytecode(self) -> Bytecode {
        Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}

impl Compiler {
    fn compile_stmt(&mut self, stmt: Statement) -> CompileResult {
        match stmt {
            Statement::Let(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Expression(e) => {
                self.compile_expr(e)?;
                self.emit(Instruction::new(OpCode::Pop, &[]));
                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: Expression) -> CompileResult {
        match expr {
            Expression::Ident(_) => todo!(),
            Expression::Number(x) => {
                let obj = Object::Integer(x);
                let idx = self.add_constant(obj) as i32;
                self.emit(Instruction::new(OpCode::Constant, &[idx]));
            }
            Expression::String(_) => todo!(),
            Expression::Prefix(_) => todo!(),
            Expression::Infix(i) => self.compile_infix(i)?,
            Expression::Bool(b) => {
                match b {
                    true => self.emit(Instruction::new(OpCode::True, &[])),
                    false => self.emit(Instruction::new(OpCode::False, &[])),
                };
            }
            Expression::If(_) => todo!(),
            Expression::Func(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::Hash(_) => todo!(),
        }

        Ok(())
    }
}

impl Compiler {
    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, i: Instruction) -> usize {
        let pos = self.instructions.len();
        self.instructions.push(i);
        pos
    }

    fn compile_infix(&mut self, i: InfixExpr) -> CompileResult {
        match i.operator {
            TokenType::Lt => self.compile_infix_rev(i),
            _ => self.compile_infix_normal(i),
        }
    }

    fn compile_infix_normal(&mut self, i: InfixExpr) -> CompileResult {
        self.compile_expr(*i.left)?;
        self.compile_expr(*i.right)?;

        match i.operator {
            TokenType::Plus => self.emit(Instruction::new(OpCode::Add, &[])),
            TokenType::Minus => self.emit(Instruction::new(OpCode::Sub, &[])),
            TokenType::Star => self.emit(Instruction::new(OpCode::Mul, &[])),
            TokenType::Slash => self.emit(Instruction::new(OpCode::Div, &[])),
            TokenType::Gt => self.emit(Instruction::new(OpCode::Greater, &[])),
            TokenType::Eq => self.emit(Instruction::new(OpCode::Eq, &[])),
            TokenType::NotEq => self.emit(Instruction::new(OpCode::NotEq, &[])),
            _ => unreachable!(),
        };
        Ok(())
    }

    fn compile_infix_rev(&mut self, i: InfixExpr) -> CompileResult {
        self.compile_expr(*i.right)?;
        self.compile_expr(*i.left)?;

        match i.operator {
            TokenType::Lt => self.emit(Instruction::new(OpCode::Greater, &[])),
            _ => unreachable!(),
        };
        Ok(())
    }
}

type CompileResult = Result<(), String>;

#[cfg(test)]
mod test;
