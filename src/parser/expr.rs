use super::Info;
use std::fmt;

pub type BExpr = Box<Expr>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NumInt(i32, Info),
    NumFloat(f32, Info),
    Str(String, Info),
    Id(String, Info),
    Bool(bool, Info),
    Arith(BExpr, ArithOp, BExpr, Info),
    Comp(BExpr, CompOp, BExpr, Info),
    Logic(BExpr, LogicOp, BExpr, Info),
    Unary(UnaryOp, BExpr, Info),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::NumInt(n, _) => write!(f, "{}", n),
            Expr::NumFloat(n, _) => write!(f, "{}", n),
            Expr::Str(s, _) => write!(f, "\"{}\"", s),
            Expr::Id(id, _) => write!(f, "{}", id),
            Expr::Bool(b, _) => write!(f, "{}", b),
            Expr::Arith(l, op, r, _) => write!(f, "({} {} {})", l, op, r),
            Expr::Comp(l, op, r, _) => write!(f, "({} {} {})", l, op, r),
            Expr::Logic(l, op, r, _) => write!(f, "({} {} {})", l, op, r),
            Expr::Unary(op, expr, _) => write!(f, "{}{}", op, expr),
        }
    }
}

impl Expr {
    pub fn info(&self) -> Info {
        match self {
            Expr::NumInt(_, info) => info.clone(),
            Expr::NumFloat(_, info) => info.clone(),
            Expr::Str(_, info) => info.clone(),
            Expr::Id(_, info) => info.clone(),
            Expr::Bool(_, info) => info.clone(),
            Expr::Arith(_, _, _, info) => info.clone(),
            Expr::Comp(_, _, _, info) => info.clone(),
            Expr::Logic(_, _, _, info) => info.clone(),
            Expr::Unary(_, _, info) => info.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithOp {
    Add(Info),
    Sub(Info),
    Mul(Info),
    Div(Info),
    Pow(Info),
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithOp::Add(_) => write!(f, "+"),
            ArithOp::Sub(_) => write!(f, "-"),
            ArithOp::Mul(_) => write!(f, "*"),
            ArithOp::Div(_) => write!(f, "/"),
            ArithOp::Pow(_) => write!(f, "^"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompOp {
    Gt(Info),
    Ge(Info),
    Lt(Info),
    Le(Info),
    Eq(Info),
    Ne(Info),
}

impl fmt::Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompOp::Gt(_) => write!(f, ">"),
            CompOp::Ge(_) => write!(f, ">="),
            CompOp::Lt(_) => write!(f, "<"),
            CompOp::Le(_) => write!(f, "<="),
            CompOp::Eq(_) => write!(f, "=="),
            CompOp::Ne(_) => write!(f, "!="),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicOp {
    And(Info),
    Or(Info),
}

impl fmt::Display for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicOp::And(_) => write!(f, "&&"),
            LogicOp::Or(_) => write!(f, "||"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg(Info),
    Not(Info),
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg(_) => write!(f, "-"),
            UnaryOp::Not(_) => write!(f, "!"),
        }
    }
}
