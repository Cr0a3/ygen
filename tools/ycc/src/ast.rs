#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Extern,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstTypeMeta {
    Bool,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,

    Struct,
    Enum
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstType {
    pub meta: AstTypeMeta,
    pub signed: bool,
    pub unsigned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelStmt {
    Func(FuncStmt),
    Global(GlobalStmt),
    Const(ConstStmt),
    Enum(EnumStmt),
    Struct(StructStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    name: String,
    visibility: Visibility,
    return_type: AstType,
    args: Vec<(String, AstType)>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalStmt {
    name: String,
    visibility: Visibility,
    ty: AstType,
    initializer: Option<Expr>, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstStmt {
    name: String,
    visibility: Visibility,
    ty: AstType,
    initializer: Expr, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumStmt {
    name: String,
    values: Vec<(String, Option<Expr>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructStmt {
    name: String,

    fields: Vec<(String, AstType)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstOperand {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Xor,
    Or,
    And,
    Shl,
    Shr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        ls: Box<Expr>,
        op: AstOperand,
        rs: Box<Expr>,
    },

    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Return {
        value: Expr,
    },
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IntLiteral(l0), Self::IntLiteral(r0)) => l0 == r0,
            (Self::FloatLiteral(l0), Self::FloatLiteral(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => l0 == r0,
            (Self::CharLiteral(l0), Self::CharLiteral(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Expr {}