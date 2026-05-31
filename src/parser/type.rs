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
