#![allow(dead_code)]

use std::rc::Rc;

use crate::{
    compiler::{Bytecode, Bytes, OpCode},
    eval::Object,
};

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 0xFFFF;

pub struct Vm {
    instructions: Bytes,
    constants: Vec<Object>,

    globals: Vec<Object>,
    stack: Box<[Object; STACK_SIZE]>,
    /// Points to next value. Top of stack is at sp - 1
    sp: usize,
}

impl Vm {
    pub fn new(b: Bytecode) -> Self {
        Vm {
            instructions: b.instructions,
            constants: b.constants,

            globals: vec![Object::Null; GLOBALS_SIZE],
            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,
        }
    }

    pub fn new_with_state(b: Bytecode, globals: Vec<Object>) -> Self {
        assert_eq!(globals.len(), GLOBALS_SIZE);

        Self {
            instructions: b.instructions,
            constants: b.constants,
            globals,
            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,
        }
    }

    pub fn state(&self) -> Vec<Object> {
        self.globals.clone()
    }

    pub fn run(&mut self) -> RunResult {
        let mut ip = 0;

        while ip < self.instructions.len() {
            let op: OpCode = self.instructions.read(ip);
            ip += 1;

            match op {
                OpCode::Constant => {
                    let const_idx: u16 = self.instructions.read(ip);
                    ip += 2;
                    self.push(self.constants[const_idx as usize].clone())?;
                }
                OpCode::Add
                | OpCode::Sub
                | OpCode::Mul
                | OpCode::Div
                | OpCode::Greater
                | OpCode::Eq
                | OpCode::NotEq => self.execute_bin_op(op)?,
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::True => self.push(Object::Bool(true))?,
                OpCode::False => self.push(Object::Bool(false))?,
                OpCode::Minus => {
                    let right = self.pop();
                    match right {
                        Object::Integer(right) => self.push(Object::Integer(-right))?,
                        _ => return Err(format!("unknown operator: -{}", right.kind())),
                    }
                }
                OpCode::Bang => {
                    let right = self.pop();
                    self.push(Object::Bool(!right.is_truthy()))?
                }
                OpCode::JumpNotTrue => {
                    let jmp_to: u16 = self.instructions.read(ip);
                    ip += 2;

                    let cond = self.pop();
                    if !cond.is_truthy() {
                        ip = jmp_to as usize;
                    }
                }
                OpCode::Jump => {
                    let jmp_to: u16 = self.instructions.read(ip);
                    ip = jmp_to as usize;
                }
                OpCode::SetGlobal => {
                    let idx: u16 = self.instructions.read(ip);
                    ip += 2;

                    self.globals[idx as usize] = self.pop();
                }
                OpCode::GetGlobal => {
                    let idx: u16 = self.instructions.read(ip);
                    ip += 2;

                    self.push(self.globals[idx as usize].clone())?
                }
                OpCode::Array => {
                    let len: u16 = self.instructions.read(ip);
                    let len = len as usize;
                    ip += 2;

                    let mut arr = vec![Object::Null.into(); len];
                    for i in (0..len).rev() {
                        arr[i] = Rc::new(self.pop());
                    }

                    self.push(Object::Array(crate::eval::ArrayObj { elements: arr }))?
                }
                _ => todo!(),
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

    pub fn last_popped(&self) -> &Object {
        &self.stack[self.sp]
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

    fn execute_bin_op(&mut self, op: OpCode) -> RunResult {
        let right = self.pop();
        let left = self.pop();

        match (&left, &right) {
            (Object::Integer(left), Object::Integer(right)) => match op {
                OpCode::Add => self.push(Object::Integer(left + right)),
                OpCode::Sub => self.push(Object::Integer(left - right)),
                OpCode::Mul => self.push(Object::Integer(left * right)),
                OpCode::Div => self.push(Object::Integer(left / right)),
                OpCode::Eq => self.push(Object::Bool(left == right)),
                OpCode::NotEq => self.push(Object::Bool(left != right)),
                OpCode::Greater => self.push(Object::Bool(left > right)),
                _ => unreachable!(),
            },
            (Object::String(l), Object::String(r)) => match op {
                OpCode::Add => self.push(Object::String(l.to_owned() + r)),
                _ => Err(format!(
                    "unknown operation: {} {} {}",
                    left.kind(),
                    op,
                    right.kind()
                )),
            },
            _ if left.kind() == right.kind() => match op {
                OpCode::Eq => self.push(Object::Bool(left == right)),
                OpCode::NotEq => self.push(Object::Bool(left != right)),
                _ => Err(format!(
                    "unknown operation: {} {} {}",
                    left.kind(),
                    op,
                    right.kind()
                )),
            },
            _ => Err(format!(
                "unknown operation: {} {} {}",
                left.kind(),
                op,
                right.kind()
            )),
        }
    }
}

pub type RunResult = Result<(), String>;

#[cfg(test)]
mod test;
