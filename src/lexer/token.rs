use std::fmt;
pub type TKind = TokenKind;

#[derive(Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Star,

    Assign,

    Id(String),
    Bool(bool),
    NumFloat(f32),
    NumInt(i32),

    Eof,
}

#[derive(Clone)]
pub struct Token {
    kind: TKind,

    line: usize,
    offset: usize,
    len: usize,
}

impl Token {
    pub fn new(kind: TKind, line: usize, offset: usize, len: usize) -> Self {
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
                TKind::Assign => format!("'='"),
                TKind::Eof => format!("'\\0'"),
            },
            self.line,
            self.offset
        )
    }
}
