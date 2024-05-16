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
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, None),
            '!' => Token::new(TokenType::Bang, None),
            '+' => Token::new(TokenType::Plus, None),
            '-' => Token::new(TokenType::Minus, None),
            '/' => Token::new(TokenType::Slash, None),
            '*' => Token::new(TokenType::Star, None),
            '(' => Token::new(TokenType::LParen, None),
            ')' => Token::new(TokenType::RParen, None),
            '{' => Token::new(TokenType::LBrace, None),
            '}' => Token::new(TokenType::RBrace, None),
            ',' => Token::new(TokenType::Comma, None),
            ';' => Token::new(TokenType::Semicolon, None),
            '<' => Token::new(TokenType::Lt, None),
            '>' => Token::new(TokenType::Gt, None),
            '\0' => Token::new(TokenType::Eof, None),

            ch if is_ident_char(ch, true) => return self.read_ident(),
            ch if ch.is_ascii_digit() => return self.read_num(),

            _ => Token::new(TokenType::Illegal, None),
        };

        self.read();
        token
    }
}

impl Lexer {
    fn read_ident(&mut self) -> Token {
        let start = self.pos;

        while is_ident_char(self.ch, false) {
            self.read();
        }
        let ident: String = self.input[start..self.pos].iter().collect();
        keyword_or_ident(ident)
    }

    fn read_num(&mut self) -> Token {
        let start = self.pos;

        while self.ch.is_ascii_digit() {
            self.read();
        }
        let num: String = self.input[start..self.pos].iter().collect();
        Token::new(TokenType::Number, Some(num))
    }

    fn read(&mut self) {
        self.ch = if self.read_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_pos]
        };

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }
}

fn is_ident_char(ch: char, first: bool) -> bool {
    if first {
        matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
    } else {
        matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
    }
}

fn keyword_or_ident(s: String) -> Token {
    match s.as_str() {
        "let" => Token::new(TokenType::Let, None),
        "fn" => Token::new(TokenType::Fn, None),
        "if" => Token::new(TokenType::If, None),
        "else" => Token::new(TokenType::Else, None),
        "return" => Token::new(TokenType::Return, None),
        "true" => Token::new(TokenType::True, None),
        "false" => Token::new(TokenType::False, None),
        _ => Token::new(TokenType::Ident, Some(s)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    enum TestToken {
        Token(TokenType),
        Number(i64),
        Ident(String),
    }

    impl PartialEq<Token> for TestToken {
        fn eq(&self, other: &Token) -> bool {
            match self {
                TestToken::Number(x) => {
                    other.ty == TokenType::Number && other.literal == TokenLiteral::Num(*x)
                }
                TestToken::Ident(s) => {
                    other.ty == TokenType::Ident && other.literal == TokenLiteral::Ident(s.into())
                }
                TestToken::Token(t) => other.ty == *t,
            }
        }
    }

    #[test]
    fn lexer_test() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}
        "#;

        let expected = vec![
            TestToken::Token(TokenType::Let),
            TestToken::Ident("five".into()),
            TestToken::Token(TokenType::Assign),
            TestToken::Number(5),
            TestToken::Token(TokenType::Semicolon),
            //
            TestToken::Token(TokenType::Let),
            TestToken::Ident("ten".into()),
            TestToken::Token(TokenType::Assign),
            TestToken::Number(10),
            TestToken::Token(TokenType::Semicolon),
            //
            TestToken::Token(TokenType::Let),
            TestToken::Ident("add".into()),
            TestToken::Token(TokenType::Assign),
            TestToken::Token(TokenType::Fn),
            TestToken::Token(TokenType::LParen),
            TestToken::Ident("x".into()),
            TestToken::Token(TokenType::Comma),
            TestToken::Ident("y".into()),
            TestToken::Token(TokenType::RParen),
            TestToken::Token(TokenType::LBrace),
            TestToken::Ident("x".into()),
            TestToken::Token(TokenType::Plus),
            TestToken::Ident("y".into()),
            TestToken::Token(TokenType::Semicolon),
            TestToken::Token(TokenType::RBrace),
            TestToken::Token(TokenType::Semicolon),
            //
            TestToken::Token(TokenType::Let),
            TestToken::Ident("result".into()),
            TestToken::Token(TokenType::Assign),
            TestToken::Ident("add".into()),
            TestToken::Token(TokenType::LParen),
            TestToken::Ident("five".into()),
            TestToken::Token(TokenType::Comma),
            TestToken::Ident("ten".into()),
            TestToken::Token(TokenType::RParen),
            TestToken::Token(TokenType::Semicolon),
            //
            TestToken::Token(TokenType::Bang),
            TestToken::Token(TokenType::Minus),
            TestToken::Token(TokenType::Slash),
            TestToken::Token(TokenType::Star),
            TestToken::Number(5),
            TestToken::Token(TokenType::Semicolon),
            TestToken::Number(5),
            TestToken::Token(TokenType::Lt),
            TestToken::Number(10),
            TestToken::Token(TokenType::Gt),
            TestToken::Number(5),
            TestToken::Token(TokenType::Semicolon),
            //
            TestToken::Token(TokenType::If),
            TestToken::Token(TokenType::LParen),
            TestToken::Number(5),
            TestToken::Token(TokenType::Lt),
            TestToken::Number(10),
            TestToken::Token(TokenType::RParen),
            TestToken::Token(TokenType::LBrace),
            TestToken::Token(TokenType::Return),
            TestToken::Token(TokenType::True),
            TestToken::Token(TokenType::Semicolon),
            TestToken::Token(TokenType::RBrace),
            TestToken::Token(TokenType::Else),
            TestToken::Token(TokenType::LBrace),
            TestToken::Token(TokenType::Return),
            TestToken::Token(TokenType::False),
            TestToken::Token(TokenType::Semicolon),
            TestToken::Token(TokenType::RBrace),
        ];

        let mut lexer = Lexer::new(input.into());

        for (i, e) in expected.into_iter().enumerate() {
            assert_eq!(e, lexer.next(), "Invalid token at index {}", i);
        }
    }
}
