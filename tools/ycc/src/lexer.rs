use std::{char, collections::HashMap};

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
    Star,
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

    keywords: HashMap<&'static str, TokenType>
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut keywords = HashMap::new();
        
        // types

        keywords.insert("unsigned", TokenType::Unsigned);
        keywords.insert("signed", TokenType::Signed);
        keywords.insert("char", TokenType::Char);
        keywords.insert("bool", TokenType::Bool);
        keywords.insert("short", TokenType::Short);
        keywords.insert("int", TokenType::Int);
        keywords.insert("long", TokenType::Long);
        keywords.insert("float", TokenType::Float);
        keywords.insert("double", TokenType::Double);
        keywords.insert("void", TokenType::Void);

        // keywords

        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("do", TokenType::Do);
        keywords.insert("goto", TokenType::Goto);
        keywords.insert("switch", TokenType::Switch);
        keywords.insert("case", TokenType::Case);
        keywords.insert("default", TokenType::Default);
        keywords.insert("break", TokenType::Break);
        keywords.insert("continue", TokenType::Continue);
        keywords.insert("return", TokenType::Return);
        keywords.insert("const", TokenType::Const);
        keywords.insert("enum", TokenType::Enum);
        keywords.insert("struct", TokenType::Struct);
        keywords.insert("union", TokenType::Union);
        keywords.insert("typedef", TokenType::Typedef);
        keywords.insert("sizeof", TokenType::Sizeof);
        keywords.insert("extern", TokenType::Extern);
        keywords.insert("static", TokenType::Static);
        keywords.insert("register", TokenType::Register);
        keywords.insert("volatile", TokenType::Volatile);
        keywords.insert("auto", TokenType::Auto);

        Self {
            tokens: Vec::new(),
            line: 0,
            col: 0,
            start: 0,
            current: 0,
            code: code,
            errors: Vec::new(),
            critical_error: false,
            keywords: keywords,
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

    fn scan_token(&mut self) -> Option<()> {
        let mut token = None;

        let char = self.advance()?;

        if char.is_whitespace() {
            return Some(());
        }

        match char {
            ';' => token = self.token(TokenType::Semicolon),
            '(' => token = self.token(TokenType::LeftParan),
            ')' => token = self.token(TokenType::RightParan),
            ',' => token = self.token(TokenType::Comma),
            '{' => token = self.token(TokenType::LeftBracket),
            '}' => token = self.token(TokenType::RightBracket),
            '[' => token = self.token(TokenType::LeftSquare),
            ']' => token = self.token(TokenType::RightSquare),

            '=' => token = self.parse_equal(),
            '+' => token = self.parse_add(),
            '-' => token = self.parse_sub(),
            '*' => token = self.parse_mul(),
            '/' => token = self.parse_div(),
            '%' => token = self.parse_mod(),
            '^' => token = self.parse_xor(),
            '|' => token = self.parse_or(),
            '&' => token = self.parse_and(),
            '!' => token = self.parse_not(),

            '>' => token = self.parse_bigger(),
            '<' => token = self.parse_smaller(),

            '~' => token = self.token(TokenType::BitwiseNot),

            'a'..='z' | 'A'..='Z' | '_' => token = self.parse_ident(),
            '0'..='9' => token = self.parse_num(),
            '\'' => token = self.parse_char(),
            '\"' => token = self.parse_string(),

            unexpected => self.errors.push(YccError {
                loc: self.construct_loc(),
                head: "unexpected token",
                where_string: format!("unexpected token: {unexpected}"),
            }),
        }

        if let Some(token) = token {
            self.tokens.push(token);
        }

        Some(())
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            self.errors.push(YccError {
                loc: self.construct_loc(),
                head: "lexer ran out of tokens",
                where_string: "the lexer ran out of tokens".to_owned(),
            });
            self.critical_error = true;

            return None;
        }

        let Some(char) = self.code.chars().nth(self.current as usize) else { panic!() };

        if char == '\n' { self.line += 1; self.col = 0; }
        else { self.col += 1; }

        self.current += 1;

        Some(char)
    }

    #[inline]
    fn last(&self) -> Option<char> {
        self.code.chars().nth(self.current as usize - 1)
    }

    #[inline]
    fn current(&self) -> Option<char> {
        self.code.chars().nth(self.current as usize)
    }

    #[inline]
    fn next(&self) -> Option<char> {
        self.code.chars().nth(self.current as usize + 1)
    }

    #[inline]
    fn token(&self, ty: TokenType) -> Option<Token> {
        Some(Token {
            ty: ty,
            pos: self.construct_loc(),
        })
    }

    fn construct_loc(&self) -> ErrorLoc {
        ErrorLoc {
            line: self.line,
            col: self.col,
            length: self.current - self.start,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.chars().count() as u64
    }

    #[inline]
    fn parse_equal(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::EqualEqual)
        } else {
            self.token(TokenType::Equal)
        }
    }

    #[inline]
    fn parse_add(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::AddEqual)
        } else if let Some('+') = next {
            self.advance();
            self.token(TokenType::PlusPlus)
        } else {
            self.token(TokenType::Add)
        }
    }

    #[inline]
    fn parse_sub(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::SubEqual)
        } else if let Some('-') = next {
            self.advance();
            self.token(TokenType::SubSub)
        } else {
            self.token(TokenType::Sub)
        }
    }

    #[inline]
    fn parse_mul(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::MulEqual)
        } else {
            self.token(TokenType::Star)
        }
    }

    #[inline]
    fn parse_div(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::DivEqual)
        } else if let Some('/') = next {
            self.parse_comment()
        } else if let Some('*') = next {
            self.advance();
            self.parse_block_comment() 
        } else {
            self.token(TokenType::Div)
        }
    }

    fn parse_comment(&mut self) -> Option<Token> {
        let ref_line = self.line;

        loop {
            self.advance()?;

            if self.line != ref_line {
                break;
            }
        }

        None
    }

    fn parse_block_comment(&mut self) -> Option<Token> {

        let mut last = self.current()?;

        loop {
            let curr = self.advance()?;

            if last == '*' && curr == '/' {
                break;
            }

            last = curr;
        }

        None
    }

    #[inline]
    fn parse_mod(&self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.token(TokenType::ModEqual)
        } else {
            self.token(TokenType::Mod)
        }
    }

    #[inline]
    fn parse_xor(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::XorEqual)
        } else {
            self.token(TokenType::Xor)
        }
    }

    #[inline]
    fn parse_or(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::OrEqual)
        } else if let Some('|') = next {
            self.advance();
            self.token(TokenType::OrOr)
        } else {
            self.token(TokenType::Or)
        }
    }

    #[inline]
    fn parse_and(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::AndEqual)
        } else if let Some('&') = next {
            self.advance();
            self.token(TokenType::AndAnd)
        } else {
            self.token(TokenType::And)
        }
    }

    #[inline]
    fn parse_not(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::NotEqual)
        } else {
            self.token(TokenType::Not)
        }
    }

    #[inline]
    fn parse_smaller(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::SmallerOrEqual)
        } else if let Some('<') = next {
            self.advance();
            self.parse_lshift()
        } else {
            self.token(TokenType::Smaller)
        }
    }

    #[inline]
    fn parse_lshift(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::ShiftLeftEqual)
        } else {
            self.token(TokenType::ShiftLeft)
        }
    }

    #[inline]
    fn parse_bigger(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::BiggerOrEqual)
        } else if let Some('<') = next {
            self.advance();
            self.parse_rshift()
        } else {
            self.token(TokenType::Bigger)
        }
    }

    #[inline]
    fn parse_rshift(&mut self) -> Option<Token> {
        let next = self.next();

        if let Some('=') = next {
            self.advance();
            self.token(TokenType::ShiftRightEqual)
        } else {
            self.token(TokenType::ShiftRight)
        }
    }

    fn parse_ident(&mut self) -> Option<Token> {
        // a-zA-Z0-9_

        let mut ident = self.last()?.to_string();
        
        loop {
            let curr = self.current()?;

            let mut pushed = true;

            match curr {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => ident.push(curr),
                _ => pushed = false
            }

            if pushed { self.advance(); }
            else { break; }
        }

        if let Some(keyword) = self.keywords.get(ident.as_str()) {
            self.token(keyword.to_owned())
        } else {
            self.token(TokenType::Ident(ident))
        }
    }

    fn parse_num(&mut self) -> Option<Token> {
        let mut num_string = self.last()?.to_string();

        let mut float = false;
        let mut binary = false;
        let mut hex = false;
        let mut octal = false;

        loop {
            let curr = self.current()?;

            if !hex && !binary && !float && !octal {
                if curr == 'x' && num_string == "0" {
                    if !hex { hex = true }
                    else { 
                        self.errors.push(YccError { 
                            loc: self.construct_loc(), 
                            head: "unexpected character", 
                            where_string: format!("expected number found {curr}") 
                        });
                        return None;
                    }
                    self.advance();
                    num_string.pop();
                    continue;
                } else if curr == 'b' && num_string == "0" {
                    if !binary { binary = true }
                    else { 
                        self.errors.push(YccError { 
                            loc: self.construct_loc(), 
                            head: "unexpected character", 
                            where_string: format!("expected number found {curr}") 
                        });
                        return None;
                    }
                    self.advance();
                    num_string.pop();
                    continue;
                }
                else if curr == 'o' && num_string == "0" {
                    if !octal { octal = true }
                    else { 
                        self.errors.push(YccError { 
                            loc: self.construct_loc(), 
                            head: "unexpected character", 
                            where_string: format!("expected number found {curr}") 
                        });
                        return None;
                    }
                    self.advance();
                    num_string.pop();
                    continue;
                }
            }

            if curr == '.' {
                if num_string.contains(".") {
                    self.errors.push(YccError {
                        loc: self.construct_loc(),
                        head: "unexpected token",
                        where_string: format!("unexpected token: ."),
                    });
                    return None;
                }

                num_string.push('.');
                self.advance();
                continue;
            }

            if binary { if !matches!(curr, '0' | '1') { break } }
            else if hex { if !matches!(curr, '0'..='9' | 'a'..='f' | 'A'..='Z') { break } }
            else if octal { if !matches!(curr, '0'..='7') { break } }
            else if !matches!(curr, '0'..='9' | '_') { // normal numbers
                break
            }

            num_string.push(curr);

            self.advance()?;
        }

        // check if it is a float
        if let Some('f') = self.current() { float = true; self.advance(); }

        if float {
            let Ok(float) = num_string.parse::<f64>() else {
                self.errors.push(YccError { 
                    loc: self.construct_loc(), 
                    head: "float parsing error", 
                    where_string: "invalid float".into()
                });
                return None;
            };

            self.token(TokenType::FloatLiteral(float))
        } else {
            let result = if binary { i64::from_str_radix(&num_string, 2) } 
            else if hex { i64::from_str_radix(&num_string, 16) }
            else if octal { i64::from_str_radix(&num_string, 8) }  
            else {num_string.parse::<i64>() };
            let int = match result {
                Ok(int) => int,
                Err(err) => {
                    self.errors.push(YccError { 
                        loc: self.construct_loc(), 
                        head: "intenger parsing error", 
                        where_string: err.to_string()
                    });
                    return None;
                }
            };

            self.token(TokenType::IntLiteral(int))
        }
    }

    fn parse_char(&mut self) -> Option<Token> {
        //self.advance()?; // '

        let value = self.advance()?;

        if value == '\'' {
            self.errors.push(YccError {
                loc: self.construct_loc(),
                head: "empty char literal",
                where_string: "char literals cannot be empty".into(),
            });
            return None;
        }

        if let Some('\'') = self.advance() {} else {
            self.errors.push(YccError {
                loc: self.construct_loc(),
                head: "undetermined char literal",
                where_string: "expected char literals to be build like this: '{char}'".into(),
            });
            return None;
        }
        
        self.token(TokenType::CharLiteral(value))
    }

    fn parse_string(&mut self) -> Option<Token> {
        //self.advance(); // "

        let mut string = String::new();
        
        loop {
            let current = self.current()?;

            if current == '"' {
                break;
            }

            string.push(current);
            self.advance()?;
        }

        self.advance(); // "

        self.token(TokenType::StringLiteral(string))
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.to_owned()
    }
}