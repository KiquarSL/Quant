use super::{ArithOp, CompOp, Expr, Info, LogicOp, UnaryOp};
use crate::error::{CompileError, CompileErrorKind};
use crate::lexer::{TKind, Token};
use crate::{compilation_error, info};

type CEKind = CompileErrorKind;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    lines: Vec<&'a str>,
}

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

    fn check(&mut self, kind: TKind) -> bool {
        if self.peek(0).kind == kind {
            self.advance(1);
            true
        } else {
            false
        }
    }

    fn peek(&self, offset: i8) -> Token {
        let idx = self.pos + offset as usize;
        self.tokens[idx].clone()
    }

    fn get_line(&self, line: usize) -> String {
        self.lines[line].to_string()
    }

    fn advance(&mut self, offset: i8) {
        self.pos += offset as usize;
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
                            tkn.line,
                            tkn.offset,
                            tkn.len,
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
                        tkn.line,
                        tkn.offset,
                        tkn.len,
                        self.get_line(tkn.line),
                        "Not found closed ')'"
                    );
                }
                Ok(expr)
            }
            _ => {
                return compilation_error!(
                    CEKind::UnexpectedToken,
                    tkn.line,
                    tkn.offset,
                    tkn.len,
                    self.get_line(tkn.line),
                    "Unexpected token {}",
                    tkn
                );
            }
        }
    }
}
