use colored::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CompileErrorKind {
    InvalidChar,
    InvalidNumber,
    UnknownChar,
    FailedParse,

    UnexpectedToken,
}

#[derive(Debug)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub line: usize,
    pub offset: usize,
    pub len: usize,
    pub message: String,
    pub source_line: String,
}

impl CompileError {
    pub fn new(
        kind: CompileErrorKind,
        line: usize,
        offset: usize,
        len: usize,
        message: String,
        source_line: String,
    ) -> Self {
        Self {
            kind,
            line,
            offset,
            len,
            message,
            source_line,
        }
    }

    pub fn report(&self, file_name: &str) -> String {
        let mut err = String::new();
        let header = format!("Error: {}", self.message).red();
        let file = format!("-> {file_name} in {}:{}", self.line, self.offset).blue();
        let point = self.point();
        err.push_str(&format!("{header}\n{file}\n{point}\n"));
        err
    }

    fn point(&self) -> String {
        let line = format!("{} | {}", self.line, self.source_line);
        let line_num = " ".repeat(self.line.to_string().len());
        let padds = " ".repeat(self.offset);
        let points = "^".repeat(self.len).yellow();
        let err_line = format!("{} | {}{}", line_num, padds, points);
        format!("{line}\n{err_line}")
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl Error for CompileError {}

#[macro_export]
macro_rules! compilation_error {
    ($kind:expr, $line:expr, $offset:expr, $len:expr, $source_line:expr, $($fmt:tt)*) => {
        Err($crate::error::CompileError::new(
            $kind,
            $line,
            $offset,
            $len,
            format!($($fmt)*),
            $source_line.to_string(),
        ))
    };
}
