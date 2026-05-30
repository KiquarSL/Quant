use super::{ArithOp, CompOp, Expr, LogicOp, UnaryOp};
use crate::compilation_error;
use crate::error::{CompileError, CompileErrorKind};
use crate::lexer::{TKind, Token};

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
        let mut left = self.comparison()?;
        loop {
            let op = match self.peek(0).kind {
                TKind::And => LogicOp::And,
                TKind::Or => LogicOp::Or,
                _ => break,
            };
            self.advance(1);
            let right = self.comparison()?;
            left = Expr::Logic(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.additive()?;
        loop {
            let op = match self.peek(0).kind {
                TKind::Gt => CompOp::Gt,
                TKind::Ge => CompOp::Ge,
                TKind::Lt => CompOp::Lt,
                TKind::Le => CompOp::Le,
                TKind::Eq => CompOp::Eq,
                TKind::Ne => CompOp::Ne,
                _ => break,
            };
            self.advance(1);
            let right = self.additive()?;
            left = Expr::Comp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn additive(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.multiplicative()?;
        loop {
            let op = match self.peek(0).kind {
                TKind::Plus => ArithOp::Add,
                TKind::Minus => ArithOp::Sub,
                _ => break,
            };
            self.advance(1);
            let right = self.multiplicative()?;
            left = Expr::Arith(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.power()?;
        loop {
            let tkn = self.peek(0);
            let op = match tkn.kind {
                TKind::Star => ArithOp::Mul,
                TKind::Slash => ArithOp::Div,
                TKind::LParen => {
                    self.advance(1);
                    left = Expr::Arith(Box::new(left), ArithOp::Mul, Box::new(self.expr()?));
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
                    continue;
                }
                _ => break,
            };
            self.advance(1);
            let right = self.power()?;
            left = Expr::Arith(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn power(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.unary()?;
        if self.peek(0).kind == TKind::Pow {
            self.advance(1);
            let right = self.power()?;
            left = Expr::Arith(Box::new(left), ArithOp::Pow, Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, CompileError> {
        let tkn = self.peek(0);
        match tkn.kind {
            TKind::Bang => {
                self.advance(1);
                Ok(Expr::Unary(UnaryOp::Not, Box::new(self.primary()?)))
            }
            TKind::Minus => {
                self.advance(1);
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(self.primary()?)))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, CompileError> {
        let tkn = self.peek(0);
        match tkn.kind {
            TKind::NumInt(n) => {
                self.advance(1);
                Ok(Expr::NumInt(n))
            }
            TKind::NumFloat(n) => {
                self.advance(1);
                Ok(Expr::NumFloat(n))
            }
            TKind::Bool(truth) => {
                self.advance(1);
                Ok(Expr::Bool(truth))
            }
            TKind::Id(id) => {
                self.advance(1);
                Ok(Expr::Id(id))
            }
            TKind::Str(s) => {
                self.advance(1);
                Ok(Expr::Str(s))
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
