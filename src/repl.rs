use crate::{ast::Parser, compiler::Compiler, eval::Object, lexer::Lexer, vm::Vm};
use std::io::Write;

pub fn start() {
    loop {
        match run() {
            Ok(o) => println!("{}", o),
            Err(s) => println!("Errors: {}", s),
        }
    }
}

fn run() -> Result<Object, String> {
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse().map_err(|e| {
        e.into_iter().fold(String::new(), |mut acc, e| {
            acc += &format!("{:?}", e);
            acc
        })
    })?;

    let mut comp = Compiler::default();
    comp.compile(program)?;

    let mut vm = Vm::new(comp.bytecode());
    vm.run()?;

    Ok(vm.stack_top().unwrap_or(&Object::Null).clone())
}
