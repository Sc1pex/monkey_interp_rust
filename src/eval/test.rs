use super::*;
use crate::{ast::Parser, lexer::Lexer};

#[test]
fn eval_int() {
    let inputs = [("5", Object::Integer(5)), ("20", Object::Integer(20))];

    for (inp, exp) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");

        let res = eval_program(prog);
        assert_eq!(res, exp);
    }
}

#[test]
fn eval_bool() {
    let inputs = [("true", Object::Bool(true)), ("false", Object::Bool(false))];

    for (inp, exp) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse().expect("Skill issue");

        let res = eval_program(prog);
        assert_eq!(res, exp);
    }
}
