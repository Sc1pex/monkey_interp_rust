use super::instructions::OpCode;
use std::fmt::Display;

#[derive(Default, Debug, PartialEq)]
pub struct Bytes {
    data: Vec<u8>,
}

impl Bytes {
    pub fn push<T: BytesWrite>(&mut self, val: T) {
        val.write(self);
    }

    pub fn read<T: BytesRead>(&self, start: usize) -> T {
        T::read(self, start)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = 0;

        while idx < self.data.len() {
            write!(f, "{:0>4} ", idx)?;
            let op: OpCode = self.data[idx].into();
            idx += 1;

            let def = op.def();
            write!(f, "{}", def.name)?;

            for w in def.operands {
                match w {
                    2 => {
                        let operand: [u8; 2] = self.data[idx..(idx + 2)]
                            .try_into()
                            .expect("u suck at math");
                        let operand = u16::from_be_bytes(operand);

                        write!(f, " {}", operand)?;
                    }
                    _ => unimplemented!(),
                }
                idx += w;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl PartialEq<&[u8]> for Bytes {
    fn eq(&self, other: &&[u8]) -> bool {
        self.data == *other
    }
}

pub trait BytesWrite {
    fn write(&self, b: &mut Bytes);
}

pub trait BytesRead {
    fn read(b: &Bytes, idx: usize) -> Self;
}

impl BytesWrite for OpCode {
    fn write(&self, b: &mut Bytes) {
        b.data.push(*self as u8);
    }
}

impl BytesRead for OpCode {
    fn read(b: &Bytes, idx: usize) -> Self {
        b.data[idx].into()
    }
}

macro_rules! impl_bytes_traits {
    ($(($ty:ty, $size:expr)),*) => {$(
        impl BytesWrite for $ty {
            fn write(&self, b: &mut Bytes) {
                b.data.extend_from_slice(&self.to_be_bytes());
            }
        }

        impl BytesRead for $ty {
            fn read(b: &Bytes, idx: usize) -> Self {
                let d = &b.data[idx..(idx + $size)];
                Self::from_be_bytes(d.try_into().unwrap())
            }
        }
    )*};
}
impl_bytes_traits!((u8, 1), (i8, 1), (u16, 2), (i16, 2), (u32, 4), (i32, 4));

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::instructions::Instruction;

    #[test]
    fn bytes_string() {
        let instrs = [
            Instruction::new(OpCode::Add, &[]),
            Instruction::new(OpCode::Constant, &[2]),
            Instruction::new(OpCode::Constant, &[65534]),
        ];

        let bytes = instrs.into_iter().fold(Bytes::default(), |mut acc, x| {
            acc.push(x);
            acc
        });

        let expected = r#"0000 OpAdd
0001 OpConstant 2
0004 OpConstant 65534
"#;

        assert_eq!(expected, bytes.to_string());
    }
}
