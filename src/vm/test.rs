use super::*;
use crate::{
    ast::Parser,
    compiler::Compiler,
    eval::{ArrayObj, HashObj},
    lexer::Lexer,
};
use std::{collections::HashMap, rc::Rc};

macro_rules! test {
    ($($case:expr),* $(,)?) => {
        test(&[$($case),*])
    };
}

macro_rules! test_err {
    ($($case:expr),* $(,)?) => {
        test_err(&[$($case),*])
    };
}

#[test]
fn integer_math() {
    test!(
        ("1", Object::Integer(1)),
        ("2", Object::Integer(2)),
        ("1 + 2", Object::Integer(3)),
        ("1 - 2", Object::Integer(-1)),
        ("1 * 2", Object::Integer(2)),
        ("4 / 2", Object::Integer(2)),
        ("50 / 2 * 2 + 10 - 5", Object::Integer(55)),
        ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
        ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
        ("5 * 2 + 10", Object::Integer(20)),
        ("5 + 2 * 10", Object::Integer(25)),
        ("5 * (2 + 10)", Object::Integer(60)),
        ("-5", Object::Integer(-5)),
        ("-10", Object::Integer(-10)),
        ("-50 + 100 + -50", Object::Integer(0)),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
    )
}

#[test]
fn bool_expressions() {
    test!(
        ("true", Object::Bool(true)),
        ("false", Object::Bool(false)),
        ("1 < 2", Object::Bool(true)),
        ("1 > 2", Object::Bool(false)),
        ("1 < 1", Object::Bool(false)),
        ("1 > 1", Object::Bool(false)),
        ("1 == 1", Object::Bool(true)),
        ("1 != 1", Object::Bool(false)),
        ("1 == 2", Object::Bool(false)),
        ("1 != 2", Object::Bool(true)),
        ("true == true", Object::Bool(true)),
        ("false == false", Object::Bool(true)),
        ("true == false", Object::Bool(false)),
        ("true != false", Object::Bool(true)),
        ("false != true", Object::Bool(true)),
        ("(1 < 2) == true", Object::Bool(true)),
        ("(1 < 2) == false", Object::Bool(false)),
        ("(1 > 2) == true", Object::Bool(false)),
        ("(1 > 2) == false", Object::Bool(true)),
        ("!true", Object::Bool(false)),
        ("!false", Object::Bool(true)),
        ("!5", Object::Bool(false)),
        ("!!true", Object::Bool(true)),
        ("!!false", Object::Bool(false)),
        ("!!5", Object::Bool(true)),
        ("!(if (false) { 5; })", Object::Bool(true)),
    )
}

#[test]
fn conditionals() {
    test!(
        ("if (true) { 10 }", Object::Integer(10)),
        ("if (true) { 10 } else { 20 }", Object::Integer(10)),
        ("if (false) { 10 } else { 20 } ", Object::Integer(20)),
        ("if (1) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        ("if (false) { 10 }", Object::Null),
        ("if (1 > 2) { 10 }", Object::Null),
        (
            "if ((if (false) { 10 })) { 10 } else { 20 }",
            Object::Integer(20)
        ),
    )
}

#[test]
fn global_let() {
    test!(
        ("let one = 1; one", Object::Integer(1)),
        ("let one = 1; let two = 2; one + two", Object::Integer(3)),
        (
            "let one = 1; let two = one + one; one + two",
            Object::Integer(3)
        ),
    )
}

#[test]
fn strings() {
    test!(
        (r#" "monkey" "#, Object::String("monkey".into())),
        (r#" "mon" + "key" "#, Object::String("monkey".into())),
        (
            r#" "mon" + "key" + "banana" "#,
            Object::String("monkeybanana".into())
        ),
    )
}

#[test]
fn arrays() {
    test!(
        ("[]", Object::Array(ArrayObj { elements: vec![] })),
        (
            "[1, 2, 3]",
            Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::Integer(1)),
                    Rc::new(Object::Integer(2)),
                    Rc::new(Object::Integer(3)),
                ]
            })
        ),
        (
            "[1 + 2, 3 * 4, 5 + 6]",
            Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::Integer(3)),
                    Rc::new(Object::Integer(12)),
                    Rc::new(Object::Integer(11)),
                ]
            })
        ),
    )
}

#[test]
fn hashes() {
    test!(
        (
            "{}",
            Object::Hash(HashObj {
                map: HashMap::new()
            })
        ),
        (
            "{1: 2, 2: 3}",
            Object::Hash(HashObj {
                map: [
                    (Rc::new(Object::Integer(1)), Rc::new(Object::Integer(2))),
                    (Rc::new(Object::Integer(2)), Rc::new(Object::Integer(3))),
                ]
                .into()
            })
        ),
        (
            "{1 + 1: 2 * 2, 3 + 3: 4 * 4}",
            Object::Hash(HashObj {
                map: [
                    (Rc::new(Object::Integer(2)), Rc::new(Object::Integer(4))),
                    (Rc::new(Object::Integer(6)), Rc::new(Object::Integer(16))),
                ]
                .into()
            })
        ),
    )
}

#[test]
fn index() {
    test!(
        ("[1, 2, 3][1]", Object::Integer(2)),
        ("[1, 2, 3][0 + 2]", Object::Integer(3)),
        ("[[1, 1, 1]][0][0]", Object::Integer(1)),
        ("[][0]", Object::Null),
        ("[1, 2, 3][99]", Object::Null),
        ("[1][-1]", Object::Null),
        ("{1: 1, 2: 2}[1]", Object::Integer(1)),
        (
            r#"{1: 1, 2: 2, "abc": "def"}["abc"]"#,
            Object::String("def".into())
        ),
        ("{1: 1}[0]", Object::Null),
        ("{}[0]", Object::Null),
    )
}

#[test]
fn functions() {
    test!(
        (
            r#"
            let a = fn() { 10 + 5}; 
            a() "#,
            Object::Integer(15)
        ),
        (
            r#"
            let one = fn() { 1; };
            let two = fn() { 2; };
            one() + two() "#,
            Object::Integer(3)
        ),
        (
            r#"
            let a = fn() { 1 };
            let b = fn() { a() + 1 };
            fn(){ b() + 1 }()"#,
            Object::Integer(3)
        ),
    )
}

#[test]
fn functions_return() {
    test!((
        r#"
        let a = fn() {return 20; return 40;}; 
        a() "#,
        Object::Integer(20)
    ))
}

#[test]
fn functions_no_return() {
    test!(
        (
            r#"
            let a = fn(){}; 
            a() "#,
            Object::Null
        ),
        (
            r#"
            let a = fn(){}; 
            let b = fn(){ a() }; 
            b() "#,
            Object::Null
        )
    )
}

#[test]
fn higher_oreder_funcs() {
    test!(
        (
            r#"
            let returnsOne = fn() { 1; };
            let returnsOneReturner = fn() { returnsOne; };
            returnsOneReturner()(); "#,
            Object::Integer(1)
        ),
        (
            r#"
            let returnsOneReturner = fn() {
                let returnsOne = fn() { 1; };
                returnsOne;
            };
            returnsOneReturner()(); "#,
            Object::Integer(1)
        )
    )
}

#[test]
fn funcs_with_bindings() {
    test!(
        (
            r#"
            let one = fn() { let one =  1; one}
            one(); "#,
            Object::Integer(1)
        ),
        (
            r#"
            let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
            oneAndTwo(); "#,
            Object::Integer(3)
        ),
        (
            r#"
            let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
            let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
            oneAndTwo() + threeAndFour(); "#,
            Object::Integer(10)
        ),
        (
            r#"
            let firstFoobar = fn() { let foobar = 50; foobar; };
            let secondFoobar = fn() { let foobar = 100; foobar; };
            firstFoobar() + secondFoobar(); "#,
            Object::Integer(150)
        ),
        (
            r#"
            let globalSeed = 50;
            let minusOne = fn() {
                let num = 1;
                globalSeed - num;
            }
            let minusTwo = fn() {
                let num = 2;
                globalSeed - num;
            }
            minusOne() + minusTwo();  "#,
            Object::Integer(97)
        ),
    )
}

#[test]
fn funcs_with_arguments() {
    test!(
        (
            r#"
            let identity = fn(a) { a; };
            identity(4); "#,
            Object::Integer(4)
        ),
        (
            r#"
            let sum = fn(a, b) { a + b; };
            sum(1, 3); "#,
            Object::Integer(4)
        ),
        (
            r#"
            let sum = fn(a, b) {
                let c = a + b;
                c;
            };
            sum(1, 2); "#,
            Object::Integer(3)
        ),
        (
            r#"
            let globalNum = 10;
            let sum = fn(a, b) {
                let c = a + b;
                c + globalNum;
            };
            let outer = fn() {
                sum(1, 2) + sum(3, 4) + globalNum;
            };
            outer() + globalNum; "#,
            Object::Integer(50)
        )
    )
}

#[test]
fn call_with_wrong_arguments() {
    test_err!(
        (
            "fn() { 1; }(1);",
            "wrong number of arguments. expected 0, got 1"
        ),
        (
            "fn(a) { a; }();",
            "wrong number of arguments. expected 1, got 0"
        ),
        (
            "fn(a, b) { a + b; }(1);",
            "wrong number of arguments. expected 2, got 1"
        ),
    )
}

fn test(cases: &[(&str, Object)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().expect("Skill issue");

        let mut compiler = Compiler::default();
        compiler.compile(program).expect("Skill issue");
        let bytecode = compiler.bytecode();
        let s = format!("{}", bytecode.instructions);
        println!("{}", s);
        for c in &bytecode.constants {
            println!("{}", c);
        }

        let mut vm = Vm::new(bytecode);
        vm.run().unwrap();

        assert_eq!(vm.last_popped(), exp, "{}", s);
    }
}

fn test_err(cases: &[(&str, &str)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().expect("Skill issue");

        let mut compiler = Compiler::default();
        compiler.compile(program).expect("Skill issue");
        let bytecode = compiler.bytecode();
        let s = format!("{}", bytecode.instructions);
        println!("{}", s);
        for c in &bytecode.constants {
            println!("{}", c);
        }

        let mut vm = Vm::new(bytecode);

        match vm.run() {
            Ok(_) => panic!("test did not error:\n{}", inp),
            Err(e) => assert_eq!(&e, exp),
        }
    }
}
