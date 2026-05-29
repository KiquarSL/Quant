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

    fn advance(&mut self, offset: i8) -> Result<(), CompileError> {
        self.pos += offset as usize;
        if self.peek(0)? == '\n' {
            self.line += 1;
            self.offset = 0;
        } else {
            self.offset += 1;
        }
        Ok(())
    }

    fn current_line(&self) -> String {
        self.lines[self.line].to_string()
    }

    fn push(&mut self, kind: TKind, line: usize, offset: usize, len: usize) {
        self.tokens.push(Token::new(kind, line, offset, len));
    }

    fn tokenize_ident(&mut self) -> Result<(), CompileError> {
        let mut buffer = String::new();
        let mut current = self.peek(0)?;
        let start_line = self.line;
        let start_offset = self.offset;
        while current.is_alphabetic() {
            buffer.push(current);
            self.advance(1)?;
            current = self.peek(0)?;
        }
        let len = buffer.len();
        let kind = match buffer.as_str() {
            "true" | "false" => TKind::Bool(match buffer.parse::<bool>() {
                Ok(ok) => ok,
                Err(err) => {
                    return compilation_error!(
                        CEKind::FailedParse,
                        start_line,
                        start_offset,
                        1,
                        self.current_line(),
                        "Failed parse boolean value: {err}"
                    );
                }
            }),
            _ => TKind::Id(buffer),
        };
        self.push(kind, start_line, start_offset, len);
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        while self.pos < self.len {
            let current = self.peek(0)?;
            match current {
                c if c.is_whitespace() => self.advance(1)?,
                '+' => {
                    self.push(TKind::Plus, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '-' => {
                    self.push(TKind::Minus, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '*' => {
                    self.push(TKind::Star, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '/' => {
                    self.push(TKind::Slash, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '=' => {
                    self.push(TKind::Assign, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                c if c.is_alphabetic() => self.tokenize_ident()?,
                _ => {
                    return compilation_error!(
                        CEKind::UnknownChar,
                        self.line,
                        self.offset,
                        1,
                        self.current_line(),
                        "Unknown character '{current}'"
                    );
                }
            }
        }
        self.push(TKind::Eof, self.line, self.offset, 0);
        todo!();
    }
}
