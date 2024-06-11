#![allow(dead_code)]

use crate::{ast::*, eval::Object, lexer::TokenType};

pub use code::Bytes;
pub use instructions::{Instruction, OpCode};
pub use symbol_table::SymbolTable;

mod code;
mod instructions;
mod symbol_table;

pub struct Compiler {
    instructions: Bytes,
    constants: Vec<Object>,

    last: Option<Emmited>,
    prev: Option<Emmited>,

    symbol_table: SymbolTable,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            instructions: Bytes::default(),
            constants: vec![Object::Null],

            last: None,
            prev: None,

            symbol_table: SymbolTable::default(),
        }
    }
}

#[derive(Clone, Copy)]
struct Emmited {
    opcode: OpCode,
    pos: usize,
}

#[derive(Default)]
pub struct Bytecode {
    pub instructions: Bytes,
    pub constants: Vec<Object>,
}

impl Compiler {
    pub fn new_with_state(symbol_table: SymbolTable, constants: Vec<Object>) -> Self {
        Self {
            symbol_table,
            constants,
            ..Default::default()
        }
    }

    pub fn state(&self) -> (SymbolTable, Vec<Object>) {
        (self.symbol_table.clone(), self.constants.clone())
    }

    pub fn compile(&mut self, program: Program) -> CompileResult {
        self.compile_block(program.statements)
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
            Statement::Let(l) => {
                self.compile_expr(l.expr)?;
                let sym = self.symbol_table.define(&l.ident);
                self.emit(Instruction::new(OpCode::SetGlobal, &[sym.index as u32]));
                Ok(())
            }
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
            Expression::Ident(i) => {
                let sym = self
                    .symbol_table
                    .resolve(&i)
                    .ok_or(format!("undefined symbol: {}", i))?;
                self.emit(Instruction::new(OpCode::GetGlobal, &[sym.index as u32]));
            }
            Expression::Number(x) => {
                let obj = Object::Integer(x);
                let idx = self.add_constant(obj) as u32;
                self.emit(Instruction::new(OpCode::Constant, &[idx]));
            }
            Expression::String(s) => {
                let obj = Object::String(s);
                let idx = self.add_constant(obj) as u32;
                self.emit(Instruction::new(OpCode::Constant, &[idx]));
            }
            Expression::Prefix(p) => self.compile_prefix(p)?,
            Expression::Infix(i) => self.compile_infix(i)?,
            Expression::Bool(b) => {
                match b {
                    true => self.emit(Instruction::new(OpCode::True, &[])),
                    false => self.emit(Instruction::new(OpCode::False, &[])),
                };
            }
            Expression::If(IfExpr {
                condition,
                if_branch,
                else_branch,
            }) => {
                self.compile_expr(*condition)?;
                let jmp_if = self.emit(Instruction::new(OpCode::JumpNotTrue, &[9999]));

                self.compile_block(if_branch)?;
                if self.last_is(OpCode::Pop) {
                    self.remove_last();
                }
                let jmp_else = self.emit(Instruction::new(OpCode::Jump, &[9999]));

                self.patch(
                    jmp_if,
                    Instruction::new(OpCode::JumpNotTrue, &[self.instructions.len() as u32]),
                );

                if let Some(else_branch) = else_branch {
                    self.compile_block(else_branch)?;
                    if self.last_is(OpCode::Pop) {
                        self.remove_last();
                    }
                } else {
                    self.emit(Instruction::null());
                }
                self.patch(
                    jmp_else,
                    Instruction::new(OpCode::Jump, &[self.instructions.len() as u32]),
                )
            }
            Expression::Func(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::Array(a) => {
                let len = a.elements.len();
                for e in a.elements {
                    self.compile_expr(e)?;
                }
                self.emit(Instruction::new(OpCode::Array, &[len as u32]));
            }
            Expression::Index(_) => todo!(),
            Expression::Hash(h) => {
                let len = h.pairs.len();
                for (k, v) in h.pairs {
                    self.compile_expr(k)?;
                    self.compile_expr(v)?;
                }
                self.emit(Instruction::new(OpCode::Hash, &[len as u32]));
            }
        }

        Ok(())
    }
}

impl Compiler {
    fn compile_block(&mut self, block: Vec<Statement>) -> CompileResult {
        for stmt in block {
            self.compile_stmt(stmt)?;
        }
        Ok(())
    }

    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, i: Instruction) -> usize {
        let pos = self.instructions.len();

        self.prev = self.last;
        self.last = Some(Emmited { opcode: i.op, pos });

        self.instructions.push(i);
        pos
    }

    fn compile_prefix(&mut self, p: PrefixExpr) -> CompileResult {
        self.compile_expr(*p.right)?;
        match p.operator {
            TokenType::Minus => self.emit(Instruction::new(OpCode::Minus, &[])),
            TokenType::Bang => self.emit(Instruction::new(OpCode::Bang, &[])),
            _ => unreachable!(),
        };

        Ok(())
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

    fn last_is(&self, op: OpCode) -> bool {
        self.last.map(|l| l.opcode == op).unwrap_or(false)
    }

    fn remove_last(&mut self) {
        let last = self.last.expect("No instruction to remove");
        self.instructions.remove(last.pos);

        self.last = self.prev;
    }

    fn patch(&mut self, pos: usize, i: Instruction) {
        self.instructions.patch(pos, i);
    }
}

type CompileResult = Result<(), String>;

#[cfg(test)]
mod test;
