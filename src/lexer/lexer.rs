use super::token::{TKind, Token};
use crate::compilation_error;
use crate::error::{CompileError, CompileErrorKind};

type CEKind = CompileErrorKind;

pub struct Lexer<'a> {
    chars: Vec<char>,
    tokens: Vec<Token>,
    pos: usize,
    len: usize,

    line: usize,
    offset: usize,
    lines: Vec<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        Self {
            chars: source.chars().collect(),
            tokens: vec![],
            pos: 0,
            len: source.len(),

            line: 0,
            offset: 0,
            lines,
        }
    }

    fn peek(&self, offset: i8) -> Result<char, CompileError> {
        let idx = self.pos + offset as usize;
        let c = self.chars.get(idx);
        match c {
            Some(c) => Ok(*c),
            None => compilation_error!(
                CEKind::InvalidChar,
                self.line,
                self.offset,
                1,
                self.current_line(),
                "Invalid character"
            ),
        }
    }

    fn current_line(&self) -> String {
        self.lines[self.line].to_string()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        while self.pos < self.len {
            return match self.peek(0)? {
                _ => compilation_error!(
                    CEKind::UnknownChar,
                    self.line,
                    self.offset,
                    1,
                    self.current_line(),
                    "Unknown character"
                ),
            };
        }
        todo!();
    }
}
