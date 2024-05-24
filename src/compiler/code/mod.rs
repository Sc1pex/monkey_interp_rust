#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant,
}

impl OpCode {
    pub fn def(&self) -> Definition {
        match self {
            OpCode::Constant => Definition::new("OpConstant", &[2]),
        }
    }
}

pub struct Definition {
    name: &'static str,
    operands: &'static [usize],
    len: usize,
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
    pub fn make(self) -> Box<[u8]> {
        let def = self.op.def();

        let mut instr = vec![0; def.len];
        let mut off = 1;

        instr[0] = self.op as u8;
        for (width, operand) in def.operands.iter().zip(self.operands) {
            match width {
                2 => {
                    let operand = *operand as u16;
                    instr[off..(off + 2)].copy_from_slice(&operand.to_be_bytes());
                }
                _ => unimplemented!(),
            }
            off += width;
        }

        instr.into()
    }
}

#[cfg(test)]
mod test;
