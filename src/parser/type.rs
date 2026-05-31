use std::fmt;

#[derive(Debug)]
pub enum Type {
    Unknown,
    Int,
    Num,
    Bool,
    Str,
}

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "num" => Type::Num,
            "int" => Type::Int,
            "bool" => Type::Bool,
            "str" => Type::Str,
            _ => Type::Unknown,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Int => "int",
                Type::Num => "num",
                Type::Str => "str",
                Type::Bool => "bool",
                Type::Unknown => "unknown",
            }
        )
    }
}
