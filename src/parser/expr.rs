type BExpr = Box<Expr>;

pub enum Expr {
    NumInt(i32),
    NumFloat(f32),
    Str(String),
    Bool(bool),

    Arith(BExpr, ArithOp, BExpr),
    Comp(BExpr, CompOp, BExpr),
    Logic(BExpr, LogicOp, BExpr),
    Unary(UnaryOp, BExpr),
}

pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub enum CompOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

pub enum LogicOp {
    And,
    Or,
}

pub enum UnaryOp {
    Neg,
    Not,
}
