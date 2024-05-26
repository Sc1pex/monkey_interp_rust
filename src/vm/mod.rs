#![allow(dead_code)]

use crate::{
    compiler::{Bytecode, Bytes, OpCode},
    eval::Object,
};

const STACK_SIZE: usize = 2048;

pub struct Vm {
    instructions: Bytes,
    constants: Vec<Object>,

    stack: Box<[Object; STACK_SIZE]>,
    /// Points to next value. Top of stack is at sp - 1
    sp: usize,
}

impl Vm {
    pub fn new(b: Bytecode) -> Self {
        Vm {
            instructions: b.instructions,
            constants: b.constants,

            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,
        }
    }

    pub fn run(&mut self) -> RunResult {
        let mut ip = 0;

        while ip < self.instructions.len() {
            let op: OpCode = self.instructions.read(ip);
            ip += 1;

            match op {
                OpCode::Constant => {
                    let const_idx: u16 = self.instructions.read(ip);
                    self.push(self.constants[const_idx as usize].clone())?;
                    ip += 2;
                }
                OpCode::Add => {
                    let right = self.pop();
                    let left = self.pop();

                    match (&left, &right) {
                        (Object::Integer(left), Object::Integer(right)) => {
                            self.push(Object::Integer(left + right))?
                        }
                        _ => return Err(format!("unknown operation: {} + {}", left, right)),
                    }
                }
            }
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }
}

impl Vm {
    fn push(&mut self, obj: Object) -> RunResult {
        if self.sp >= STACK_SIZE {
            Err(format!("Stack overflow"))
        } else {
            self.stack[self.sp] = obj;
            self.sp += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Object {
        let obj = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        obj
    }
}

pub type RunResult = Result<(), String>;

#[cfg(test)]
mod test;
