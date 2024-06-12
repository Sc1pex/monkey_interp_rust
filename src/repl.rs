use crate::{
    ast::Parser,
    compiler::{Compiler, SymbolTableRef},
    eval::Object,
    lexer::Lexer,
    vm::Vm,
};
use std::io::Write;

pub fn start() {
    let mut comp_state = None;
    let mut vm_state = None;

    loop {
        match run(&mut comp_state, &mut vm_state) {
            Ok(o) => println!("{}", o),
            Err(s) => println!("Errors: {}", s),
        }
    }
}

fn run(
    comp_state: &mut Option<(SymbolTableRef, Vec<Object>)>,
    vm_state: &mut Option<Vec<Object>>,
) -> Result<Object, String> {
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

    let mut comp = match comp_state {
        Some((s, c)) => Compiler::new_with_state(s.clone(), c.clone()),
        None => Compiler::default(),
    };
    comp.compile(program)?;
    comp_state.replace(comp.state());

    let mut vm = match vm_state {
        Some(s) => Vm::new_with_state(comp.bytecode(), s.clone()),
        None => Vm::new(comp.bytecode()),
    };
    vm.run()?;
    vm_state.replace(vm.state());

    Ok(vm.last_popped().clone())
}
