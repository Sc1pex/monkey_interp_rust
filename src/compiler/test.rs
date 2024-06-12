use super::*;
use crate::{ast::Parser, eval::CompiledFuncObj, lexer::Lexer};
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
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 - 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Sub, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 / 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Div, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 * 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Mul, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "-1",
            &[Object::Integer(1)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Minus, &[]),
                Instruction::new(OpCode::Pop, &[])
            ]
        )
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
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Greater, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 < 2",
            &[Object::Integer(2), Object::Integer(1)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Greater, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 == 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Eq, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "1 != 2",
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::NotEq, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "!false",
            &[],
            &[
                Instruction::new(OpCode::False, &[]),
                Instruction::new(OpCode::Bang, &[]),
                Instruction::new(OpCode::Pop, &[])
            ]
        ),
    )
}

#[test]
fn conditionals() {
    test!(
        (
            "if (true) { 10; }; 3333;",
            &[Object::Integer(10), Object::Integer(3333)],
            &[
                Instruction::new(OpCode::True, &[]),          // 0
                Instruction::new(OpCode::JumpNotTrue, &[10]), // 1
                Instruction::new(OpCode::Constant, &[1]),     // 4
                Instruction::new(OpCode::Jump, &[13]),        // 7
                Instruction::null(),                          // 10
                Instruction::new(OpCode::Pop, &[]),           // 13
                Instruction::new(OpCode::Constant, &[2]),     // 14
                Instruction::new(OpCode::Pop, &[]),           // 17
            ]
        ),
        (
            "if (true) { 10; } else { 20; }; 3333;",
            &[
                Object::Integer(10),
                Object::Integer(20),
                Object::Integer(3333)
            ],
            &[
                Instruction::new(OpCode::True, &[]),          // 0
                Instruction::new(OpCode::JumpNotTrue, &[10]), // 1
                Instruction::new(OpCode::Constant, &[1]),     // 4
                Instruction::new(OpCode::Jump, &[13]),        // 7
                Instruction::new(OpCode::Constant, &[2]),     // 10
                Instruction::new(OpCode::Pop, &[]),           // 13
                Instruction::new(OpCode::Constant, &[3]),     // 14
                Instruction::new(OpCode::Pop, &[]),           // 17
            ]
        ),
    )
}

#[test]
fn global_let() {
    test!(
        (
            r#" let one = 1;
            let two = 2;"#,
            &[Object::Integer(1), Object::Integer(2)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::SetGlobal, &[1]),
            ]
        ),
        (
            r#" let one = 1;
            one;"#,
            &[Object::Integer(1)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::GetGlobal, &[0]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            r#" let one = 1;
            let two = one;
            two;"#,
            &[Object::Integer(1)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::GetGlobal, &[0]),
                Instruction::new(OpCode::SetGlobal, &[1]),
                Instruction::new(OpCode::GetGlobal, &[1]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
    )
}

#[test]
fn strings() {
    test!(
        (
            r#""monkey""#,
            &[Object::String("monkey".into())],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            r#""mon" + "key""#,
            &[Object::String("mon".into()), Object::String("key".into())],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
    )
}

#[test]
fn arrays() {
    test!(
        (
            "[]",
            &[],
            &[
                Instruction::new(OpCode::Array, &[0]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            "[1, 2, 3]",
            &[Object::Integer(1), Object::Integer(2), Object::Integer(3)],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Array, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "[1 + 2, 3 - 4, 5 * 6]",
            &[
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
                Object::Integer(5),
                Object::Integer(6)
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Sub, &[]),
                Instruction::new(OpCode::Constant, &[5]),
                Instruction::new(OpCode::Constant, &[6]),
                Instruction::new(OpCode::Mul, &[]),
                Instruction::new(OpCode::Array, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        )
    )
}

#[test]
fn hashes() {
    test!(
        (
            "{}",
            &[],
            &[
                Instruction::new(OpCode::Hash, &[0]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            "{1: 2, 3: 4, 5: 6}",
            &[
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
                Object::Integer(5),
                Object::Integer(6),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Constant, &[5]),
                Instruction::new(OpCode::Constant, &[6]),
                Instruction::new(OpCode::Hash, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "{1: 2 + 3, 4: 5 * 6}",
            &[
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
                Object::Integer(5),
                Object::Integer(6),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Constant, &[5]),
                Instruction::new(OpCode::Constant, &[6]),
                Instruction::new(OpCode::Mul, &[]),
                Instruction::new(OpCode::Hash, &[2]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        )
    )
}

#[test]
fn index() {
    test!(
        (
            "[1, 2, 3][1 + 1]",
            &[
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(1),
                Object::Integer(1),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Array, &[3]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Constant, &[5]),
                Instruction::new(OpCode::Add, &[]),
                Instruction::new(OpCode::Index, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
        (
            "{1: 2}[2 - 1]",
            &[
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(2),
                Object::Integer(1),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Hash, &[1]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Sub, &[]),
                Instruction::new(OpCode::Index, &[]),
                Instruction::new(OpCode::Pop, &[]),
            ],
        ),
    )
}

#[test]
fn functions() {
    test!(
        (
            "fn() { return 10 + 5 }",
            &[
                Object::Integer(10),
                Object::Integer(5),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::Constant, &[2]),
                        Instruction::new(OpCode::Add, &[]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            "fn() { 10 + 5 }",
            &[
                Object::Integer(10),
                Object::Integer(5),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::Constant, &[2]),
                        Instruction::new(OpCode::Add, &[]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            "fn() {10; 5}",
            &[
                Object::Integer(10),
                Object::Integer(5),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::Pop, &[]),
                        Instruction::new(OpCode::Constant, &[2]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            "fn() {}",
            &[Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                [Instruction::new(OpCode::Return, &[]),].into_iter().fold(
                    Bytes::default(),
                    |mut b, i| {
                        b.push(i);
                        b
                    }
                ),
                0,
                0,
            )))],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
    )
}

#[test]
fn function_calls() {
    test!(
        (
            "fn() { 10 + 5 }()",
            &[
                Object::Integer(10),
                Object::Integer(5),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::Constant, &[2]),
                        Instruction::new(OpCode::Add, &[]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Call, &[0]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            r#"
            let noArg = fn() { 24 };
            noArg();
            "#,
            &[
                Object::Integer(24),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::GetGlobal, &[0]),
                Instruction::new(OpCode::Call, &[0]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            r#"
            let oneArg = fn(a) { a; };
            oneArg(24); "#,
            &[
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::GetLocal, &[0]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    1,
                    1,
                ))),
                Object::Integer(24),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::GetGlobal, &[0]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Call, &[1]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            r#"
            let manyArg = fn(a, b, c) { a; b; c; };
            manyArg(24, 25, 26); "#,
            &[
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::GetLocal, &[0]),
                        Instruction::new(OpCode::Pop, &[]),
                        Instruction::new(OpCode::GetLocal, &[1]),
                        Instruction::new(OpCode::Pop, &[]),
                        Instruction::new(OpCode::GetLocal, &[2]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    3,
                    3,
                ))),
                Object::Integer(24),
                Object::Integer(25),
                Object::Integer(26),
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::GetGlobal, &[0]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Constant, &[3]),
                Instruction::new(OpCode::Constant, &[4]),
                Instruction::new(OpCode::Call, &[3]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
    )
}

#[test]
fn function_scopes() {
    test!(
        (
            r#"
            let num = 55;
            fn() { num } "#,
            &[
                Object::Integer(55),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::GetGlobal, &[0]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    0,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[1]),
                Instruction::new(OpCode::SetGlobal, &[0]),
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Pop, &[]),
            ]
        ),
        (
            r#"
            fn() {
                let num = 55;
                num
            } "#,
            &[
                Object::Integer(55),
                Object::CompiledFunc(Rc::new(CompiledFuncObj::new(
                    [
                        Instruction::new(OpCode::Constant, &[1]),
                        Instruction::new(OpCode::SetLocal, &[0]),
                        Instruction::new(OpCode::GetLocal, &[0]),
                        Instruction::new(OpCode::ReturnValue, &[]),
                    ]
                    .into_iter()
                    .fold(Bytes::default(), |mut b, i| {
                        b.push(i);
                        b
                    }),
                    1,
                    0,
                )))
            ],
            &[
                Instruction::new(OpCode::Constant, &[2]),
                Instruction::new(OpCode::Pop, &[]),
            ]
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
            "Wrong instructions for \n{}\nExpected:\n{}got:\n{}",
            input,
            expected_bytes,
            bytecode.instructions,
        );

        assert!(
            &bytecode.constants[1..] == *consts,
            "Wrong constants for \n{}\nExpected:\n{}got:\n{}",
            input,
            print_objs(consts),
            print_objs(&bytecode.constants[1..]),
        );
    }
}

fn print_objs(objs: &[Object]) -> String {
    let mut s = String::new();
    for o in objs {
        s += &format!("{}\n", o);
    }
    s
}
