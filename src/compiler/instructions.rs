use super::code::{Bytes, BytesWrite};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant,
    Add,
    Pop,
    Sub,
    Mul,
    Div,
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
        }
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

pub struct Instruction<'a> {
    op: OpCode,
    operands: &'a [i32],
}

impl<'a> Instruction<'a> {
    pub fn new(op: OpCode, operands: &'a [i32]) -> Self {
        Self { op, operands }
    }

    pub fn make(self) -> Bytes {
        let mut b = Bytes::default();
        b.push(self);
        b
    }
}

impl<'a> BytesWrite for Instruction<'a> {
    fn write(&self, b: &mut Bytes) {
        (&self).write(b)
    }
}
impl<'a> BytesWrite for &Instruction<'a> {
    fn write(&self, b: &mut Bytes) {
        let def = self.op.def();

        b.push(self.op);
        for (width, operand) in def.operands.iter().zip(self.operands) {
            match width {
                2 => {
                    let operand = *operand as u16;
                    b.push(operand);
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make() {
        let tests: [(Instruction, &[u8]); 2] = [
            (
                Instruction {
                    op: OpCode::Constant,
                    operands: &[65534],
                },
                &[OpCode::Constant as u8, 255, 254],
            ),
            (
                Instruction {
                    op: OpCode::Add,
                    operands: &[],
                },
                &[OpCode::Add as u8],
            ),
        ];

        for (instr, exp) in tests {
            assert_eq!(instr.make(), &exp[..]);
        }
    }
}
