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

fn test(cases: &[(&str, Object)]) {
    for (inp, exp) in cases {
        let lexer = Lexer::new(inp.to_string());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");

        let res = eval_program(prog);
        assert_eq!(&res, exp);
    }
}
