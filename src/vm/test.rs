use super::*;
use crate::{ast::Parser, compiler::Compiler, lexer::Lexer};

#[test]
fn integer_math() {
    let inputs = [
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
    ];

    for (inp, exp) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().expect("Skill issue");

        let mut compiler = Compiler::default();
        compiler.compile(program).expect("Skill issue");
        let bytecode = compiler.bytecode();

        let mut vm = Vm::new(bytecode);
        vm.run().unwrap();

        assert_eq!(vm.last_popped(), &exp);
    }
}
