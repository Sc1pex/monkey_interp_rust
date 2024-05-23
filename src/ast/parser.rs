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

        Ok(Statement::Expression(expr))
    }

    fn parse_return(&mut self) -> ParseResult<Statement> {
        self.next(); // Skip 'Return' token

        let expr = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next();
        }

        Ok(Statement::Return(ReturnStmt { expr }))
    }

    fn parse_let(&mut self) -> ParseResult<Statement> {
        self.expect_peek(TokenType::Ident)?;
        let ident: String = self.cur_token.literal.ident().unwrap().into();

        self.expect_peek(TokenType::Assign)?;
        self.next();

        let expr = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next();
        }

        Ok(Statement::Let(LetStmt { ident, expr }))
    }

    fn parse_expr(&mut self, prec: Precedence) -> ParseResult<Expression> {
        let mut left = self.prefix()?;
        while !self.peek_token_is(TokenType::Semicolon) && prec < self.peek_precedence() {
            match self.peek_token.ty {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Star
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::Lt
                | TokenType::Gt => {
                    self.next();
                    left = self.parse_infix(left)?;
                }
                TokenType::LParen => {
                    self.next();
                    left = self.parse_call(left)?;
                }
                TokenType::LBracket => {
                    self.next();
                    left = self.parse_index(left)?;
                }
                _ => return Ok(left),
            }
        }

        Ok(left)
    }

    fn prefix(&mut self) -> ParseResult<Expression> {
        match self.cur_token.ty {
            TokenType::Ident => self.parse_ident(),
            TokenType::Number => self.parse_number(),
            TokenType::String => self.parse_string(),
            TokenType::True | TokenType::False => self.parse_bool(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix(),
            TokenType::LParen => self.parse_group(),
            TokenType::LBracket => self.parse_arr(),
            TokenType::If => self.parse_if(),
            TokenType::Fn => self.parse_func(),
            TokenType::LBrace => self.parse_hash(),
            _ => Err(vec![ParseErrorKind::UnknownPrefixExpr(self.cur_token.ty)]),
        }
    }

    fn next(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.lexer.next());
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
            Err(vec![ParseErrorKind::UnexpectedToken(UnexpectedErr::new(
                ty,
                self.peek_token.ty,
            ))])
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

    fn parse_string(&mut self) -> ParseResult<Expression> {
        let s = self
            .cur_token
            .literal
            .string()
            .ok_or(vec![ParseErrorKind::InvalidParseFn])?;
        Ok(Expression::String(s.into()))
    }

    fn parse_bool(&mut self) -> ParseResult<Expression> {
        match self.cur_token.ty {
            TokenType::True => Ok(Expression::Bool(true)),
            TokenType::False => Ok(Expression::Bool(false)),
            _ => Err(vec![ParseErrorKind::InvalidParseFn]),
        }
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

    fn parse_if(&mut self) -> ParseResult<Expression> {
        self.expect_peek(TokenType::LParen)?;
        self.next();
        let condition = self.parse_expr(Precedence::Lowest)?;
        self.expect_peek(TokenType::RParen)?;
        self.expect_peek(TokenType::LBrace)?;
        self.next();

        let if_branch = self.parse_block()?;

        if self.peek_token_is(TokenType::Else) {
            self.next();
            self.expect_peek(TokenType::LBrace)?;
            self.next();

            let else_branch = self.parse_block()?;

            Ok(Expression::If(IfExpr {
                condition: Box::new(condition),
                if_branch,
                else_branch: Some(else_branch),
            }))
        } else {
            Ok(Expression::If(IfExpr {
                condition: Box::new(condition),
                if_branch,
                else_branch: None,
            }))
        }
    }

    fn parse_func(&mut self) -> ParseResult<Expression> {
        self.expect_peek(TokenType::LParen)?;
        self.next();

        let params = self.parse_params()?;

        self.expect_peek(TokenType::LBrace)?;
        self.next();
        let body = self.parse_block()?;

        Ok(Expression::Func(FuncExpr { params, body }))
    }

    fn parse_hash(&mut self) -> ParseResult<Expression> {
        self.next();

        if self.cur_token_is(TokenType::RBrace) {
            return Ok(Expression::Hash(HashExpr { pairs: vec![] }));
        }

        let key = self.parse_expr(Precedence::Lowest)?;
        self.expect_peek(TokenType::Colon)?;
        self.next();
        let value = self.parse_expr(Precedence::Lowest)?;
        let mut res = vec![(key, value)];

        while self.peek_token_is(TokenType::Comma) {
            self.next();
            self.next();
            let key = self.parse_expr(Precedence::Lowest)?;
            self.expect_peek(TokenType::Colon)?;
            self.next();
            let value = self.parse_expr(Precedence::Lowest)?;
            res.push((key, value));
        }
        self.next();

        Ok(Expression::Hash(HashExpr { pairs: res }))
    }

    fn parse_params(&mut self) -> ParseResult<Vec<Ident>> {
        if self.cur_token_is(TokenType::RParen) {
            return Ok(vec![]);
        }

        let mut res: Vec<Ident> = vec![];
        while self.peek_token_is(TokenType::Comma) {
            let ident =
                self.cur_token
                    .literal
                    .ident()
                    .ok_or(vec![ParseErrorKind::UnexpectedToken(UnexpectedErr::new(
                        TokenType::Ident,
                        self.cur_token.ty,
                    ))])?;
            res.push(ident.into());

            self.expect_peek(TokenType::Comma)?;
            self.next();
        }
        let ident = self
            .cur_token
            .literal
            .ident()
            .ok_or(vec![ParseErrorKind::UnexpectedToken(UnexpectedErr::new(
                TokenType::Ident,
                self.cur_token.ty,
            ))])?;
        res.push(ident.into());
        self.expect_peek(TokenType::RParen)?;

        Ok(res)
    }

    fn parse_block(&mut self) -> ParseResult<Vec<Statement>> {
        let mut statements = vec![];

        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::Eof) {
            let s = self.parse_stmt()?;
            statements.push(s);
            self.next();
        }

        Ok(statements)
    }

    fn parse_call(&mut self, func: Expression) -> ParseResult<Expression> {
        self.next();
        let args = self.parse_expr_list(TokenType::RParen)?;
        Ok(Expression::Call(CallExpr {
            func: Box::new(func),
            arguments: args,
        }))
    }

    fn parse_index(&mut self, left: Expression) -> ParseResult<Expression> {
        self.next();
        let index = self.parse_expr(Precedence::Lowest)?;
        self.expect_peek(TokenType::RBracket)?;

        Ok(Expression::Index(IndexExpr {
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

    fn parse_arr(&mut self) -> ParseResult<Expression> {
        self.next();
        let elements = self.parse_expr_list(TokenType::RBracket)?;
        Ok(Expression::Array(ArrayExpr { elements }))
    }

    fn parse_expr_list(&mut self, end: TokenType) -> ParseResult<Vec<Expression>> {
        if self.cur_token_is(end) {
            return Ok(vec![]);
        }

        let expr = self.parse_expr(Precedence::Lowest)?;
        let mut res = vec![expr];

        while self.peek_token_is(TokenType::Comma) {
            self.next();
            self.next();
            let expr = self.parse_expr(Precedence::Lowest)?;
            res.push(expr);
        }
        self.next();

        Ok(res)
    }

    fn parse_group(&mut self) -> ParseResult<Expression> {
        self.next();

        let exp = self.parse_expr(Precedence::Lowest)?;
        self.expect_peek(TokenType::RParen)?;
        Ok(exp)
    }
}

type ParseError = Vec<ParseErrorKind>;
type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken(UnexpectedErr),
    UnknownPrefixExpr(TokenType),
    InvalidParseFn,
}

#[derive(Debug)]
pub struct UnexpectedErr {
    pub expected: TokenType,
    pub found: TokenType,
}

impl UnexpectedErr {
    pub fn new(expected: TokenType, found: TokenType) -> Self {
        Self { expected, found }
    }
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
    Index,
}

fn token_precedence(ty: TokenType) -> Precedence {
    match ty {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::Ltgt,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Star | TokenType::Slash => Precedence::Prodcut,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
