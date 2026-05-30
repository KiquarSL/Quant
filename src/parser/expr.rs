type BExpr = Box<Expr>;
use strum_macros::Display;

#[derive(Debug, Clone, Display)]
pub enum Expr {
    #[strum(to_string = "{0}")]
    NumInt(i32),
    #[strum(to_string = "{0}")]
    NumFloat(f32),
    #[strum(to_string = "\"{0}\"")]
    Str(String),
    #[strum(to_string = "{0}")]
    Id(String),
    #[strum(to_string = "{0}")]
    Bool(bool),

    #[strum(to_string = "({0} {1} {2})")]
    Arith(BExpr, ArithOp, BExpr),
    #[strum(to_string = "({0} {1} {2})")]
    Comp(BExpr, CompOp, BExpr),
    #[strum(to_string = "({0} {1} {2})")]
    Logic(BExpr, LogicOp, BExpr),
    #[strum(to_string = "{0}{1}")]
    Unary(UnaryOp, BExpr),
}

#[derive(Debug, Clone, Display)]
pub enum ArithOp {
    #[strum(to_string = "+")]
    Add,
    #[strum(to_string = "-")]
    Sub,
    #[strum(to_string = "*")]
    Mul,
    #[strum(to_string = "/")]
    Div,
    #[strum(to_string = "^")]
    Pow,
}

#[derive(Debug, Clone, Display)]
pub enum CompOp {
    #[strum(to_string = ">")]
    Gt,
    #[strum(to_string = ">=")]
    Ge,
    #[strum(to_string = "<")]
    Lt,
    #[strum(to_string = "<=")]
    Le,
    #[strum(to_string = "==")]
    Eq,
    #[strum(to_string = "!=")]
    Ne,
}
#[derive(Debug, Clone, Display)]
pub enum LogicOp {
    #[strum(to_string = "&&")]
    And,
    #[strum(to_string = "||")]
    Or,
}
#[derive(Debug, Clone, Display)]
pub enum UnaryOp {
    #[strum(to_string = "-")]
    Neg,
    #[strum(to_string = "!")]
    Not,
}
