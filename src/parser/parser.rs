use super::Expr;
use crate::lexer::{TKind, Token};

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

    pub fn parse_expr() {}
}

impl Parser<'_> {
    fn expr(&mut self) -> Expr {
        todo!()
    }
    fn logical(&mut self) -> Expr {
        todo!()
    }
    fn comparison(&mut self) -> Expr {
        todo!()
    }
    fn additive(&mut self) -> Expr {
        todo!()
    }
    fn multiplicative(&mut self) -> Expr {
        todo!()
    }
    fn primary(&mut self) -> Expr {
        todo!()
    }
}
