use Ygen::IR::TypeMetadata;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var((String, Option<TypeMetadata>)), // (name, type)
    Binary((Operator, Option<Box<Expr>>, Option<Box<Expr>>)), // (op, left, right)
    LiteralInt(i64),
    LiteralString(String),
    Call(CallStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,

    Assign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Fn(FnStmt),
    Expr(Expr),
    Ret(RetStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnStmt {
    pub name: String,
    pub body: Vec<Statement>,

    pub args: Vec<Expr>,

    pub extrn: bool,
    pub import: bool,
    pub dynamic_args: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetStmt {
    pub var: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallStmt {
    pub name: String,
    pub args: Vec<Expr>,
}

