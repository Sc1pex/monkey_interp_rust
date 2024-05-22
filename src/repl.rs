use crate::{
    ast::Parser,
    eval::{eval_program, Environment},
    lexer::Lexer,
};
use std::io::Write;

pub fn start() {
    let env = Environment::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        match parser.parse() {
            Ok(p) => {
                let eval = eval_program(p, &env.clone());
                match eval {
                    Ok(r) => println!("{}", r),
                    Err(e) => println!("Evaluation error: {}", e),
                }
            }
            Err(e) => println!("Parser error: {:?}", e),
        }
    }
}
