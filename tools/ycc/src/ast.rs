#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Extern,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstTypeMeta {
    Void,

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
    pub name: String,
    pub visibility: Visibility,
    pub return_type: AstType,
    pub args: Vec<(String, AstType)>,
    pub body: Vec<Stmt>,
    pub only_ty_indector: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalStmt {
    pub name: String,
    pub visibility: Visibility,
    pub ty: AstType,
    pub initializer: Option<Expr>, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstStmt {
    pub name: String,
    pub visibility: Visibility,
    pub ty: AstType,
    pub initializer: Expr, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumStmt {
    pub name: String,
    pub values: Vec<(String, Option<Expr>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructStmt {
    pub name: String,

    pub fields: Vec<(String, AstType)>,
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
    Block {
        body: Vec<Stmt>
    }
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