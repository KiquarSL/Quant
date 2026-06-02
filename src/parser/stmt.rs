use super::{BExpr, Info, Parser, Type};
use crate::compilation_error;
use crate::error::{CEKind, CompileError};
use crate::lexer::TKind;
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Declare(String, Type, BExpr, Info),
    Assign(String, AssignOp, BExpr, Info),
    Write(Vec<BExpr>, Info),
    WhileLoop(BExpr, Vec<Stmt>, Info),
}

pub enum StmtKind {
    Declare,
    Assign,
    Write,
    WhileLoop,
}

impl Stmt {
    pub fn define(pr: &Parser) -> Result<StmtKind, CompileError> {
        let start = pr.peek(0);
        let next = pr.peek(1);
        match (start.kind, next.kind) {
            (TKind::Id(_), TKind::Colon) => Ok(StmtKind::Declare),
            (TKind::Id(_), TKind::Assign) => Ok(StmtKind::Assign),
            (TKind::LBracket, _) => Ok(StmtKind::WhileLoop),
            (TKind::Write, _) => Ok(StmtKind::Write),
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

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Stmt::Declare(id, ty, val, info) => format!("{id}: {ty} = {val} ({info})"),
                Stmt::Assign(id, assign, val, info) => format!("{id} {assign} {val} ({info})"),
                Stmt::Write(args, info) => {
                    let args = args
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("!? {args} ({info})")
                }
                Stmt::WhileLoop(cond, body, info) => {
                    let body = body
                        .iter()
                        .map(|i| "  ".to_owned() + &i.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!("[ {cond} ] {{\n{body}\n}} ({info})")
                }
            }
        )
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
