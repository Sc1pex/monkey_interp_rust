use super::*;
use crate::{ast::Parser, compiler::Compiler, lexer::Lexer};

#[test]
fn integer_math() {
    let inputs = [
        ("1", Object::Integer(1)),
        ("2", Object::Integer(2)),
        ("1 + 2", Object::Integer(3)),
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

        assert_eq!(vm.stack_top(), Some(exp).as_ref());
    }
}
