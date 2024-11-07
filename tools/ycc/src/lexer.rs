use crate::error::{ErrorLoc, YccError};

#[derive(Debug, Clone)]
pub enum TokenType {
    /// ;
    Semicolon,
    /// =
    Equal,
    /// +=
    AddEqual,
    /// -=
    SubEqual,
    /// *=
    MulEqual,
    /// /=
    DivEqual,
    /// %=
    ModEqual,
    /// ^=
    XorEqual,
    /// |=
    OrEqual,
    /// &=
    AndEqual,
    /// !=
    NotEqual,
    /// ==
    EqualEqual,
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ^
    Xor,
    /// |
    Or,
    /// &
    And,
    /// !
    Not,
    /// unsigned
    Unsigned,
    /// signed
    Signed,
    /// char
    Char,
    /// bool
    Bool,
    /// short
    Short,
    /// int
    Int,
    /// Long
    Long,
    /// float
    Float,
    /// double
    Double,
    /// void
    Void,
    /// (
    LeftParan,
    /// )
    RightParan,
    /// ,
    Comma,
    /// {
    LeftBracket,
    /// }
    RightBracket,
    /// if
    If,
    /// else
    Else,
    /// do
    Do,
    /// goto
    Goto,
    /// switch
    Switch,
    /// case
    Case,
    /// default
    Default,
    /// break
    Break,
    /// continue
    Continue,
    /// return
    Return,
    /// >
    Bigger,
    /// <
    Smaller,
    /// >=
    BiggerOrEqual,
    /// <=
    SmallerOrEqual,
    /// ++
    PlusPlus,
    /// --
    SubSub,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// <<=
    ShiftLeftEqual,
    /// >>=
    ShiftRightEqual,
    /// &&
    AndAnd,
    /// ||
    OrOr,
    /// ~
    BitwiseNot,
    /// [
    LeftSquare,
    /// ]
    RightSquare,
    /// const
    Const,
    /// enum
    Enum,
    /// struct
    Struct,
    /// union
    Union,
    /// typedef
    Typedef,
    /// sizeof
    Sizeof,
    /// extern
    Extern,
    /// static
    Static,
    /// Register
    Register,
    /// volatile
    Volatile,
    /// auto
    Auto,

    /// Identifer
    Ident(String),
    /// Intenger literal
    IntLiteral(i64),
    /// Float literal
    FloatLiteral(f64),
    /// String literal
    StringLiteral(String),
    /// Char literal
    CharLiteral(char),
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
            (Self::IntLiteral(l0), Self::IntLiteral(r0)) => l0 == r0,
            (Self::FloatLiteral(l0), Self::FloatLiteral(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => l0 == r0,
            (Self::CharLiteral(l0), Self::CharLiteral(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for TokenType {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub pos: ErrorLoc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer<'a> {
    tokens: Vec<Token>,

    line: u64,
    col: u64,

    start: u64,
    current: u64,

    code: &'a str,

    pub errors: Vec<YccError>,
    critical_error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            line: 0,
            col: 0,
            start: 0,
            current: 0,
            code: code,
            errors: Vec::new(),
            critical_error: false,
        }
    }

    pub fn lex(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();

            if self.critical_error {
                break;
            }
        }
    }

    fn scan_token(&mut self) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.chars().count() as u64
    }
}