type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    NumInt(i32),
    NumFloat(f32),
    Str(String),
    Id(String),
    Bool(bool),

    Arith(BExpr, ArithOp, BExpr),
    Comp(BExpr, CompOp, BExpr),
    Logic(BExpr, LogicOp, BExpr),
    Unary(UnaryOp, BExpr),
}

#[derive(Debug, Clone)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}
#[derive(Debug, Clone)]
pub enum LogicOp {
    And,
    Or,
}
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}
