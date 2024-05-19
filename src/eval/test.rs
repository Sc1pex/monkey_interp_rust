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
        ("5", Ok(Object::Integer(5))),
        ("20", Ok(Object::Integer(20))),
        ("-5", Ok(Object::Integer(-5))),
        ("-20", Ok(Object::Integer(-20))),
    );
}

#[test]
fn eval_bool() {
    test!(
        ("true", Ok(Object::Bool(true))),
        ("false", Ok(Object::Bool(false)))
    );
}

#[test]
fn eval_bang() {
    test!(
        ("!true", Ok(Object::Bool(false))),
        ("!false", Ok(Object::Bool(true))),
        ("!5", Ok(Object::Bool(false))),
        ("!!true", Ok(Object::Bool(true))),
        ("!!false", Ok(Object::Bool(false))),
        ("!!5", Ok(Object::Bool(true))),
    )
}

#[test]
fn eval_math() {
    test!(
        ("5 + 5 + 5 + 5 - 10", Ok(Object::Integer(10))),
        ("2 * 2 * 2 * 2 * 2", Ok(Object::Integer(32))),
        ("-50 + 100 + -50", Ok(Object::Integer(0))),
        ("5 * 2 + 10", Ok(Object::Integer(20))),
        ("5 + 2 * 10", Ok(Object::Integer(25))),
        ("20 + 2 * -10", Ok(Object::Integer(0))),
        ("50 / 2 * 2 + 10", Ok(Object::Integer(60))),
        ("2 * (5 + 10)", Ok(Object::Integer(30))),
        ("3 * 3 * 3 + 10", Ok(Object::Integer(37))),
        ("3 * (3 * 3) + 10", Ok(Object::Integer(37))),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Ok(Object::Integer(50))),
    )
}

#[test]
fn eval_comare() {
    test!(
        ("1 < 2", Ok(Object::Bool(true))),
        ("1 > 2", Ok(Object::Bool(false))),
        ("1 < 1", Ok(Object::Bool(false))),
        ("1 > 1", Ok(Object::Bool(false))),
        ("1 == 1", Ok(Object::Bool(true))),
        ("1 != 1", Ok(Object::Bool(false))),
        ("1 == 2", Ok(Object::Bool(false))),
        ("1 != 2", Ok(Object::Bool(true))),
        ("true == true", Ok(Object::Bool(true))),
        ("false == false", Ok(Object::Bool(true))),
        ("true == false", Ok(Object::Bool(false))),
        ("true != false", Ok(Object::Bool(true))),
        ("false != true", Ok(Object::Bool(true))),
        ("(1 < 2) == true", Ok(Object::Bool(true))),
        ("(1 < 2) == false", Ok(Object::Bool(false))),
        ("(1 > 2) == true", Ok(Object::Bool(false))),
        ("(1 > 2) == false", Ok(Object::Bool(true))),
    )
}

#[test]
fn eval_if() {
    test!(
        ("if (true) { 10 }", Ok(Object::Integer(10))),
        ("if (false) { 10 }", Ok(Object::Null)),
        ("if (1) { 10 }", Ok(Object::Integer(10))),
        ("if (1 < 2) { 10 }", Ok(Object::Integer(10))),
        ("if (1 > 2) { 10 }", Ok(Object::Null)),
        ("if (1 > 2) { 10 } else { 20 }", Ok(Object::Integer(20))),
        ("if (1 < 2) { 10 } else { 20 }", Ok(Object::Integer(10))),
        ("if (0) { 10 } else { 20 }", Ok(Object::Integer(20))),
    )
}

#[test]
fn eval_return() {
    test!(
        ("return 10;", Ok(Object::Integer(10))),
        ("return 10; 9;", Ok(Object::Integer(10))),
        ("return 2 * 5; 9;", Ok(Object::Integer(10))),
        ("9; return 2 * 5; 9;", Ok(Object::Integer(10))),
        (
            r#"if (10 > 1) {
                if (10 > 1) {
                 return 10;
                }
                return 1;
            }"#,
            Ok(Object::Integer(10))
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
    )
}

fn test(cases: &[(&str, EvalResult)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");

        let res = eval_program(prog);
        assert_eq!(&res, exp);
    }
}
