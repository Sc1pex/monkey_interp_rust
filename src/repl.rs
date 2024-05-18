use crate::{ast::Parser, lexer::Lexer};
use std::io::Write;

pub fn start() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        match parser.parse() {
            Ok(p) => println!("{}", p),
            Err(e) => println!("Parser error: {:?}", e),
        }
    }
}
