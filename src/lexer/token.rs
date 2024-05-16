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

#[derive(Debug, PartialEq, Clone)]
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

    Lt,
    Gt,
    Eq,
    NotEq,

    Illegal,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_lowercase())
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
