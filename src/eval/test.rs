use std::collections::HashMap;

use super::*;
use crate::{ast::Parser, lexer::Lexer};

macro_rules! test {
    ($($case:expr),* $(,)?) => {
        test(&[$($case),*])
    };
}

#[test]
fn eval_int() {
    test!(
        ("5", Ok(Rc::new(Object::Integer(5)))),
        ("20", Ok(Rc::new(Object::Integer(20)))),
        ("-5", Ok(Rc::new(Object::Integer(-5)))),
        ("-20", Ok(Rc::new(Object::Integer(-20)))),
    );
}

#[test]
fn eval_bool() {
    test!(
        ("true", Ok(Rc::new(Object::Bool(true)))),
        ("false", Ok(Rc::new(Object::Bool(false))))
    );
}

#[test]
fn eval_string() {
    test!(
        ("\"foobar\"", Ok(Rc::new(Object::String("foobar".into())))),
        (
            r#""hello" + " " + "world" "#,
            Ok(Rc::new(Object::String("hello world".into())))
        ),
    );
}

#[test]
fn eval_bang() {
    test!(
        ("!true", Ok(Rc::new(Object::Bool(false)))),
        ("!false", Ok(Rc::new(Object::Bool(true)))),
        ("!5", Ok(Rc::new(Object::Bool(false)))),
        ("!!true", Ok(Rc::new(Object::Bool(true)))),
        ("!!false", Ok(Rc::new(Object::Bool(false)))),
        ("!!5", Ok(Rc::new(Object::Bool(true)))),
    )
}

#[test]
fn eval_math() {
    test!(
        ("5 + 5 + 5 + 5 - 10", Ok(Rc::new(Object::Integer(10)))),
        ("2 * 2 * 2 * 2 * 2", Ok(Rc::new(Object::Integer(32)))),
        ("-50 + 100 + -50", Ok(Rc::new(Object::Integer(0)))),
        ("5 * 2 + 10", Ok(Rc::new(Object::Integer(20)))),
        ("5 + 2 * 10", Ok(Rc::new(Object::Integer(25)))),
        ("20 + 2 * -10", Ok(Rc::new(Object::Integer(0)))),
        ("50 / 2 * 2 + 10", Ok(Rc::new(Object::Integer(60)))),
        ("2 * (5 + 10)", Ok(Rc::new(Object::Integer(30)))),
        ("3 * 3 * 3 + 10", Ok(Rc::new(Object::Integer(37)))),
        ("3 * (3 * 3) + 10", Ok(Rc::new(Object::Integer(37)))),
        (
            "(5 + 10 * 2 + 15 / 3) * 2 + -10",
            Ok(Rc::new(Object::Integer(50)))
        ),
    )
}

#[test]
fn eval_comare() {
    test!(
        ("1 < 2", Ok(Rc::new(Object::Bool(true)))),
        ("1 > 2", Ok(Rc::new(Object::Bool(false)))),
        ("1 < 1", Ok(Rc::new(Object::Bool(false)))),
        ("1 > 1", Ok(Rc::new(Object::Bool(false)))),
        ("1 == 1", Ok(Rc::new(Object::Bool(true)))),
        ("1 != 1", Ok(Rc::new(Object::Bool(false)))),
        ("1 == 2", Ok(Rc::new(Object::Bool(false)))),
        ("1 != 2", Ok(Rc::new(Object::Bool(true)))),
        ("true == true", Ok(Rc::new(Object::Bool(true)))),
        ("false == false", Ok(Rc::new(Object::Bool(true)))),
        ("true == false", Ok(Rc::new(Object::Bool(false)))),
        ("true != false", Ok(Rc::new(Object::Bool(true)))),
        ("false != true", Ok(Rc::new(Object::Bool(true)))),
        ("(1 < 2) == true", Ok(Rc::new(Object::Bool(true)))),
        ("(1 < 2) == false", Ok(Rc::new(Object::Bool(false)))),
        ("(1 > 2) == true", Ok(Rc::new(Object::Bool(false)))),
        ("(1 > 2) == false", Ok(Rc::new(Object::Bool(true)))),
        (r#" "hello" == "hello" "#, Ok(Rc::new(Object::Bool(true)))),
        (
            r#" "lorem ipsum" == "good placeholder" "#,
            Ok(Rc::new(Object::Bool(false)))
        ),
        (
            r#" "lorem ipsum" != "good placeholder" "#,
            Ok(Rc::new(Object::Bool(true)))
        )
    )
}

#[test]
fn eval_if() {
    test!(
        ("if (true) { 10 }", Ok(Rc::new(Object::Integer(10)))),
        ("if (false) { 10 }", Ok(Rc::new(Object::Null))),
        ("if (1) { 10 }", Ok(Rc::new(Object::Integer(10)))),
        ("if (1 < 2) { 10 }", Ok(Rc::new(Object::Integer(10)))),
        ("if (1 > 2) { 10 }", Ok(Rc::new(Object::Null))),
        (
            "if (1 > 2) { 10 } else { 20 }",
            Ok(Rc::new(Object::Integer(20)))
        ),
        (
            "if (1 < 2) { 10 } else { 20 }",
            Ok(Rc::new(Object::Integer(10)))
        ),
        (
            "if (0) { 10 } else { 20 }",
            Ok(Rc::new(Object::Integer(20)))
        ),
    )
}

#[test]
fn eval_return() {
    test!(
        ("return 10;", Ok(Rc::new(Object::Integer(10)))),
        ("return 10; 9;", Ok(Rc::new(Object::Integer(10)))),
        ("return 2 * 5; 9;", Ok(Rc::new(Object::Integer(10)))),
        ("9; return 2 * 5; 9;", Ok(Rc::new(Object::Integer(10)))),
        (
            r#"if (10 > 1) {
                if (10 > 1) {
                 return 10;
                }
                return 1;
            }"#,
            Ok(Rc::new(Object::Integer(10)))
        )
    )
}

#[test]
fn error_handling() {
    test!(
        ("5 + true;", Err("type mismatch: INTEGER + BOOL".into())),
        ("5 + true; 5;", Err("type mismatch: INTEGER + BOOL".into())),
        ("-true", Err("unknown operator: -BOOL".into())),
        ("true + false;", Err("unknown operator: BOOL + BOOL".into())),
        (
            "5; true + false; 5",
            Err("unknown operator: BOOL + BOOL".into())
        ),
        (
            "if (10 > 1) { true + false; }",
            Err("unknown operator: BOOL + BOOL".into()),
        ),
        (
            r#"if (10 > 1) {
                if (10 > 1) {
                 return true + false;
                }
                return 1;
            }"#,
            Err("unknown operator: BOOL + BOOL".into()),
        ),
        ("baz", Err("identifier not found: baz".into())),
        (
            r#" "hello" - "world" "#,
            Err("unknown operator: STRING - STRING".into())
        ),
        (
            r#"{"name": "Monkey"}[fn(x) { x }];"#,
            Err("unusable as hash key: FUNCTION".into()),
        )
    )
}

#[test]
fn eval_let() {
    test!(
        ("let a = 5; a;", Ok(Rc::new(Object::Integer(5)))),
        ("let a = 5 * 5; a;", Ok(Rc::new(Object::Integer(25)))),
        ("let a = 5; let b = a; b;", Ok(Rc::new(Object::Integer(5)))),
        (
            "let a = 5; let b = a; let c = a + b + 5; c;",
            Ok(Rc::new(Object::Integer(15)))
        ),
    )
}

#[test]
fn eval_func() {
    test!(
        (
            "let identity = fn(x) { x; }; identity(5);",
            Ok(Rc::new(Object::Integer(5)))
        ),
        (
            "let identity = fn(x) { return x; }; identity(5);",
            Ok(Rc::new(Object::Integer(5)))
        ),
        (
            "let double = fn(x) { x * 2; }; double(5);",
            Ok(Rc::new(Object::Integer(10)))
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5, 5);",
            Ok(Rc::new(Object::Integer(10)))
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            Ok(Rc::new(Object::Integer(20)))
        ),
        ("fn(x) { x; }(5)", Ok(Rc::new(Object::Integer(5)))),
        (
            "let fact = fn(n) { if (n == 0) { 1 } else { n * fact(n - 1)} }; fact(5)",
            Ok(Rc::new(Object::Integer(120)))
        )
    )
}

#[test]
fn array_literal() {
    test!((
        "[1, 2 * 2, 3 + 3]",
        Ok(Rc::new(Object::Array(ArrayObj {
            elements: vec![
                Rc::new(Object::Integer(1)),
                Rc::new(Object::Integer(4)),
                Rc::new(Object::Integer(6))
            ]
        })))
    ))
}

#[test]
fn index_arr() {
    test!(
        ("[1, 2, 3][0]", Ok(Rc::new(Object::Integer(1)))),
        ("[1, 2, 3][1]", Ok(Rc::new(Object::Integer(2)))),
        ("[1, 2, 3][2]", Ok(Rc::new(Object::Integer(3)))),
        ("let i = 0; [1][i];", Ok(Rc::new(Object::Integer(1)))),
        ("[1, 2, 3][1 + 1];", Ok(Rc::new(Object::Integer(3)))),
        (
            "let myArray = [1, 2, 3]; myArray[2];",
            Ok(Rc::new(Object::Integer(3)))
        ),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            Ok(Rc::new(Object::Integer(6)))
        ),
        (
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
            Ok(Rc::new(Object::Integer(2)))
        ),
        ("[1, 2, 3][3]", Ok(Rc::new(Object::Null))),
        ("[1, 2, 3][-1]", Ok(Rc::new(Object::Null))),
    )
}

#[test]
fn hash_literal() {
    test!((
        r#"
    let two = "two";
    {
        "one": 10 - 9,
        two: 1 + 1,
        "thr" + "ee": 6 / 2,
        4: 4,
        true: 5,
        false: 6
    }
    "#,
        Ok(Rc::new(Object::Hash(HashObj {
            map: HashMap::from([
                (
                    Rc::new(Object::String("one".into())),
                    Rc::new(Object::Integer(1))
                ),
                (
                    Rc::new(Object::String("two".into())),
                    Rc::new(Object::Integer(2))
                ),
                (
                    Rc::new(Object::String("three".into())),
                    Rc::new(Object::Integer(3))
                ),
                (Rc::new(Object::Integer(4)), Rc::new(Object::Integer(4))),
                (Rc::new(Object::Bool(true)), Rc::new(Object::Integer(5))),
                (Rc::new(Object::Bool(false)), Rc::new(Object::Integer(6))),
            ])
        })))
    ))
}

#[test]
fn index_hash() {
    test!(
        (r#"{"foo": 5}["foo"]"#, Ok(Rc::new(Object::Integer(5)))),
        (r#"{"foo": 5}["bar"]"#, Ok(Rc::new(Object::Null))),
        (
            r#"let key = "foo"; {"foo": 5}[key]"#,
            Ok(Rc::new(Object::Integer(5)))
        ),
        (r#"{}["foo"]"#, Ok(Rc::new(Object::Null))),
        (r#"{5: 5}[5]"#, Ok(Rc::new(Object::Integer(5)))),
        (r#"{true: 5}[true]"#, Ok(Rc::new(Object::Integer(5)))),
        (r#"{false: 5}[false]"#, Ok(Rc::new(Object::Integer(5)))),
    )
}

#[test]
fn builtin_len() {
    test!(
        (r#"len("")"#, Ok(Rc::new(Object::Integer(0)))),
        (r#"len("four")"#, Ok(Rc::new(Object::Integer(4)))),
        (r#"len("hello world")"#, Ok(Rc::new(Object::Integer(11)))),
        (
            r#"len(1)"#,
            Err("argument to `len` not supported, got INTEGER".into())
        ),
        (
            r#"len("one", "two")"#,
            Err("wrong number of arguments. expected 1, got 2".into())
        ),
        (r#"len([1, 2, 3, 4])"#, Ok(Rc::new(Object::Integer(4)))),
    )
}

#[test]
fn builtin_first() {
    test!(
        (
            r#"first(["a", "b"])"#,
            Ok(Rc::new(Object::String("a".into())))
        ),
        (r#"first([])"#, Ok(Rc::new(Object::Null))),
        (
            r#"first(1)"#,
            Err("argument to `first` not supported, got INTEGER".into())
        ),
        (
            r#"first("one", "two")"#,
            Err("wrong number of arguments. expected 1, got 2".into())
        ),
    )
}

#[test]
fn builtin_last() {
    test!(
        (
            r#"last(["a", "b"])"#,
            Ok(Rc::new(Object::String("b".into())))
        ),
        (r#"last([])"#, Ok(Rc::new(Object::Null))),
        (
            r#"last(1)"#,
            Err("argument to `last` not supported, got INTEGER".into())
        ),
        (
            r#"last("one", "two")"#,
            Err("wrong number of arguments. expected 1, got 2".into())
        ),
    )
}

#[test]
fn builtin_rest() {
    test!(
        (
            r#"rest(["a", "b", "c"])"#,
            Ok(Rc::new(Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::String("b".into())),
                    Rc::new(Object::String("c".into()))
                ]
            })))
        ),
        (
            r#"rest(["a"])"#,
            Ok(Rc::new(Object::Array(ArrayObj { elements: vec![] })))
        ),
        (
            r#"rest([])"#,
            Ok(Rc::new(Object::Array(ArrayObj { elements: vec![] })))
        ),
        (
            r#"rest(1)"#,
            Err("argument to `rest` not supported, got INTEGER".into())
        ),
        (
            r#"rest("one", "two")"#,
            Err("wrong number of arguments. expected 1, got 2".into())
        ),
    )
}

#[test]
fn builtin_push() {
    test!(
        (
            r#"push(["a", "b"], "c")"#,
            Ok(Rc::new(Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::String("a".into())),
                    Rc::new(Object::String("b".into())),
                    Rc::new(Object::String("c".into()))
                ]
            })))
        ),
        (
            r#"push(["a"], 1)"#,
            Ok(Rc::new(Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::String("a".into())),
                    Rc::new(Object::Integer(1))
                ]
            })))
        ),
        (
            r#"push(["a"], [1])"#,
            Ok(Rc::new(Object::Array(ArrayObj {
                elements: vec![
                    Rc::new(Object::String("a".into())),
                    Rc::new(Object::Array(ArrayObj {
                        elements: vec![Rc::new(Object::Integer(1))]
                    }))
                ]
            })))
        ),
        (
            r#"push([], "bar")"#,
            Ok(Rc::new(Object::Array(ArrayObj {
                elements: vec![Rc::new(Object::String("bar".into()))]
            })))
        ),
        (
            r#"push(1, 2)"#,
            Err("argument to `push` not supported, got INTEGER".into())
        ),
        (
            r#"push([])"#,
            Err("wrong number of arguments. expected 2, got 1".into())
        ),
    )
}

fn test(cases: &[(&str, EvalResult)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");
        let env = Environment::new();

        let res = eval_program(prog, &env);
        assert_eq!(&res, exp);
    }
}
