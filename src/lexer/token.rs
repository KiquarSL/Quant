use std::fmt;
pub type TKind = TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Star,
    Pow,

    Bang,
    Dollar,
    Colon,
    Comma,

    Write,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Assign,
    RArrow,
    LArrow,

    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,

    And,
    Or,

    Id(String),
    Str(String),
    Bool(bool),
    NumFloat(f32),
    NumInt(i32),

    Eof,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TKind,

    pub line: usize,
    pub offset: usize,
    pub len: usize,
    pub pos: usize,
}

impl Token {
    pub fn new(kind: TKind, line: usize, offset: usize, len: usize, pos: usize) -> Self {
        Self {
            kind,
            line,
            offset,
            len,
            pos,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 'token'
        write!(
            f,
            "{}",
            match &self.kind {
                TKind::NumInt(int) => format!("'{int}'"),
                TKind::NumFloat(float) => format!("'{float}'"),
                TKind::Id(id) => format!("{id}"),
                TKind::Str(s) => format!("\"{s}\""),
                TKind::Bool(truth) => format!("'{truth}'"),
                TKind::Plus => "'+'".to_string(),
                TKind::Minus => "'-'".to_string(),
                TKind::Slash => "'/'".to_string(),
                TKind::Star => "'*'".to_string(),
                TKind::Pow => "'^'".to_string(),
                TKind::Bang => "'!'".to_string(),
                TKind::Assign => "'='".to_string(),
                TKind::RArrow => "'=>'".to_string(),
                TKind::LArrow => "'<-'".to_string(),
                TKind::LParen => "'('".to_string(),
                TKind::LBracket => "'['".to_string(),
                TKind::RBracket => "']'".to_string(),
                TKind::Write => "'!?'".to_string(),
                TKind::Dollar => "'$'".to_string(),
                TKind::RParen => "')'".to_string(),
                TKind::And => "'&&'".to_string(),
                TKind::Or => "'||'".to_string(),
                TKind::Colon => "':'".to_string(),
                TKind::Comma => "','".to_string(),
                TKind::LBrace => "'{'".to_string(),
                TKind::RBrace => "'}'".to_string(),
                TKind::Eof => "'\\0'".to_string(),
                TKind::Lt => "'<'".to_string(),
                TKind::Le => "'<='".to_string(),
                TKind::Gt => "'>'".to_string(),
                TKind::Ge => "'>='".to_string(),
                TKind::Eq => "'=='".to_string(),
                TKind::Ne => "'!='".to_string(),
            }
        )
    }
}
