mod expr;
mod parser;
mod stmt;
mod r#type;

pub use expr::{ArithOp, BExpr, CompOp, Expr, LogicOp, UnaryOp};
pub use parser::Parser;
pub use stmt::{AssignOp, Stmt};
pub use r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub line: usize,
    pub offset: usize,
    pub len: usize,
}

#[macro_export]
macro_rules! info {
    ($tkn:expr) => {
        info!($tkn.line, $tkn.offset, $tkn.len)
    };
    ($line:expr, $offset:expr, $len:expr) => {
        $crate::parser::Info {
            line: $line,
            offset: $offset,
            len: $len,
        }
    };
}
