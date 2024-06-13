#![allow(dead_code)]

use std::rc::Rc;

use crate::{
    builtin::Builtin,
    compiler::{Bytecode, Bytes, OpCode},
    eval::{CompiledFuncObj, Object},
};

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 0xFFFF;

struct Frame {
    func: Rc<CompiledFuncObj>,
    ip: usize,
    sp: usize,
}

pub struct Vm {
    constants: Vec<Object>,

    globals: Vec<Object>,
    stack: Box<[Object; STACK_SIZE]>,
    /// Points to next value. Top of stack is at sp - 1
    sp: usize,

    frames: Vec<Frame>,
}

impl Vm {
    pub fn new(b: Bytecode) -> Self {
        let frame = Frame {
            func: Rc::new(CompiledFuncObj {
                instructions: b.instructions,
                locals: 0,
                params: 0,
            }),
            ip: 0,
            sp: 0,
        };
        Vm {
            // instructions: b.instructions,
            constants: b.constants,

            globals: vec![Object::Null; GLOBALS_SIZE],
            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,

            frames: vec![frame],
        }
    }

    pub fn new_with_state(b: Bytecode, globals: Vec<Object>) -> Self {
        assert_eq!(globals.len(), GLOBALS_SIZE);

        let frame = Frame {
            func: Rc::new(CompiledFuncObj {
                instructions: b.instructions,
                locals: 0,
                params: 0,
            }),
            ip: 0,
            sp: 0,
        };

        Self {
            constants: b.constants,
            globals,
            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,

            frames: vec![frame],
        }
    }

    pub fn state(&self) -> Vec<Object> {
        self.globals.clone()
    }

    pub fn run(&mut self) -> RunResult {
        while self.ip() < self.instructions().len() {
            let op: OpCode = self.instructions().read(self.ip());
            *self.ip_mut() += 1;

            match op {
                OpCode::Constant => {
                    let const_idx: u16 = self.instructions().read(self.ip());
                    *self.ip_mut() += 2;
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
                    let jmp_to: u16 = self.instructions().read(self.ip());
                    *self.ip_mut() += 2;

                    let cond = self.pop();
                    if !cond.is_truthy() {
                        *self.ip_mut() = jmp_to as usize;
                    }
                }
                OpCode::Jump => {
                    let jmp_to: u16 = self.instructions().read(self.ip());
                    *self.ip_mut() = jmp_to as usize;
                }
                OpCode::SetGlobal => {
                    let idx: u16 = self.instructions().read(self.ip());
                    *self.ip_mut() += 2;

                    self.globals[idx as usize] = self.pop();
                }
                OpCode::GetGlobal => {
                    let idx: u16 = self.instructions().read(self.ip());
                    *self.ip_mut() += 2;

                    self.push(self.globals[idx as usize].clone())?
                }
                OpCode::Array => {
                    let len: u16 = self.instructions().read(self.ip());
                    let len = len as usize;
                    *self.ip_mut() += 2;

                    let mut arr = vec![Object::Null.into(); len];
                    for i in (0..len).rev() {
                        arr[i] = Rc::new(self.pop());
                    }

                    self.push(Object::Array(crate::eval::ArrayObj { elements: arr }))?
                }
                OpCode::Hash => {
                    let len: u16 = self.instructions().read(self.ip());
                    let len = len as usize;
                    *self.ip_mut() += 2;

                    let mut pairs = vec![];
                    for _ in 0..len {
                        let v = Rc::new(self.pop());
                        let k = Rc::new(self.pop());
                        pairs.push((k, v));
                    }
                    self.push(Object::Hash(crate::eval::HashObj {
                        map: pairs.into_iter().collect(),
                    }))?
                }
                OpCode::Index => {
                    let index = self.pop();
                    let left = self.pop();
                    self.execute_index_op(left, index)?;
                }
                OpCode::Call => {
                    let args: u8 = self.instructions().read(self.ip());
                    *self.ip_mut() += 1;

                    self.execute_call(args)?;
                }
                OpCode::ReturnValue => {
                    let val = self.pop();
                    self.sp = self.pop_frame().sp - 1;
                    self.push(val)?;
                }
                OpCode::Return => {
                    self.sp = self.pop_frame().sp - 1;
                    self.push(Object::Null)?;
                }
                OpCode::SetLocal => {
                    let idx: u8 = self.instructions().read(self.ip());
                    *self.ip_mut() += 1;

                    let val = self.pop();
                    self.stack[self.frame().sp + idx as usize] = val;
                }
                OpCode::GetLocal => {
                    let idx: u8 = self.instructions().read(self.ip());
                    *self.ip_mut() += 1;

                    let val = self.stack[self.frame().sp + idx as usize].clone();
                    self.push(val)?;
                }
                OpCode::GetBuiltin => {
                    let idx: u8 = self.instructions().read(self.ip());
                    *self.ip_mut() += 1;

                    let builtin =
                        Builtin::from_u8(idx).ok_or(&format!("unknown builtin {}", idx))?;
                    self.push(Object::Builtin(builtin))?;
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

    fn execute_call(&mut self, args: u8) -> RunResult {
        match self
            .stack
            .get(self.sp - 1 - args as usize)
            .expect("nothing to call")
        {
            Object::CompiledFunc(c) => self.call_func(args, c.clone()),
            Object::Builtin(b) => self.call_builtin(args, *b),
            o => return Err(format!("cannot call object {:?}", o)),
        }
    }

    fn call_builtin(&mut self, args: u8, b: Builtin) -> RunResult {
        let args: Vec<Object> = self.stack[(self.sp - args as usize)..self.sp]
            .into_iter()
            .map(|x| x.clone())
            .collect();
        let a: Vec<&Object> = args.iter().collect();

        let o: Object = b.call(a)?;
        self.push(o)
    }

    fn call_func(&mut self, args: u8, func: Rc<CompiledFuncObj>) -> RunResult {
        if args as usize != func.params {
            return Err(format!(
                "wrong number of arguments. expected {}, got {}",
                func.params, args
            ));
        }
        let locals = func.locals;
        self.push_frame(Frame {
            func,
            ip: 0,
            sp: self.sp - args as usize,
        });
        self.sp += locals;
        Ok(())
    }

    fn execute_index_op(&mut self, left: Object, index: Object) -> RunResult {
        match (&left, &index) {
            (Object::Array(a), Object::Integer(i)) => {
                let el = a
                    .elements
                    .get(*i as usize)
                    .map(|i| Rc::unwrap_or_clone(i.clone()))
                    .unwrap_or(Object::Null);
                self.push(el)
            }
            (Object::Hash(h), _) => {
                let el = h
                    .map
                    .get(&index)
                    .map(|i| Rc::unwrap_or_clone(i.clone()))
                    .unwrap_or(Object::Null);
                self.push(el)
            }
            _ => Err(format!(
                "index operator not supported: {} {}",
                left.kind(),
                index.kind()
            )),
        }
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

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Frame {
        assert!(self.frames.len() > 1, "Cannot leave out of main frame");
        self.frames.pop().unwrap()
    }

    fn instructions(&self) -> &Bytes {
        &self.frame().func.instructions
    }

    fn ip(&self) -> usize {
        self.frame().ip
    }

    fn ip_mut(&mut self) -> &mut usize {
        &mut self.frame_mut().ip
    }

    fn frame(&self) -> &Frame {
        self.frames
            .last()
            .expect("There should always exist at least one frame")
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.frames
            .last_mut()
            .expect("There should always exist at least one frame")
    }
}

pub type RunResult = Result<(), String>;

#[cfg(test)]
mod test;
