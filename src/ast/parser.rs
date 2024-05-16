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
        let mut left = self.prefix().map_err(|e| {
            println!("!!!! PREFIX CALL FOR TOKEN {} !!!!", self.cur_token.ty);
            e
        })?;
        while !self.peek_token_is(TokenType::Semicolon) && prec < self.peek_precedence() {
            if !is_infix_op(self.peek_token.ty) {
                return Ok(left);
            }

            self.next();
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    fn prefix(&mut self) -> ParseResult<Expression> {
        match self.cur_token.ty {
            TokenType::Ident => self.parse_ident(),
            TokenType::Number => self.parse_number(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix(),
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

    fn cur_precedence(&self) -> Precedence {
        token_precedence(self.cur_token.ty)
    }
    fn peek_precedence(&self) -> Precedence {
        token_precedence(self.peek_token.ty)
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

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        let operator = self.cur_token.ty;
        self.next();
        let expr = self.parse_expr(Precedence::Prefix)?;

        Ok(Expression::Prefix(PrefixExpr {
            operator,
            right: Box::new(expr),
        }))
    }

    fn parse_infix(&mut self, left: Expression) -> ParseResult<Expression> {
        let operator = self.cur_token.ty;
        let prec = self.cur_precedence();
        self.next();
        let right = Box::new(self.parse_expr(prec)?);

        Ok(Expression::Infix(InfixExpr {
            left: Box::new(left),
            operator,
            right,
        }))
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

fn token_precedence(ty: TokenType) -> Precedence {
    match ty {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::Ltgt,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Star | TokenType::Slash => Precedence::Prodcut,
        _ => Precedence::Lowest,
    }
}

fn is_infix_op(ty: TokenType) -> bool {
    matches!(
        ty,
        TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Star
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Gt
    )
}
