mod token;

pub use token::*;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut s = Self {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: '\0',
        };
        s.read();
        s
    }

    pub fn next(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, None),
            '+' => Token::new(TokenType::Plus, None),
            '(' => Token::new(TokenType::LParen, None),
            ')' => Token::new(TokenType::RParen, None),
            '{' => Token::new(TokenType::LBrace, None),
            '}' => Token::new(TokenType::RBrace, None),
            ',' => Token::new(TokenType::Comma, None),
            ';' => Token::new(TokenType::Semicolon, None),
            '\0' => Token::new(TokenType::Eof, None),

            _ => Token::new(TokenType::Illegal, None),
        };

        self.read();
        token
    }
}

impl Lexer {
    pub fn read(&mut self) {
        self.ch = if self.read_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_pos]
        };

        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_test() {
        let input = "=+(){},;";

        let expected = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
        ];

        let mut lexer = Lexer::new(input.into());

        for (i, e) in expected.into_iter().enumerate() {
            assert_eq!(e, lexer.next().ty, "Invalid token at index {}", i);
        }
    }
}
