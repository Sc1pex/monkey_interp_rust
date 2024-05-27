use super::*;
use crate::{ast::Parser, compiler::Compiler, lexer::Lexer};

macro_rules! test {
    ($($case:expr),* $(,)?) => {
        test(&[$($case),*])
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

        let mut vm = Vm::new(bytecode);
        vm.run().unwrap();

        assert_eq!(vm.last_popped(), exp);
    }
}
