use std::fmt;
pub type TKind = TokenKind;

pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Star,

    Id(String),
    Bool(bool),
    NumFloat(f64),
    NumInt(i64),

    Eof,
}

pub struct Token {
    kind: TKind,

    line: usize,
    offset: usize,
    len: usize,
}

impl Token {
    fn new(kind: TKind, line: usize, offset: usize, len: usize) -> Self {
        Self {
            kind,
            line,
            offset,
            len,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NumInt (n) in l:o
        write!(
            f,
            "{} in {}:{}",
            match &self.kind {
                TKind::NumInt(int) => format!("'{int}'"),
                TKind::NumFloat(float) => format!("'{float}'"),
                TKind::Id(id) => format!("Identificator '{}'", id),
                TKind::Bool(truth) => format!("'{truth}'"),
                TKind::Plus => format!("'+'"),
                TKind::Minus => format!("'-'"),
                TKind::Slash => format!("'/'"),
                TKind::Star => format!("'*'"),
                TKind::Eof => format!("'\\0'"),
            },
            self.line,
            self.offset
        )
    }
}
