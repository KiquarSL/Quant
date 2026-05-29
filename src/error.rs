#[derive(Debug)]
pub enum CompileErrorKind {
    InvalidChar,
    UnknownChar,
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
            message,
            offset,
            source_line,
            len,
        }
    }

    pub fn report(&self, file_name: &str) -> String {
        let mut err = String::new();
        err.push_str(&format!("Error: {}\n", self.message));
        err.push_str(&format!("{file_name} in {}:{}\n", self.line, self.offset));
        err.push_str(&self.point());
        err
    }

    fn point(&self) -> String {
        format!(
            "{} | {}\n{} | {}{}\n",
            self.line,
            self.source_line,
            " ".repeat(self.line.to_string().len()),
            " ".repeat(self.offset),
            "^".repeat(self.len)
        )
    }
}

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
