use std::collections::HashMap;

use super::IrError;

/// A location reference.
/// Is recommended to be used for giving tokens locations
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Loc {
    /// The line number
    pub line: u64,
    /// The coloumn
    pub coloumn: u64,
    /// The length of the sequence
    pub length: u64,
    /// The entire source line
    pub line_string: String,
}

/// The token type for parsing ir
#[derive(Debug, Clone)]
pub enum TokenType {
    /// :
    Dot,

    /// ...
    TripleDot,

    /// const
    Const,

    /// ,
    Comma,

    /// %name
    Var(String),

    /// =
    Equal,

    /// (
    LParam,

    /// )
    RParam,

    /// {
    LBracket,

    /// }
    RBracket,

    /// [
    LSquare,

    /// ]
    RSquare,

    /// a-zA-Z.
    Ident(String),

    /// "abc"
    String(String),

    /// 1234
    Int(f64),

    /// declare
    Declare,

    /// define
    Define,

    /// @func_name
    Func(String),

    /// block:
    Block(String),

    /// cond
    Cond,

    /// !
    ExclamationMark,

    /// ^abc (till line end)
    UnIdent(String),

    /// <
    VecStartIndicator,

    /// >
    VecEndIndicator,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Var(l0), Self::Var(r0)) => l0 == r0,
            (Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Func(l0), Self::Func(r0)) => l0 == r0,
            (Self::Block(l0), Self::Block(r0)) => l0 == r0,
            (Self::UnIdent(l0), Self::UnIdent(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for TokenType {}

impl TokenType {
    pub(crate) fn name(&self) -> String {
        match self {
            TokenType::Dot => ":",
            TokenType::Const => "const",
            TokenType::Comma => ",",
            TokenType::Var(_) => "var",
            TokenType::Equal => "=",
            TokenType::LParam => "(",
            TokenType::RParam => ")",
            TokenType::LBracket => "{",
            TokenType::RBracket => "}",
            TokenType::LSquare => "[",
            TokenType::RSquare => "]",
            TokenType::Ident(_) => "ident",
            TokenType::String(_) => "string",
            TokenType::Int(_) => "int",
            TokenType::Declare => "declare",
            TokenType::Define => "define",
            TokenType::Func(_) => "func",
            TokenType::TripleDot => "...",
            TokenType::Block(_) => "block",
            TokenType::Cond => "cond",
            TokenType::ExclamationMark => "!",
            TokenType::UnIdent(_) => "uident",
            TokenType::VecStartIndicator => ">",
            TokenType::VecEndIndicator => "<",
        }.to_string()
    }
}

/// An ir token
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// the type
    pub typ: TokenType,
    /// the location
    pub loc: Loc,
}

/// A lexer for lexing ygen ir strings
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrLexer {
    input_stream: String,

    line: String,
    lines: Vec<String>,
    line_no: u64,

    coloumn: u64,

    start: u64,
    current: u64,

    loc: Loc,

    no_pop: bool,

    keywords: HashMap<String, TokenType>,

    /// The output
    pub out: Vec<Token>,
}

impl IrLexer {
    /// Creates a new ir lexer
    pub fn new(input: String) -> Self {
        let mut keys = HashMap::new();

        keys.insert("declare".into(), TokenType::Declare);
        keys.insert("define".into(), TokenType::Define);
        keys.insert("const".into(), TokenType::Const);
        keys.insert("cond".into(), TokenType::Cond);

        let input = format!("{}\n", input);

        let lines = input
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Self {
            input_stream: input,
            line: String::new(),
            line_no: 1,

            lines: lines,

            coloumn: 0,
            start: 0,
            current: 0,

            keywords: keys,

            loc: Loc {
                line: 1,
                coloumn: 0,
                length: 0,
                line_string: String::new(),
            },

            out: vec![],

            no_pop: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current > (self.input_stream.chars().count() - 1) as u64
    }

    fn update_loc(&mut self) {
        self.loc.line = self.line_no;
        self.loc.coloumn = self.coloumn;
        self.loc.line_string = self.line.to_string();
    }

    fn update_line_string(&mut self) {
        let line = self.lines.get((self.line_no - 1) as usize);
        self.line = line.expect("ran out of lines").to_string();
    }

    fn advance(&mut self) -> Result<char, IrError> {
        if !self.no_pop {
            self.current += 1;
        }    
        
        let peek = self.peek();

        let mut out = ' ';

        if let Some(peek) = peek {
            if peek == '\n' {
                if !self.no_pop {
                    self.coloumn = 0;
                    self.line_no += 1;

                    self.update_line_string();
                }
            } else {
                if !self.no_pop {
                    self.coloumn += 1;
                }
                out = peek;
            }
        } else {
            println!("curr: {}", self.current);
            println!("len: {}", self.input_stream.chars().count());
            panic!("lexer ran out of chars");
        }

        if self.no_pop {
            self.no_pop = false;
        }

        if self.current != self.start {
            self.loc.length = self.current - self.start;
        } else {
            self.loc.length = 1;
        }


        Ok(out)
    }

    fn peek(&mut self) -> Option<char> {
        self.input_stream.chars().nth((self.current - 1) as usize)
    }


    /// "lexes" the input
    pub fn lex(&mut self) -> Result<(), IrError> { 
        self.advance()?;
        self.update_line_string();

        while !self.is_at_end() {
            self.update_loc();

            self.start = self.current;

            self.lex_tok()?;
        }

        Ok(())
    }

    fn lex_tok(&mut self) -> Result<(), IrError> {
        let mut ty = None;
        let peek = self.peek().unwrap();
        match peek {
            '\n' | '\r' | '\t' | ' ' => {},

            '(' => ty = Some(TokenType::LParam),
            '{' => ty = Some(TokenType::LBracket),
            '[' => ty = Some(TokenType::LSquare),

            ')' => ty = Some(TokenType::RParam),
            '}' => ty = Some(TokenType::RBracket),
            ']' => ty = Some(TokenType::RSquare),

            ':' => ty = Some(TokenType::Dot),
            ',' => ty = Some(TokenType::Comma),

            '=' => ty = Some(TokenType::Equal),

            '%' => ty = Some(self.scan_var_name()?),

            '"' => ty = Some(self.scan_string()?),

            ';' => {
                let ref_line = self.line_no;

                loop {
                    self.advance()?;

                    if self.line_no != ref_line {
                        break;
                    }
                }
            }

            '.' => {
                self.advance()?;
                if let '.' = self.peek().unwrap() {} else {
                    Err(IrError::UnexpectedCharacter { 
                        chr: self.peek().unwrap(), 
                        loc: self.loc.clone() 
                    })?
                }
                self.advance()?;
                if let '.' = self.peek().unwrap() {} else {
                    Err(IrError::UnexpectedCharacter { 
                        chr: self.peek().unwrap(), 
                        loc: self.loc.clone() 
                    })?
                }
                ty = Some(TokenType::TripleDot)
            },

            '!' => ty = Some(TokenType::ExclamationMark),

            '^' => {
                self.advance()?;
                let mut ident = String::new();

                loop {
                    let peek = self.peek();

                    if let Some(peek) = peek {
                        if '\n' == peek {
                            break;
                        }

                        if '\r' != peek {
                            ident.push(peek);
                        }
                    } else {
                        break;
                    }

                    self.advance()?;
                }

                ty = Some(TokenType::UnIdent(ident))
            },

            'a'..='z' | 'A'..='Z' | '_' => ty = Some(self.scan_ident()?),

            '0'..='9' | '-' => ty = Some(self.scan_num()?),

            '@' => ty = Some(self.scan_func()?),


            '<' => ty = Some(TokenType::VecStartIndicator),
            '>' => ty = Some(TokenType::VecEndIndicator),

            any => Err(IrError::UnexpectedCharacter { 
                chr: any, 
                loc: self.loc.clone() 
            })?
        };

        if let Some(typ) = ty {
            self.out.push(Token { 
                typ: typ, 
                loc: self.loc.clone() 
            });
        }

        if !self.is_at_end() {
            self.advance()?;
        }

        Ok(())
    }

    fn scan_var_name(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        out.push('%');
        self.advance()?;

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid variable names"#) 
                })?
            }

            let chr = self.peek().unwrap();

            match chr {
                '0'..='9' => out.push(chr),
                'a'..='z' => out.push(chr),
                'A'..='Z' => out.push(chr),
                '_' => out.push(chr),

                _ => looping = false,
            }

            if looping {
                self.advance()?;
            }
        }

        self.no_pop = true;

        Ok(TokenType::Var(out))
    }

    fn scan_string(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"" or ' for valid strings"#) 
                })?
            }

            let chr = self.advance()?;

            match chr {
                '"' => looping = false,

                _ => out.push(chr),
            }
        }

        let unescaped = unescaper::unescape(&out);

        let unescaped = match unescaped {
            Ok(string) => string,
            Err(err) => {
                Err(IrError::Boxed { 
                    loc: self.loc.clone(), 
                    err: Box::new(err) 
                })?
            }
        };

        Ok(TokenType::String(unescaped))
    }

    fn scan_ident(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        let mut looping = true;
        let mut block = false;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z', '0'..'9' or '_' for idents"#) 
                })?
            }

            let chr = self.peek().unwrap();

            match chr {
                '0'..='9' => out.push(chr),
                'a'..='z' => out.push(chr),
                'A'..='Z' => out.push(chr),
                '_' => out.push(chr),
                '.' => out.push(chr),

                ':' => {
                    looping = false;
                    block = true;
                }
                
                _ => looping = false,
            };

            if looping {
                self.advance()?;
            }
        }

        if !block {
            self.no_pop = true;
        }

        if block {
            return Ok(TokenType::Block(out));
        }

        if let Some(keyword) = self.keywords.get(&out) {
            Ok(keyword.clone())
        } else {
            Ok(TokenType::Ident(out))
        }
    }

    fn scan_num(&mut self) -> Result<TokenType, IrError> {
        let mut string = String::new();

        if let Some('-') = self.peek() {
            string.push('-');
            self.advance()?;
        }

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid function names"#) 
                })?
            }

            let chr = self.peek().unwrap();

            match chr {
                '0'..='9' => string.push(chr),
                'x' => string.push('x'),
                'b' => string.push('b'),

                _ => looping = false,
            }

            if looping {
                self.advance()?;
            }
        }

        let mut negate = false;

        let mut out = if string.contains('.') {
            match string.parse::<f64>() {
                Ok(i) => i,
                Err(err) => Err(IrError::Boxed{ err: Box::from(err), loc: self.loc.clone() })?,
            }
        } else {
            match if string.starts_with("0x") {
                i64::from_str_radix(&string.replace("0x", ""), 16)
            } else if string.starts_with("0b") {
                i64::from_str_radix(&string.replace("0b", ""), 2)
            } else if string.starts_with("-") {
                negate = true;
                string.replace("-", "").parse::<i64>()
            } else {
                string.parse()
            } {
                Ok(i) => i as f64,
                Err(err) => Err(IrError::Boxed{ err: Box::from(err), loc: self.loc.clone() })?,
            }
        };

        if negate {
            out = -out;
        }

        self.no_pop = true;

        Ok(TokenType::Int(out as f64))
    }

    fn scan_func(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        self.advance()?; // @

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid function names"#) 
                })?
            }

            let chr = self.peek().unwrap();

            match chr {
                '0'..='9' => out.push( chr ),
                'a'..='z' => out.push( chr ),
                'A'..='Z' => out.push( chr ),
                '@' => out.push('@'),
                '_' => out.push( '_' ),
                '.' => out.push( '.' ),

                _ => looping = false,
            }

            if looping {
                self.advance()?;
            }
        }

        self.no_pop = true;

        Ok(TokenType::Func(out))
    }
}