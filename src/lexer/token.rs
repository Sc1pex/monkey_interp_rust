use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub literal: TokenLiteral,
}

impl Token {
    pub fn new(ty: TokenType, literal: Option<String>) -> Self {
        match ty {
            TokenType::Ident => {
                let lit = literal.expect("Expected a literal for identifier token");
                Self {
                    ty,
                    literal: TokenLiteral::Ident(lit),
                }
            }
            TokenType::Number => {
                let lit = literal.expect("Expected a literal for number token");
                let lit = lit
                    .parse()
                    .expect("Expected a number literal for number token");
                Self {
                    ty,
                    literal: TokenLiteral::Num(lit),
                }
            }
            TokenType::String => {
                let lit = literal.expect("Expected a literal for string token");
                Self {
                    ty,
                    literal: TokenLiteral::String(lit),
                }
            }
            _ if literal.is_none() => Self {
                literal: TokenLiteral::String(ty.to_string()),
                ty,
            },
            _ => {
                panic!("Token type: {:?} doesn't require any literal", ty)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Let,
    Fn,
    If,
    Else,
    Return,
    True,
    False,

    Ident,
    Number,
    String,

    Assign,
    Bang,
    Plus,
    Minus,
    Slash,
    Star,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Lt,
    Gt,
    Eq,
    NotEq,

    Illegal,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Let => "let",
                TokenType::Fn => "fn",
                TokenType::If => "if",
                TokenType::Else => "else",
                TokenType::Return => "return",
                TokenType::True => "true",
                TokenType::False => "false",
                TokenType::Ident => "ident",
                TokenType::Number => "number",
                TokenType::String => "string",
                TokenType::Assign => "=",
                TokenType::Bang => "!",
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                TokenType::Slash => "/",
                TokenType::Star => "*",
                TokenType::Comma => ",",
                TokenType::Semicolon => ";",
                TokenType::LParen => "(",
                TokenType::RParen => ")",
                TokenType::LBrace => "{",
                TokenType::RBrace => "}",
                TokenType::LBracket => "[",
                TokenType::RBracket => "]",
                TokenType::Lt => "<",
                TokenType::Gt => ">",
                TokenType::Eq => "==",
                TokenType::NotEq => "!=",
                TokenType::Illegal => "illegal",
                TokenType::Eof => "eof",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    Ident(String),
    Num(i64),
    String(String),
}

impl TokenLiteral {
    pub fn ident(&self) -> Option<&str> {
        match self {
            TokenLiteral::Ident(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn num(&self) -> Option<i64> {
        match self {
            &TokenLiteral::Num(n) => Some(n),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<&str> {
        match self {
            TokenLiteral::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
}
