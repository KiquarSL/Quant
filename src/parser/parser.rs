use super::{ArithOp, AssignOp, BExpr, CompOp, Expr, LogicOp, Stmt, StmtKind, Type, UnaryOp};
use crate::error::{CEKind, CompileError};
use crate::lexer::{TKind, Token};
use crate::{compilation_error, info};
use std::mem::discriminant;

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<Token>,
    lines: Vec<&'a str>,
}

const EMPTY: String = String::new();

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a str) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        Self {
            tokens,
            pos: 0,
            lines,
        }
    }

    pub fn parse_expr(&mut self) -> Result<Vec<Expr>, CompileError> {
        let mut exprs = vec![];
        while self.peek(0).kind != TKind::Eof {
            let expr = self.expr();
            match expr {
                Ok(ok) => exprs.push(ok),
                Err(err) => return Err(err),
            }
        }
        Ok(exprs)
    }

    pub fn parse_stmt(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = vec![];
        while self.peek(0).kind != TKind::Eof {
            let stmt = self.stmt();
            match stmt {
                Ok(ok) => stmts.push(ok),
                Err(err) => return Err(err),
            }
        }
        Ok(stmts)
    }

    fn check(&mut self, kind: TKind) -> bool {
        if discriminant(&self.peek(0).kind) == discriminant(&kind) {
            self.advance(1);
            true
        } else {
            false
        }
    }

    pub fn peek(&self, offset: i8) -> Token {
        let idx = self.pos + offset as usize;
        self.tokens[idx].clone()
    }

    pub fn get_line(&self, line: usize) -> String {
        self.lines[line].to_string()
    }

    fn advance(&mut self, offset: i8) {
        self.pos += offset as usize;
    }

    fn parse_type(&mut self) -> Result<Type, CompileError> {
        let tkn = self.peek(0);
        match tkn.kind {
            TKind::Id(id) => {
                self.advance(1);
                Ok(Type::from_str(&id))
            }
            _ => compilation_error!(
                CEKind::InvalidType,
                tkn,
                self.get_line(tkn.line),
                "Invalid type"
            ),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<BExpr>, CompileError> {
        let mut args = vec![];
        while self.peek(0).kind != TKind::Eof {
            let arg = self.expr()?;
            args.push(Box::new(arg));
            if !self.check(TKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn expected_token<T>(
        &mut self,
        expected_kind: TKind,
        expected: &str,
        success: &dyn Fn(Token) -> Result<T, CompileError>,
    ) -> Result<T, CompileError> {
        let token = self.peek(0);
        if !self.check(expected_kind) {
            compilation_error!(
                CEKind::ExpectedToken,
                token,
                self.get_line(token.line),
                "Expected '{expected}', found {token}",
            )
        } else {
            Ok(success(token)?)
        }
    }

    pub fn parse_body(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut body = vec![];
        while self.peek(0).kind != TKind::Eof && self.peek(0).kind != TKind::RBrace {
            let expr = self.stmt();
            match expr {
                Ok(ok) => body.push(ok),
                Err(err) => return Err(err),
            }
        }
        Ok(body)
    }
}

impl Parser<'_> {
    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.logical()
    }

    fn logical(&mut self) -> Result<Expr, CompileError> {
        let start = self.peek(0);
        let mut left = self.comparison()?;
        loop {
            let op_tkn = self.peek(0);
            let op = match op_tkn.kind {
                TKind::And => LogicOp::And(info!(op_tkn)),
                TKind::Or => LogicOp::Or(info!(op_tkn)),
                _ => break,
            };
            self.advance(1);
            let right = self.comparison()?;
            left = Expr::Logic(Box::new(left), op, Box::new(right), info!(start));
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, CompileError> {
        let start = self.peek(0);
        let mut left = self.additive()?;
        loop {
            let op_tkn = self.peek(0);
            let op = match op_tkn.kind {
                TKind::Gt => CompOp::Gt(info!(op_tkn)),
                TKind::Ge => CompOp::Ge(info!(op_tkn)),
                TKind::Lt => CompOp::Lt(info!(op_tkn)),
                TKind::Le => CompOp::Le(info!(op_tkn)),
                TKind::Eq => CompOp::Eq(info!(op_tkn)),
                TKind::Ne => CompOp::Ne(info!(op_tkn)),
                _ => break,
            };
            self.advance(1);
            let right = self.additive()?;
            left = Expr::Comp(Box::new(left), op, Box::new(right), info!(start));
        }
        Ok(left)
    }

    fn additive(&mut self) -> Result<Expr, CompileError> {
        let start = self.peek(0);
        let mut left = self.multiplicative()?;
        loop {
            let op_tkn = self.peek(0);
            let op = match op_tkn.kind {
                TKind::Plus => ArithOp::Add(info!(op_tkn)),
                TKind::Minus => ArithOp::Sub(info!(op_tkn)),
                _ => break,
            };
            self.advance(1);
            let right = self.multiplicative()?;
            left = Expr::Arith(Box::new(left), op, Box::new(right), info!(start));
        }
        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expr, CompileError> {
        let start = self.peek(0);
        let mut left = self.power()?;
        loop {
            let tkn = self.peek(0);
            match tkn.kind {
                TKind::Star => {
                    self.advance(1);
                    let right = self.power()?;
                    left = Expr::Arith(
                        Box::new(left),
                        ArithOp::Mul(info!(tkn)),
                        Box::new(right),
                        info!(start),
                    );
                }
                TKind::Slash => {
                    self.advance(1);
                    let right = self.power()?;
                    left = Expr::Arith(
                        Box::new(left),
                        ArithOp::Div(info!(tkn)),
                        Box::new(right),
                        info!(start),
                    );
                }
                TKind::LParen => {
                    self.advance(1);
                    let right = self.expr()?;
                    if !self.check(TKind::RParen) {
                        return compilation_error!(
                            CEKind::ExpectedToken,
                            tkn,
                            self.get_line(tkn.line),
                            "Not found closed ')'"
                        );
                    }
                    left = Expr::Arith(
                        Box::new(left),
                        ArithOp::Mul(info!(tkn)),
                        Box::new(right),
                        info!(start),
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn power(&mut self) -> Result<Expr, CompileError> {
        let start = self.peek(0);
        let mut left = self.unary()?;
        let pow_tkn = self.peek(0);
        if pow_tkn.kind == TKind::Pow {
            self.advance(1);
            let right = self.power()?;
            left = Expr::Arith(
                Box::new(left),
                ArithOp::Pow(info!(pow_tkn)),
                Box::new(right),
                info!(start),
            );
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, CompileError> {
        let tkn = self.peek(0);
        match tkn.kind {
            TKind::Bang => {
                self.advance(1);
                Ok(Expr::Unary(
                    UnaryOp::Not(info!(tkn)),
                    Box::new(self.primary()?),
                    info!(tkn),
                ))
            }
            TKind::Minus => {
                self.advance(1);
                Ok(Expr::Unary(
                    UnaryOp::Neg(info!(tkn)),
                    Box::new(self.primary()?),
                    info!(tkn),
                ))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, CompileError> {
        let tkn = self.peek(0);
        match tkn.kind {
            TKind::NumInt(n) => {
                self.advance(1);
                Ok(Expr::NumInt(n, info!(tkn)))
            }
            TKind::NumFloat(n) => {
                self.advance(1);
                Ok(Expr::NumFloat(n, info!(tkn)))
            }
            TKind::Bool(truth) => {
                self.advance(1);
                Ok(Expr::Bool(truth, info!(tkn)))
            }
            TKind::Id(id) => {
                self.advance(1);
                Ok(Expr::Id(id, info!(tkn)))
            }
            TKind::Str(s) => {
                self.advance(1);
                Ok(Expr::Str(s, info!(tkn)))
            }
            TKind::LParen => {
                self.advance(1);
                let expr = self.expr()?;
                if !self.check(TKind::RParen) {
                    return compilation_error!(
                        CEKind::ExpectedToken,
                        tkn,
                        self.get_line(tkn.line),
                        "Not found closed ')'"
                    );
                }
                Ok(expr)
            }
            _ => {
                return compilation_error!(
                    CEKind::UnexpectedToken,
                    tkn,
                    self.get_line(tkn.line),
                    "Unexpected token {}",
                    tkn
                );
            }
        }
    }
}

impl Parser<'_> {
    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        match Stmt::define(self)? {
            StmtKind::Assign => Ok(self.stmt_assign()?),
            StmtKind::Declare => Ok(self.stmt_declare()?),
            StmtKind::Write => Ok(self.stmt_write()?),
            StmtKind::WhileLoop => Ok(self.stmt_while_loop()?),
        }
    }

    fn stmt_while_loop(&mut self) -> Result<Stmt, CompileError> {
        let start = self.peek(0);
        self.expected_token(TKind::LBracket, "[", &|_t| Ok(()))?;
        let cond = self.expr()?;
        self.expected_token(TKind::RBracket, "]", &|_t| Ok(()))?;
        self.expected_token(TKind::LBrace, "{", &|_t| Ok(()))?;
        let body = self.parse_body()?;
        self.expected_token(TKind::RBrace, "}", &|_t| Ok(()))?;
        let end = self.peek(0);
        Ok(Stmt::WhileLoop(
            Box::new(cond),
            body,
            info!(start.line, start.offset, end.pos - 1 - start.pos),
        ))
    }

    fn stmt_write(&mut self) -> Result<Stmt, CompileError> {
        let start = self.peek(0);
        self.expected_token(TKind::Write, "!?", &|_token| Ok(()))?;
        let args = self.parse_args()?;
        let end = self.peek(0);
        Ok(Stmt::Write(
            args,
            info!(start.line, start.offset, end.pos - 1 - start.pos),
        ))
    }

    fn stmt_assign(&mut self) -> Result<Stmt, CompileError> {
        let start = self.peek(0);
        // EMPTY - Fictitious value, using for type TKind::Id indication only
        let id = self.expected_token(TKind::Id(EMPTY), "ident", &|t: Token| match t.kind {
            TKind::Id(id) => Ok(id),
            _ => unreachable!(),
        })?;
        // temporarily only '='
        let assign = self.expected_token(TKind::Assign, "=", &|_t| Ok(AssignOp::default()))?;

        let value = self.expr()?;
        let end = self.peek(0);
        Ok(Stmt::Assign(
            id,
            assign,
            Box::new(value.clone()),
            info!(start.line, start.offset, end.pos - 1 - start.pos),
        ))
    }

    fn stmt_declare(&mut self) -> Result<Stmt, CompileError> {
        let start = self.peek(0);
        let id = self.expected_token(TKind::Id(EMPTY), "ident", &|t: Token| match t.kind {
            TKind::Id(id) => Ok(id),
            _ => unreachable!(),
        })?;
        self.expected_token(TKind::Colon, ":", &|_token| Ok(()))?;

        let ty = self.parse_type()?;
        self.expected_token(TKind::Assign, "=", &|_token| Ok(()))?;

        let value = self.expr()?;
        let end = self.peek(0);
        Ok(Stmt::Declare(
            id,
            ty,
            Box::new(value.clone()),
            info!(start.line, start.offset, end.pos - 1 - start.pos),
        ))
    }
}
