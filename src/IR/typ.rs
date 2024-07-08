use std::fmt::Display;

/// Stores a type and a value of that type
/// 
/// If you want an empty Type consider using `TypeMetadata`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// Just an u16 with a value
    u16(u16),
    /// Just an u32 with a value
    u32(u32),
    /// Just an u64 with a value
    u64(u64),

    /// Just an i16 with a value
    i16(i16),
    /// Just an i32 with a value
    i32(i32),
    /// Just an i64 with a value
    i64(i64),

    /// Notype
    Void,
}

/// Stores type metadata (just the type without data)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TypeMetadata {
    /// u16
    u16,
    /// u32
    u32,
    /// u64
    u64,

    /// i16
    i16,
    /// i32
    i32,
    /// i64
    i64,

    /// Notype
    Void,
}

impl Type {
    /// Returns the inner value
    pub fn val(&self) -> u64 {
        match self {
            Type::u16(val) => *val as u64,
            Type::u32(val) => *val as u64,
            Type::u64(val) => *val as u64,
            Type::i16(val) => *val as u64,
            Type::i32(val) => *val as u64,
            Type::i64(val) => *val as u64,
            Type::Void => 0,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Type::u16(i) => format!("u16 {}", i),
            Type::u32(i) => format!("u32 {}", i),
            Type::u64(i) => format!("u64 {}", i),
            Type::i16(i) => format!("i16 {}", i),
            Type::i32(i) => format!("i32 {}", i),
            Type::i64(i) => format!("i64 {}", i),
            Type::Void => format!("void"),
        })
    }
}
impl Display for TypeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            TypeMetadata::u16 => "u16",
            TypeMetadata::u32 => "u32",
            TypeMetadata::u64 => "u64",
            TypeMetadata::i16 => "i16",
            TypeMetadata::i32 => "i32",
            TypeMetadata::i64 => "i64",
            TypeMetadata::Void => "void",
        })
    }
}

impl From<Type> for TypeMetadata {
    fn from(value: Type) -> Self {
        match value {
            Type::u16(_) => TypeMetadata::u16,
            Type::u32(_) => TypeMetadata::u32,
            Type::u64(_) => TypeMetadata::u64,
            Type::i16(_) => TypeMetadata::i16,
            Type::i32(_) => TypeMetadata::i32,
            Type::i64(_) => TypeMetadata::i64,
            Type::Void => TypeMetadata::Void,
        }
    }
}

impl From<TypeMetadata> for Type {
    fn from(value: TypeMetadata) -> Self {
        match value {
            TypeMetadata::u16 => Type::u16(0),
            TypeMetadata::u32 => Type::u32(0),
            TypeMetadata::u64 => Type::u64(0),
            TypeMetadata::i16 => Type::i16(0),
            TypeMetadata::i32 => Type::i32(0),
            TypeMetadata::i64 => Type::i64(0),
            TypeMetadata::Void => Type::Void,
        }
    }
}