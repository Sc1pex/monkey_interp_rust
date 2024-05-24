use super::*;

#[test]
fn make() {
    let instr = Instruction {
        op: OpCode::Constant,
        operands: &[65534],
    };
    let exp = [OpCode::Constant as u8, 255, 254].into();

    assert_eq!(instr.make(), exp);
}
