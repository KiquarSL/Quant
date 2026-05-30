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
        if self.pos < self.len && self.peek(0)? == '\n' {
            self.line += 1;
            self.offset = 0;
        } else {
            self.offset += offset as usize;
        }
        self.pos += offset as usize;
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
        let mut current;
        let start_line = self.line;
        let start_offset = self.offset;
        while self.pos < self.len {
            current = self.peek(0)?;
            if !current.is_alphabetic() {
                break;
            }
            self.advance(1)?;
            buffer.push(current);
        }
        let len = buffer.len();
        let kind = match buffer.as_str() {
            "true" => TKind::Bool(true),
            "false" => TKind::Bool(false),
            _ => TKind::Id(buffer),
        };
        self.push(kind, start_line, start_offset, len);
        Ok(())
    }

    fn tokenize_number(&mut self) -> Result<(), CompileError> {
        let mut buffer = String::new();
        let mut current;
        let start_line = self.line;
        let start_offset = self.offset;
        let mut has_dot = false;

        while self.pos < self.len {
            current = self.peek(0)?;
            if current.is_digit(10) || (current == '.' && !has_dot) {
                buffer.push(current);
                if current == '.' {
                    has_dot = true;
                }
                self.advance(1)?;
            } else {
                break;
            }
        }

        let len = buffer.len();
        let kind = if has_dot {
            match buffer.parse::<f32>() {
                Ok(n) => TKind::NumFloat(n),
                Err(err) => {
                    return compilation_error!(
                        CEKind::FailedParse,
                        start_line,
                        start_offset,
                        1,
                        self.current_line(),
                        "Failed parse float value: {err}"
                    );
                }
            }
        } else {
            match buffer.parse::<i32>() {
                Ok(n) => TKind::NumInt(n),
                Err(err) => {
                    return compilation_error!(
                        CEKind::FailedParse,
                        start_line,
                        start_offset,
                        1,
                        self.current_line(),
                        "Failed parse integer value: {err}"
                    );
                }
            }
        };
        self.push(kind, start_line, start_offset, len);
        Ok(())
    }

    fn tokenize_string(&mut self) -> Result<(), CompileError> {
        let mut buffer = String::new();
        let mut current;
        let start_line = self.line;
        let start_offset = self.offset;
        self.advance(1)?;
        while self.pos < self.len {
            current = self.peek(0)?;
            if current == '"' {
                self.advance(1)?;
                break;
            }
            self.advance(1)?;
            buffer.push(current);
        }
        let len = buffer.len();
        self.push(TKind::Str(buffer), start_line, start_offset, len);
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        while self.pos < self.len {
            let current = self.peek(0)?;
            match current {
                c if c.is_whitespace() => self.advance(1)?,
                '(' => {
                    self.push(TKind::LParen, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                ')' => {
                    self.push(TKind::RParen, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '{' => {
                    self.push(TKind::LBrace, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '}' => {
                    self.push(TKind::RBrace, self.line, self.offset, 1);
                    self.advance(1)?;
                }
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
                '^' => {
                    self.push(TKind::Pow, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '/' => {
                    self.push(TKind::Slash, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                ':' => {
                    self.push(TKind::Colon, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                ',' => {
                    self.push(TKind::Comma, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '$' => {
                    self.push(TKind::Dollar, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '[' => {
                    self.push(TKind::LBracket, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                ']' => {
                    self.push(TKind::RBracket, self.line, self.offset, 1);
                    self.advance(1)?;
                }
                '!' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '?' {
                        self.push(TKind::Write, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else if self.pos + 1 < self.len && self.peek(1)? == '=' {
                        self.push(TKind::Ne, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        self.push(TKind::Bang, self.line, self.offset, 1);
                        self.advance(1)?;
                    }
                }
                '=' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '>' {
                        self.push(TKind::RArrow, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else if self.pos + 1 < self.len && self.peek(1)? == '=' {
                        self.push(TKind::Eq, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        self.push(TKind::Assign, self.line, self.offset, 1);
                        self.advance(1)?;
                    }
                }
                '<' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '-' {
                        self.push(TKind::LArrow, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else if self.pos + 1 < self.len && self.peek(1)? == '=' {
                        self.push(TKind::Le, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        self.push(TKind::Lt, self.line, self.offset, 1);
                        self.advance(1)?;
                    }
                }
                '>' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '=' {
                        self.push(TKind::Ge, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        self.push(TKind::Gt, self.line, self.offset, 1);
                        self.advance(1)?;
                    }
                }
                '&' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '&' {
                        self.push(TKind::And, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        return compilation_error!(
                            CEKind::InvalidChar,
                            self.line,
                            self.offset + 1,
                            1,
                            self.current_line(),
                            "Add & for fix",
                        );
                    }
                }
                '|' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '|' {
                        self.push(TKind::Or, self.line, self.offset, 2);
                        self.advance(2)?;
                    } else {
                        return compilation_error!(
                            CEKind::InvalidChar,
                            self.line,
                            self.offset + 1,
                            1,
                            self.current_line(),
                            "Add | for fix",
                        );
                    }
                }
                '#' => {
                    if self.pos + 1 < self.len && self.peek(1)? == '*' {
                        self.advance(2)?;
                        while self.pos + 1 < self.len {
                            if self.peek(0)? == '*' && self.peek(1)? == '#' {
                                self.advance(2)?;
                                break;
                            }
                            self.advance(1)?;
                        }
                    } else {
                        while self.pos < self.len && self.peek(0)? != '\n' {
                            self.advance(1)?;
                        }
                    }
                }
                c if c.is_alphabetic() => self.tokenize_ident()?,
                c if c.is_digit(10) => self.tokenize_number()?,
                c if c == '"' => self.tokenize_string()?,
                _ => {
                    return compilation_error!(
                        CEKind::UnknownChar,
                        self.line,
                        self.offset,
                        1,
                        self.current_line(),
                        "Unknown character {}",
                        current
                    );
                }
            }
        }
        self.push(TKind::Eof, self.line, self.offset, 1);
        Ok(self.tokens.clone())
    }
}
