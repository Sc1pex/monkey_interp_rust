use super::*;
use crate::lexer::{Lexer, Token, TokenType};

pub struct Parser {
    lexer: Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut s = Self {
            lexer: l,
            cur_token: Token::new(TokenType::Illegal, None),
            peek_token: Token::new(TokenType::Illegal, None),
        };
        s.next();
        s.next();
        s
    }

    pub fn parse(&mut self) -> ParseResult<Program> {
        let mut statements = vec![];
        let mut errors = vec![];

        while self.cur_token.ty != TokenType::Eof {
            match self.parse_stmt() {
                Ok(s) => statements.push(s),
                Err(mut e) => errors.append(&mut e),
            }
            self.next();
        }

        if errors.is_empty() {
            Ok(Program { statements })
        } else {
            Err(errors)
        }
    }
}

impl Parser {
    fn parse_stmt(&mut self) -> ParseResult<Statement> {
        match self.cur_token.ty {
            TokenType::Let => self.parse_let(),
            TokenType::Return => self.parse_return(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_expr_stmt(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expr(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next();
        }

        Ok(Statement::Expression(ExpressionStmt { expr }))
    }

    fn parse_return(&mut self) -> ParseResult<Statement> {
        self.next(); // Skip 'Return' token

        // TODO: Parse expression
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next();
        }

        Ok(Statement::Return(ReturnStmt {
            expr: Expression::Todo,
        }))
    }

    fn parse_let(&mut self) -> ParseResult<Statement> {
        self.expect_peek(TokenType::Ident)?;
        let ident: String = self.cur_token.literal.ident().unwrap().into();

        self.expect_peek(TokenType::Assign)?;

        // TODO: Parse expression
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next();
        }

        Ok(Statement::Let(LetStmt {
            ident,
            expr: Expression::Todo,
        }))
    }

    fn parse_expr(&mut self, prec: Precedence) -> ParseResult<Expression> {
        self.parse_prefix()
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match self.cur_token.ty {
            TokenType::Ident => self.parse_ident(),
            TokenType::Number => self.parse_number(),
            _ => Err(vec![ParseErrorKind::UnknownPrefixExpr]),
        }
    }

    fn next(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    fn cur_token_is(&self, ty: TokenType) -> bool {
        self.cur_token.ty == ty
    }
    fn peek_token_is(&self, ty: TokenType) -> bool {
        self.peek_token.ty == ty
    }

    fn expect_peek(&mut self, ty: TokenType) -> ParseResult<()> {
        if self.peek_token_is(ty) {
            self.next();
            Ok(())
        } else {
            Err(vec![ParseErrorKind::UnexpectedToken])
        }
    }
}

impl Parser {
    fn parse_ident(&mut self) -> ParseResult<Expression> {
        let ident = self
            .cur_token
            .literal
            .ident()
            .ok_or(vec![ParseErrorKind::InvalidParseFn])?;
        Ok(Expression::Ident(ident.into()))
    }

    fn parse_number(&mut self) -> ParseResult<Expression> {
        let num = self
            .cur_token
            .literal
            .num()
            .ok_or(vec![ParseErrorKind::InvalidParseFn])?;
        Ok(Expression::Number(num))
    }
}

type ParseError = Vec<ParseErrorKind>;
type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnknownPrefixExpr,
    InvalidParseFn,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    Ltgt,
    Sum,
    Prodcut,
    Prefix,
    Call,
}
