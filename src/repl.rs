use crate::lexer::{Lexer, TokenType};
use std::io::Write;

pub fn start() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input);
        loop {
            let t = lexer.next();
            if t.ty == TokenType::Eof {
                break;
            }
            println!("{:?}", t);
        }
    }
}
