use super::{BExpr, Info, Parser, Type};
use crate::compilation_error;
use crate::error::{CEKind, CompileError};
use crate::lexer::TKind;
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Declare(String, Type, BExpr, Info),
    Assign(String, AssignOp, BExpr, Info),
}

pub enum StmtKind {
    Declare,
    Assign,
}

impl Stmt {
    pub fn define(pr: &Parser) -> Result<StmtKind, CompileError> {
        let start = pr.peek(0);
        let next = pr.peek(1);
        match (start.kind, next.kind) {
            (TKind::Id(_), TKind::Assign) => Ok(StmtKind::Assign),
            (TKind::Id(_), TKind::Colon) => Ok(StmtKind::Declare),
            _ => {
                return compilation_error!(
                    CEKind::UnknownStatement,
                    start.line,
                    start.offset,
                    start.len,
                    pr.get_line(start.line),
                    "Unknown statement",
                );
            }
        }
    }
}

#[derive(Debug, Default)]
pub enum AssignOp {
    #[default]
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for AssignOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssignOp::Assign => "=",
                AssignOp::Plus => "+=",
                AssignOp::Minus => "-=",
                AssignOp::Slash => "/=",
                AssignOp::Star => "*=",
            }
        )
    }
}
