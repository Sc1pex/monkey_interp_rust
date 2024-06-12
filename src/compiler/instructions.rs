use super::code::{Bytes, BytesWrite};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Constant,

    Add,
    Pop,
    Sub,
    Mul,
    Div,
    True,
    False,
    Eq,
    NotEq,
    Greater,
    Bang,
    Minus,

    Jump,
    JumpNotTrue,

    SetGlobal,
    GetGlobal,
    SetLocal,
    GetLocal,

    Array,
    Hash,
    Index,

    Call,
    ReturnValue,
    Return,
}

impl OpCode {
    pub fn def(&self) -> Definition {
        match self {
            OpCode::Constant => Definition::new("OpConstant", &[2]),

            OpCode::Add => Definition::new("OpAdd", &[]),
            OpCode::Pop => Definition::new("OpPop", &[]),
            OpCode::Sub => Definition::new("OpSub", &[]),
            OpCode::Mul => Definition::new("OpMul", &[]),
            OpCode::Div => Definition::new("OpDiv", &[]),
            OpCode::True => Definition::new("OpTrue", &[]),
            OpCode::False => Definition::new("OpFalse", &[]),
            OpCode::Eq => Definition::new("OpEq", &[]),
            OpCode::NotEq => Definition::new("OpNotEq", &[]),
            OpCode::Greater => Definition::new("OpGreater", &[]),
            OpCode::Bang => Definition::new("OpBang", &[]),
            OpCode::Minus => Definition::new("OpMinus", &[]),

            OpCode::Jump => Definition::new("OpJump", &[2]),
            OpCode::JumpNotTrue => Definition::new("OpJumpNotTrue", &[2]),

            OpCode::SetGlobal => Definition::new("OpSetGlobal", &[2]),
            OpCode::GetGlobal => Definition::new("OpGetGlobal", &[2]),
            OpCode::SetLocal => Definition::new("OpSetLocal", &[1]),
            OpCode::GetLocal => Definition::new("OpGetLocal", &[1]),

            OpCode::Array => Definition::new("OpArray", &[2]),
            OpCode::Hash => Definition::new("OpHash", &[2]),
            OpCode::Index => Definition::new("OpIndex", &[]),

            OpCode::Call => Definition::new("OpCall", &[]),
            OpCode::ReturnValue => Definition::new("OpReturnValue", &[]),
            OpCode::Return => Definition::new("OpReturn", &[]),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.def().name)
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        if value > std::mem::variant_count::<Self>() as u8 {
            panic!("Invalid opcode: {}", value);
        } else {
            unsafe { std::mem::transmute(value) }
        }
    }
}

pub struct Definition {
    pub name: &'static str,
    pub operands: &'static [usize],
    pub len: usize,
}

impl Definition {
    pub fn new(name: &'static str, operands: &'static [usize]) -> Self {
        Self {
            name,
            operands,
            len: 1 + operands.iter().sum::<usize>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    pub op: OpCode,
    pub operands: Box<[u32]>,
}

impl Instruction {
    pub fn new(op: OpCode, operands: &[u32]) -> Self {
        Self {
            op,
            operands: operands.into(),
        }
    }

    pub fn make(self) -> Bytes {
        let mut b = Bytes::default();
        b.push(self);
        b
    }

    // Compiler has a null object as first constant
    pub fn null() -> Self {
        Self {
            op: OpCode::Constant,
            operands: Box::new([0]),
        }
    }
}

impl BytesWrite for Instruction {
    fn write(&self, b: &mut Bytes) {
        (&self).write(b)
    }
}
impl BytesWrite for &Instruction {
    fn write(&self, b: &mut Bytes) {
        let def = self.op.def();

        b.push(self.op);
        for (width, operand) in def.operands.iter().zip(&self.operands) {
            match width {
                1 => {
                    let operand = *operand as u8;
                    b.push(operand);
                }
                2 => {
                    let operand = *operand as u16;
                    b.push(operand);
                }
                _ => unimplemented!("{}", width),
            }
        }
    }
}
