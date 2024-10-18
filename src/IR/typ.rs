use std::fmt::Display;

/// Stores a type and a value of that type
/// 
/// If you want an empty Type consider using `TypeMetadata`
#[derive(Debug, Clone, Copy)]
pub enum Type {
    /// Just an u8 with a value
    u8(u8),
    /// Just an u16 with a value
    u16(u16),
    /// Just an u32 with a value
    u32(u32),
    /// Just an u64 with a value
    u64(u64),

    /// Just an i8 with a value
    i8(i8),
    /// Just an i16 with a value
    i16(i16),
    /// Just an i32 with a value
    i32(i32),
    /// Just an i64 with a value
    i64(i64),
    /// 64Bit pointer
    ptr(i64),

    /// Notype
    Void,

    /// A f32
    f32(f32),

    /// A f64
    f64(f64),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::u8(l0), Self::u8(r0)) => l0 == r0,
            (Self::u16(l0), Self::u16(r0)) => l0 == r0,
            (Self::u32(l0), Self::u32(r0)) => l0 == r0,
            (Self::u64(l0), Self::u64(r0)) => l0 == r0,
            (Self::i8(l0), Self::i8(r0)) => l0 == r0,
            (Self::i16(l0), Self::i16(r0)) => l0 == r0,
            (Self::i32(l0), Self::i32(r0)) => l0 == r0,
            (Self::i64(l0), Self::i64(r0)) => l0 == r0,
            (Self::ptr(l0), Self::ptr(r0)) => l0 == r0,
            (Self::f32(l0), Self::f32(r0)) => l0 == r0,
            (Self::f64(l0), Self::f64(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Type {}

impl std::hash::Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

/// Stores type metadata (just the type without data)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeMetadata {
    /// u8
    u8,
    /// u16
    u16,
    /// u32
    u32,
    /// u64
    u64,

    /// i8
    i8,
    /// i16
    i16,
    /// i32
    i32,
    /// i64
    i64,
    /// ptr
    ptr,

    /// Notype
    Void,

    /// f32
    f32,

    /// f64
    f64,
}

impl Type {
    /// Returns the inner value
    pub fn val(&self) -> f64 {
        match self {
            Type::u16(val) => *val as f64,
            Type::u32(val) => *val as f64,
            Type::u64(val) => *val as f64,
            Type::i16(val) => *val as f64,
            Type::i32(val) => *val as f64,
            Type::i64(val) => *val as f64,
            Type::ptr(adr) => *adr as f64,
            Type::Void => 0.0,
            Type::u8(val) => *val as f64,
            Type::i8(val) => *val as f64,
            Type::f32(val) => *val as f64,
            Type::f64(val) => *val as f64,
        }
    }

    /// puts the intenger into a type respecting the type metadata
    pub fn from_int(ty: TypeMetadata, value: f64) -> Self {
        match ty {
            TypeMetadata::u16 => Type::u16(value as u16),
            TypeMetadata::u32 => Type::u32(value as u32),
            TypeMetadata::u64 => Type::u64(value as u64),
            TypeMetadata::i16 => Type::i16(value as i16),
            TypeMetadata::i32 => Type::i32(value as i32),
            TypeMetadata::i64 => Type::i64(value as i64),
            TypeMetadata::ptr => Type::ptr(value as i64),
            TypeMetadata::Void => Type::Void,
            TypeMetadata::u8 => Type::u8(value as u8),
            TypeMetadata::i8 => Type::i8(value as i8),
            TypeMetadata::f32 => Type::f32(value as f32),
            TypeMetadata::f64 => Type::f64(value as f64),
        }
    }
}

impl TypeMetadata {
    /// Returns the size of the type in bits
    #[inline]
    pub fn bitSize(&self) -> usize {
        match self {
            TypeMetadata::u8 | TypeMetadata::i8 => 8,
            TypeMetadata::u16 | TypeMetadata::i16 => 16,
            TypeMetadata::u32 | TypeMetadata::i32 => 32,
            TypeMetadata::u64 | TypeMetadata::i64 => 64,
            TypeMetadata::ptr => 64,
            TypeMetadata::Void => 0,

            TypeMetadata::f32 => 4,
            TypeMetadata::f64 => 8,
        }
    }

    /// Returns the size of the type in bytes
    #[inline]
    pub fn byteSize(&self) -> usize {
        if *self != TypeMetadata::Void {
            self.bitSize() / 8
        } else {
            0
        }
    }

    /// Returns if it is a signed type
    #[inline]
    pub fn signed(&self) -> bool {
        match self {
            TypeMetadata::i16 => true,
            TypeMetadata::i32 => true,
            TypeMetadata::i64 => true,

            _ => false,
        }
    }

    /// Returns if it is a float type
    #[inline]
    pub fn float(&self) -> bool {
        match self {
            TypeMetadata::f32 | TypeMetadata::f64 => true,

            _ => false,
        }
    }

    /// returns the parsed typemetadata
    #[inline]
    pub fn parse(string: String) -> Option<Self> {
        match string.as_str() {
            "u8" => Some(TypeMetadata::u8),
            "u16" => Some(TypeMetadata::u16),
            "u32" => Some(TypeMetadata::u32),
            "u64" => Some(TypeMetadata::u64),

            "i8" => Some(TypeMetadata::i8),
            "i16" => Some(TypeMetadata::i16),
            "i32" => Some(TypeMetadata::i32),
            "i64" => Some(TypeMetadata::i64),

            "ptr" => Some(TypeMetadata::ptr),

            "void" => Some(TypeMetadata::Void),

            "f32" => Some(TypeMetadata::f32),
            "f64" => Some(TypeMetadata::f64),

            _ => None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Type::u8(i) => format!("u8 {}", i),
            Type::u16(i) => format!("u16 {}", i),
            Type::u32(i) => format!("u32 {}", i),
            Type::u64(i) => format!("u64 {}", i),

            Type::i8(i) => format!("i8 {}", i),
            Type::i16(i) => format!("i16 {}", i),
            Type::i32(i) => format!("i32 {}", i),
            Type::i64(i) => format!("i64 {}", i),

            Type::ptr(adr) => format!("ptr {:#04x}", adr),
            Type::Void => format!("void"),

            Type::f32(i) => format!("f32 {}", i),
            Type::f64(i) => format!("f64 {}", i),
        })
    }
}

impl Display for TypeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            TypeMetadata::u8 => "u8",
            TypeMetadata::u16 => "u16",
            TypeMetadata::u32 => "u32",
            TypeMetadata::u64 => "u64",

            TypeMetadata::i8 => "i8",
            TypeMetadata::i16 => "i16",
            TypeMetadata::i32 => "i32",
            TypeMetadata::i64 => "i64",

            TypeMetadata::ptr => "ptr",
            TypeMetadata::Void => "void",

            TypeMetadata::f32 => "f32",
            TypeMetadata::f64 => "f64",
        })
    }
}

impl From<Type> for TypeMetadata {
    fn from(value: Type) -> Self {
        match value {
            Type::u8(_) => TypeMetadata::u8,
            Type::u16(_) => TypeMetadata::u16,
            Type::u32(_) => TypeMetadata::u32,
            Type::u64(_) => TypeMetadata::u64,

            Type::i8(_) => TypeMetadata::i8,
            Type::i16(_) => TypeMetadata::i16,
            Type::i32(_) => TypeMetadata::i32,
            Type::i64(_) => TypeMetadata::i64,

            Type::ptr(_) => TypeMetadata::ptr,
            Type::Void => TypeMetadata::Void,

            Type::f32(_) => TypeMetadata::f32,
            Type::f64(_) => TypeMetadata::f64,
        }
    }
}

impl From<TypeMetadata> for Type {
    fn from(value: TypeMetadata) -> Self {
        match value {
            TypeMetadata::u8 => Type::u8(0),
            TypeMetadata::u16 => Type::u16(0),
            TypeMetadata::u32 => Type::u32(0),
            TypeMetadata::u64 => Type::u64(0),

            TypeMetadata::i8 => Type::i8(0),
            TypeMetadata::i16 => Type::i16(0),
            TypeMetadata::i32 => Type::i32(0),
            TypeMetadata::i64 => Type::i64(0),

            TypeMetadata::ptr => Type::ptr(0),
            TypeMetadata::Void => Type::Void,

            TypeMetadata::f32 => Type::f32(0.0),
            TypeMetadata::f64 => Type::f64(0.0),
        }
    }
}

/*
/// A vector type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecType<T> 
    where T: std::fmt::Debug + Clone + PartialEq + Eq
{
    /// the size of the vec
    pub size: T,
    /// if it is a constant vector, the values of the vec
    pub elems: Vec<Type>,
    /// the type of the values of the vec
    pub ty: TypeMetadata,
}

impl<T> Display for VecType<T>
  where T: std::fmt::Debug + Clone + PartialEq + Eq 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}*/