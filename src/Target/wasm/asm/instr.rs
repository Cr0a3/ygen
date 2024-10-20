use std::fmt::Display;

use crate::CodeGen::MCInstr;

/// A wasm instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmMCInstr {
    pub(crate) mnemonic: WasmMnemonic,
    pub(crate) prefix: Option<WasmPrefix>,
    pub(crate) op1: Option<WasmOperand>,
}

impl WasmMCInstr {
    /// Creates an wasm instruction without any operands
    pub fn with0(prefix: Option<WasmPrefix>, mne: WasmMnemonic) -> Self {
        Self {
            prefix: prefix,
            mnemonic: mne,
            op1: None,
        }
    }

    /// Creates an wasm instruction with 1 operand
    pub fn with1(prefix: Option<WasmPrefix>, mne: WasmMnemonic, op1: WasmOperand) -> Self {
        Self {
            prefix: prefix,
            mnemonic: mne,
            op1: Some(op1),
        }
    }

    pub(crate) fn encode(&self) -> Result<(Vec<u8>, Option<crate::Obj::Link>), Box<dyn std::error::Error>> {
        let mut encoded;

        match self.mnemonic {
            WasmMnemonic::Get => {
                let op1 = self.op1.expect("...get expects localidx");
                let op1 = match op1 {
                    WasmOperand::Var(var) => var as u8,
                    WasmOperand::Const(_) => panic!("...set expects localidx"),
                };

                let op = match self.prefix.expect("...get expects an prefix") {
                    WasmPrefix::Local => 0x20,
                    WasmPrefix::Global => 0x23,
                    _ => panic!("...get expects either local or global as its prefix")
                };

                encoded = vec![op, op1]
            },
            WasmMnemonic::Set => {
                let op1 = self.op1.expect("...set expects localidx");
                let op1 = match op1 {
                    WasmOperand::Var(var) => var as u8,
                    WasmOperand::Const(_) => panic!("...set expects localidx"),
                };

                let op = match self.prefix.expect("...set expects an prefix") {
                    WasmPrefix::Local => 0x21,
                    WasmPrefix::Global => 0x24,
                    _ => panic!("...set expects either local or global as its prefix")
                };

                encoded = vec![op, op1]
            },
            WasmMnemonic::Const => {
                let op1 = self.op1.expect("...const expects a imm op");
                let op1 = match op1 {
                    WasmOperand::Const(imm) => imm,
                    _ => panic!("...const expects a imm op"),
                };

                let op = match self.prefix.expect("...const expects an prefix") {
                    WasmPrefix::i32 => 0x41,
                    WasmPrefix::i64 => 0x42,
                    WasmPrefix::f32 => 0x43,
                    WasmPrefix::f64 => 0x44,
                    _ => panic!("...const must only have either i32, i64, f32 or f64 as its prefix")
                };

                let mut bytes = match self.prefix.expect("...const expects an prefix") {
                    WasmPrefix::i32 => (op1 as i32).to_le_bytes().to_vec(),
                    WasmPrefix::i64 => (op1 as i64).to_le_bytes().to_vec(),
                    WasmPrefix::f32 => (op1 as f32).to_bits().to_le_bytes().to_vec(),
                    WasmPrefix::f64 => (op1 as f64).to_bits().to_le_bytes().to_vec(),
                    _ => unreachable!(),
                };


                if self.prefix.unwrap() != WasmPrefix::f32 && self.prefix.unwrap() != WasmPrefix::f64 {
                    while let Some(&last) = bytes.last() {
                        if last == 0 {
                            bytes.pop();
                        } else {
                            break;
                        }
                    }
                }

                encoded = vec![op];

                encoded.extend_from_slice(&bytes);

                if bytes.len() == 0 {
                    encoded.push(0);
                }

                if self.prefix.unwrap() != WasmPrefix::f32 && self.prefix.unwrap() != WasmPrefix::f64 {
                    match bytes.len() {
                        1 => encoded.push(0x01),
                        2 => encoded.push(0x03),
                        3 => encoded.push(0x07),
                        4 => encoded.push(0x0f),
                        5 => encoded.push(0x1f),
                        6 => encoded.push(0x3f),
                        7 => encoded.push(0x00),
                        8 => encoded.push(0x0f),
                        _ => {},
                    }
                }
            }
            WasmMnemonic::Add => {
                encoded = vec![match self.prefix.expect("...add expects an prefix") {
                    WasmPrefix::i32 => 0x6a,
                    WasmPrefix::i64 => 0x7c,
                    WasmPrefix::f32 => 0x92,
                    WasmPrefix::f64 => 0xa0,
                    _ => panic!("...add must have either i32, i64, f32 or f64 as its prefix")
                }]
            },
            WasmMnemonic::Sub => {
                encoded = vec![match self.prefix.expect("...sub expects an prefix") {
                    WasmPrefix::i32 => 0x6b,
                    WasmPrefix::i64 => 0x7d,
                    WasmPrefix::f32 => 0x93,
                    WasmPrefix::f64 => 0xa1,
                    _ => panic!("...sub must have either i32, i64, f32 or f64 as its prefix")
                }]
            },
            WasmMnemonic::Mul => {
                encoded = vec![match self.prefix.expect("...mul expects an prefix") {
                    WasmPrefix::i32 => 0x6c,
                    WasmPrefix::i64 => 0x7e,
                    WasmPrefix::f32 => 0x94,
                    WasmPrefix::f64 => 0xa2,
                    _ => panic!("...mul must have either i32, i64, f32 or f64 as its prefix")
                }]
            },
            WasmMnemonic::Div => {
                encoded = vec![match self.prefix.expect("...div expects an prefix") {
                    WasmPrefix::f32 => 0x95,
                    WasmPrefix::f64 => 0xa3,
                    _ => panic!("...div only works for f32, f64 if you want to use i32/i64 take a look at divs and divu")
                }]
            },
            WasmMnemonic::Divs => {
                encoded = vec![match self.prefix.expect("...divs expects an prefix") {
                    WasmPrefix::i32 => 0x6d,
                    WasmPrefix::i64 => 0x7f,
                    _ => panic!("...divs only works for i32, i64")
                }]
            },
            WasmMnemonic::Divu => {
                encoded = vec![match self.prefix.expect("...divu expects an prefix") {
                    WasmPrefix::i32 => 0x6e,
                    WasmPrefix::i64 => 0x80,
                    _ => panic!("...divu only works for i32, i64")
                }]
            },
            WasmMnemonic::Rems => {
                encoded = vec![match self.prefix.expect("...rems expects an prefix") {
                    WasmPrefix::i32 => 0x6f,
                    WasmPrefix::i64 => 0x81,
                    _ => panic!("...rems only works for i32, i64")
                }]
            },
            WasmMnemonic::Remu => {
                encoded = vec![match self.prefix.expect("...remu expects an prefix") {
                    WasmPrefix::i32 => 0x70,
                    WasmPrefix::i64 => 0x82,
                    _ => panic!("...remu only works for i32, i64")
                }]
            },
            WasmMnemonic::Return => encoded = vec![0x0f]
        }

        Ok((encoded, None))
        //todo!("ygen doesn't support wasm instruction encoding yet")
    }
}

/// A webassembly mnemonic (prefix.mnemonic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum WasmMnemonic {
    Get,
    Set,
    Const,
    Add,
    Mul,
    Sub,
    Div,
    Divs,
    Divu,
    Rems,
    Remu,
    Return,
}

impl From<String> for WasmMnemonic {
    fn from(value: String) -> Self {
        match value.as_str() {
            "get" => WasmMnemonic::Get,
            "set" => WasmMnemonic::Set,
            "const" => WasmMnemonic::Const,
            "add" => WasmMnemonic::Add,
            "mul" => WasmMnemonic::Mul,
            "sub" => WasmMnemonic::Sub,
            "div" => WasmMnemonic::Div,
            "div_s" => WasmMnemonic::Divs,
            "div_u" => WasmMnemonic::Divu,
            "rem_s" => WasmMnemonic::Rems,
            "rem_u" => WasmMnemonic::Remu,
            "return" => WasmMnemonic::Return,
            _ => panic!("unkown wasm mnemonic: {value}"),
        }
    }
}

impl Display for WasmMnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WasmMnemonic::Get => "get",
            WasmMnemonic::Set => "set",
            WasmMnemonic::Const => "const",
            WasmMnemonic::Add => "add",
            WasmMnemonic::Mul => "mul",
            WasmMnemonic::Sub => "sub",
            WasmMnemonic::Div => "div",
            WasmMnemonic::Divs => "div_s",
            WasmMnemonic::Divu => "div_u",
            WasmMnemonic::Rems => "rem_s",
            WasmMnemonic::Remu => "rem_u",
            WasmMnemonic::Return => "return",
        })
    }
}

/// A webassembly prefix (prefix.mnemonic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum WasmPrefix {
    Local,
    Global,

    i32,
    i64,
    f32,
    f64
}

impl From<String> for WasmPrefix {
    fn from(value: String) -> Self {
        match value.as_str() {
            "local" => WasmPrefix::Local,
            "global" => WasmPrefix::Global,
            "i32" => WasmPrefix::i32,
            "i64" => WasmPrefix::i64,
            "f32" => WasmPrefix::f32,
            "f64" => WasmPrefix::f64,

            _ => panic!("unkown wasm prefix: {value}"),
        }
    }
}

impl Display for WasmPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WasmPrefix::Local => "local",
            WasmPrefix::Global => "global",
            WasmPrefix::i32 => "i32",
            WasmPrefix::i64 => "i64",
            WasmPrefix::f32 => "f32",
            WasmPrefix::f64 => "f64",
        })
    }
}

/// A webassembly operand
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum WasmOperand {
    Var(i32),
    Const(f64),
}

impl PartialEq for WasmOperand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Var(l0), Self::Var(r0)) => l0 == r0,
            (Self::Const(l0), Self::Const(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for WasmOperand {}

impl Display for WasmOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WasmOperand::Var(var) => format!("{}", var),
            WasmOperand::Const(imm) => format!("{:.5}", imm), 
        })
    }
}

impl Display for WasmMCInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = String::new();

        if let Some(prefix) = self.prefix {
            fmt.push_str(&prefix.to_string());
            fmt.push('.');
        }

        fmt.push_str(&self.mnemonic.to_string());

        if let Some(op1) = self.op1 {
            fmt.push(' ');
            fmt.push_str(&op1.to_string());
        }

        write!(f, "{}", fmt)
    }
}

impl MCInstr for WasmMCInstr {
    fn dump(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(vec![format!("{}", self)])
    }

    fn encode(&self) -> Result<(Vec<u8>, Option<crate::Obj::Link>), Box<dyn std::error::Error>> {
        self.encode()
    }

    fn clone_box(&self) -> Box<dyn MCInstr> {
        Box::new(self.clone())
    }
}

impl From<WasmMCInstr> for Box<dyn MCInstr> {
    fn from(value: WasmMCInstr) -> Self {
        Box::new( value )
    }
}