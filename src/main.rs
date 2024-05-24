use ast::Parser;
use eval::{eval_program, Environment};
use lexer::Lexer;

mod ast;
mod compiler;
mod eval;
mod lexer;
mod repl;

fn main() {
    let mut args = std::env::args().skip(1);

    match args.len() {
        0 => repl::start(),
        1 => {
            let file = args.next().unwrap();
            run(&file)
        }
        _ => println!("Usage: monkey [file]"),
    }
}

fn run(file: &str) {
    let contents = std::fs::read_to_string(file).expect("Failed to open file");

    let lexer = Lexer::new(contents);
    let mut parser = Parser::new(lexer);

    let env = Environment::new();
    let program = parser.parse().unwrap();

    match eval_program(program, &env) {
        Ok(_) => {}
        Err(e) => println!("Evaluation error: {}", e),
    }
}
