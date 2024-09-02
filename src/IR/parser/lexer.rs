use std::collections::HashMap;

use super::IrError;

/// A location reference.
/// Is recommended to be used for giving tokens locations
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// .
    Dot,

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
    Int(i64),

    /// declare
    Declare,

    /// define
    Define,

    /// @func_name
    Func(String),
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
    line_no: u64,

    coloumn: u64,

    start: u64,
    current: u64,

    loc: Loc,

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

        Self {
            input_stream: input,
            line: String::new(),
            line_no: 1,
            coloumn: 1,
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
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.input_stream.chars().count() - 1) as u64
    }

    fn update_loc(&mut self) {
        self.loc.line = self.line_no;
        self.loc.coloumn = self.coloumn;
        self.loc.line_string = self.line.to_string();
    }

    fn update_line_string(&mut self) {
        let lines = self.input_stream.split('\n').collect::<Vec<&str>>();
        let line = lines.get((self.line_no - 1) as usize);
        self.line = line.expect("ran out of lines").to_string();
    }

    fn advance(&mut self) -> Result<char, IrError> {
        self.current += 1;
        let peek = self.peek();

        let mut out = ' ';

        if let Some(peek) = peek {
            if peek == '\n' {
                self.coloumn = 1;
                self.line_no += 1;

                self.update_line_string();
            } else {
                self.coloumn += 1;
                out = peek;
            }
        } else {
            Err(IrError::OutOfChars)?
        }

        self.loc.length = self.current - self.start - 1;

        Ok(out)
    }

    fn peek(&mut self) -> Option<char> {
        self.input_stream.chars().nth((self.current - 1) as usize)
    }

    /// "lexes" the input
    pub fn lex(&mut self) -> Result<(), IrError> {
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
        let char = self.advance()?;
        match char {
            '\n' | '\r' | '\t' | ' ' => {},

            '(' => ty = Some(TokenType::LParam),
            '{' => ty = Some(TokenType::LBracket),
            '[' => ty = Some(TokenType::LSquare),
            ')' => ty = Some(TokenType::RParam),
            '}' => ty = Some(TokenType::RBracket),
            ']' => ty = Some(TokenType::RSquare),

            '.' => ty = Some(TokenType::Dot),
            ',' => ty = Some(TokenType::Comma),

            '=' => ty = Some(TokenType::Equal),

            '%' => ty = Some(self.scan_var_name()?),

            '"' => ty = Some(self.scan_string()?),

            'a'..='z' | 'A'..='Z' | '_' => ty = Some(self.scan_ident()?),

            '0'..='9' => ty = Some(self.scan_num()?),

            '@' => ty = Some(self.scan_func()?),

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

        Ok(())
    }

    fn scan_var_name(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        out.push( self.peek().unwrap() );

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid variable names"#) 
                })?
            }

            let chr = self.advance()?;

            match chr {
                '0'..='9' => out.push(chr),
                'a'..='z' => out.push(chr),
                'A'..='Z' => out.push(chr),
                '_' => out.push(chr),

                _ => looping = false,
            }
        }

        Ok(TokenType::Var(out))
    }

    fn scan_string(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"", ' for one valid strings"#) 
                })?
            }

            let chr = self.advance()?;

            match chr {
                '"' => looping = false,

                _ => out.push(chr),
            }
        }

        Ok(TokenType::String(out))
    }

    fn scan_ident(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        let mut looping = true;

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

                
                _ => looping = false,
            };

            if looping {
                self.advance()?;
            }
        }

        if let Some(keyword) = self.keywords.get(&out) {
            Ok(keyword.clone())
        } else {
            Ok(TokenType::Ident(out))
        }
    }

    fn scan_num(&mut self) -> Result<TokenType, IrError> {
        let mut string = String::new();

        let mut looping = true;

        string.push( self.peek().unwrap() );

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid function names"#) 
                })?
            }

            let chr = self.advance()?;

            match chr {
                '0'..='9' => string.push(chr),
                'x' => string.push('x'),
                'b' => string.push('b'),

                _ => looping = false,
            }
        }

        let mut negate = false;

		let mut out = match if string.starts_with("0x") {
			i64::from_str_radix(&string.replace("0x", ""), 16)
		} else if string.starts_with("0b") {
			i64::from_str_radix(&string.replace("0b", ""), 2)
		} else if string.starts_with("-") {
            negate = true;
			string.replace("-", "").parse::<i64>()
		} else {
			string.parse()
		} {
            Ok(i) => i,
            Err(err) => Err(IrError::Boxed{ err: Box::from(err), loc: self.loc.clone() })?,
        };

        if negate {
            out = -out;
        }

        Ok(TokenType::Int(out))
    }

    fn scan_func(&mut self) -> Result<TokenType, IrError> {
        let mut out = String::new();

        out.push( self.peek().unwrap() );

        let mut looping = true;

        while looping {
            if self.is_at_end() {
                Err(IrError::UndeterminedTokenSequence { 
                    loc: self.loc.clone(), 
                    expected: String::from(r#"'a'..'z', 'A'..'Z' or '0'..'9' for valid function names"#) 
                })?
            }

            let chr = self.advance()?;

            match chr {
                '0'..='9' => out.push(chr),
                'a'..='z' => out.push(chr),
                'A'..='Z' => out.push(chr),
                '_' => out.push(chr),

                _ => looping = false,
            }
        }

        Ok(TokenType::Func(out))
    }
}