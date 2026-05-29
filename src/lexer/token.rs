use std::fmt;
pub type TKind = TokenKind;

#[derive(Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Star,

    LParen,
    RParen,
    LBrace,
    RBrace,

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
        // 'n' in l:o
        write!(
            f,
            "{} in {}:{}",
            match &self.kind {
                TKind::NumInt(int) => format!("'{int}'"),
                TKind::NumFloat(float) => format!("'{float}'"),
                TKind::Id(id) => format!("Ident '{}'", id),
                TKind::Bool(truth) => format!("'{truth}'"),
                TKind::Plus => "'+'".to_string(),
                TKind::Minus => "'-'".to_string(),
                TKind::Slash => "'/'".to_string(),
                TKind::Star => "'*'".to_string(),
                TKind::Assign => "'='".to_string(),
                TKind::LParen => "'('".to_string(),
                TKind::RParen => "')'".to_string(),
                TKind::LBrace => "'{'".to_string(),
                TKind::RBrace => "'}'".to_string(),
                TKind::Eof => "'\\0'".to_string(),
            },
            self.line,
            self.offset
        )
    }
}
