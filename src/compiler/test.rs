use super::*;
use crate::{ast::Parser, lexer::Lexer};
use instructions::{Instruction, OpCode};

macro_rules! test {
    ($($case:expr),* $(,)?) => {
        test(&[$($case),*])
    };
}

#[test]
fn integer_math() {
    test!(
        (
            "1 + 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 - 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Sub, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 / 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Div, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 * 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Mul, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
    )
}

#[test]
fn bool_expressions() {
    test!(
        (
            "true",
            &[],
            &[
                Instruction::new(OpCode::True, &[]),
                Instruction::new(OpCode::Pop, &[])
            ]
        ),
        (
            "false",
            &[],
            &[
                Instruction::new(OpCode::False, &[]),
                Instruction::new(OpCode::Pop, &[])
            ]
        ),
        (
            "1 > 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Greater, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 < 2",
            &[Object::Integer(2), Object::Integer(1)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Greater, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 == 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Eq, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 != 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[0]),
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::NotEq, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
    )
}

fn test(cases: &[(&str, &[Object], &[Instruction])]) {
    for (input, consts, instrs) in cases {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().expect("Skill issue");

        let mut compiler = Compiler::default();
        compiler.compile(program).unwrap();
        let bytecode = compiler.bytecode();

        let expected_bytes = instrs.into_iter().fold(Bytes::default(), |mut acc, x| {
            acc.push(x);
            acc
        });

        assert!(
            bytecode.instructions == expected_bytes,
            "Wrong instructions. expected:\n{}got:\n{}",
            expected_bytes,
            bytecode.instructions,
        );

        assert_eq!(bytecode.constants, *consts);
    }
}
