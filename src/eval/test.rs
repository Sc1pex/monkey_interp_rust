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
        ("5", Object::Integer(5)),
        ("20", Object::Integer(20)),
        ("-5", Object::Integer(-5)),
        ("-20", Object::Integer(-20)),
    );
}

#[test]
fn eval_bool() {
    test!(("true", Object::Bool(true)), ("false", Object::Bool(false)));
}

#[test]
fn eval_bang() {
    test!(
        ("!true", Object::Bool(false)),
        ("!false", Object::Bool(true)),
        ("!5", Object::Bool(false)),
        ("!!true", Object::Bool(true)),
        ("!!false", Object::Bool(false)),
        ("!!5", Object::Bool(true)),
    )
}

#[test]
fn eval_math() {
    test!(
        ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
        ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
        ("-50 + 100 + -50", Object::Integer(0)),
        ("5 * 2 + 10", Object::Integer(20)),
        ("5 + 2 * 10", Object::Integer(25)),
        ("20 + 2 * -10", Object::Integer(0)),
        ("50 / 2 * 2 + 10", Object::Integer(60)),
        ("2 * (5 + 10)", Object::Integer(30)),
        ("3 * 3 * 3 + 10", Object::Integer(37)),
        ("3 * (3 * 3) + 10", Object::Integer(37)),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
    )
}

#[test]
fn eval_comare() {
    test!(
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
    )
}

fn test(cases: &[(&str, Object)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");

        let res = eval_program(prog);
        assert_eq!(&res, exp);
    }
}
