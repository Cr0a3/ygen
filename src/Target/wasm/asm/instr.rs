use std::fmt::Display;

use wasm_encoder::{BlockType, ValType};

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
                let op1 = self.op1.as_ref().expect("...get expects localidx");
                let op1 = match op1 {
                    WasmOperand::Var(var) => *var as u8,
                    _ => panic!("...set expects localidx"),
                };

                let op = match self.prefix.expect("...get expects an prefix") {
                    WasmPrefix::Local => 0x20,
                    WasmPrefix::Global => 0x23,
                    _ => panic!("...get expects either local or global as its prefix")
                };

                encoded = vec![op, op1]
            },
            WasmMnemonic::Set => {
                let op1 = self.op1.as_ref().expect("...set expects localidx");
                let op1 = match op1 {
                    WasmOperand::Var(var) => *var as u8,
                    _ => panic!("...set expects localidx"),
                };

                let op = match self.prefix.expect("...set expects an prefix") {
                    WasmPrefix::Local => 0x21,
                    WasmPrefix::Global => 0x24,
                    _ => panic!("...set expects either local or global as its prefix")
                };

                encoded = vec![op, op1]
            },
            WasmMnemonic::Const => {
                let op1 = self.op1.as_ref().expect("...const expects a imm op");
                let op1 = match op1 {
                    WasmOperand::Const(imm) => *imm,
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
            },
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
            WasmMnemonic::Return => encoded = vec![0x0f],
            WasmMnemonic::Eq | WasmMnemonic::Ne | WasmMnemonic::Gt | 
            WasmMnemonic::Gts | WasmMnemonic::Gtu | WasmMnemonic::Lt | 
            WasmMnemonic::Lts | WasmMnemonic::Ltu | WasmMnemonic::Ge | 
            WasmMnemonic::Ges | WasmMnemonic::Geu | WasmMnemonic::Le | 
            WasmMnemonic::Les | WasmMnemonic::Leu => {
                let prefix = self.prefix.expect("...rems expects an prefix");

                encoded = vec![match self.mnemonic {
                    WasmMnemonic::Eq => match prefix {
                        WasmPrefix::i32 => 0x46,
                        WasmPrefix::i64 => 0x51,
                        WasmPrefix::f32 => 0x5b,
                        WasmPrefix::f64 => 0x61,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Ne => match prefix {
                        WasmPrefix::i32 => 0x47,
                        WasmPrefix::i64 => 0x52,
                        WasmPrefix::f32 => 0x5c,
                        WasmPrefix::f64 => 0x62,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Gt => match prefix {
                        WasmPrefix::f32 => 0x5e,
                        WasmPrefix::f64 => 0x64,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Gts => match prefix {
                        WasmPrefix::i32 => 0x4a,
                        WasmPrefix::i64 => 0x56,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Gtu => match prefix {
                        WasmPrefix::i32 => 0x4b,
                        WasmPrefix::i64 => 0x56,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Lt => match prefix {
                        WasmPrefix::f32 => 0x5d,
                        WasmPrefix::f64 => 0x63,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Lts => match prefix {
                        WasmPrefix::i32 => 0x48,
                        WasmPrefix::i64 => 0x53,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Ltu => match prefix {
                        WasmPrefix::i32 => 0x49,
                        WasmPrefix::i64 => 0x54,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Ge => match prefix {
                        WasmPrefix::f32 => 0x60,
                        WasmPrefix::f64 => 0x66,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Ges => match prefix {
                        WasmPrefix::i32 => 0x4e,
                        WasmPrefix::i64 => 0x59,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Geu => match prefix {
                        WasmPrefix::i32 => 0x5f,
                        WasmPrefix::i64 => 0x5a,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Le => match prefix {
                        WasmPrefix::f32 => 0x5f,
                        WasmPrefix::f64 => 0x65,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Les => match prefix {
                        WasmPrefix::i32 => 0x4c,
                        WasmPrefix::i64 => 0x57,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },
                    WasmMnemonic::Leu => match prefix {
                        WasmPrefix::i32 => 0x4d,
                        WasmPrefix::i64 => 0x58,
                        _ => panic!("cmp instructions are only usable with i32/i64/f32/f64 prefix"),
                    },

                    _ => unreachable!(),
                }]
            },
            WasmMnemonic::And | WasmMnemonic::Or | WasmMnemonic::Shl | WasmMnemonic::Shrs | WasmMnemonic::Shru | WasmMnemonic::Xor => {
                let prefix = self.prefix.expect("and/or/shl/shr/xor expect a prefix");

                encoded = vec![match self.mnemonic {
                    WasmMnemonic::And => match prefix {
                        WasmPrefix::i32 => 0x71,
                        WasmPrefix::i64 => 0x83,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    WasmMnemonic::Xor => match prefix {
                        WasmPrefix::i32 => 0x73,
                        WasmPrefix::i64 => 0x85,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    WasmMnemonic::Or => match prefix {
                        WasmPrefix::i32 => 0x72,
                        WasmPrefix::i64 => 0x84,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    WasmMnemonic::Shl => match prefix {
                        WasmPrefix::i32 => 0x74,
                        WasmPrefix::i64 => 0x86,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    WasmMnemonic::Shrs => match prefix {
                        WasmPrefix::i32 => 0x75,
                        WasmPrefix::i64 => 0x87,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    WasmMnemonic::Shru => match prefix {
                        WasmPrefix::i32 => 0x76,
                        WasmPrefix::i64 => 0x88,
                        _ => panic!("and/or/shl/shr/xor only work on i32/i64")
                    },
                    _ => unreachable!()
                }]
            },
            WasmMnemonic::Neg => {
                let prefix = self.prefix.expect("neg expects prefix");

                encoded = vec![match prefix {
                    WasmPrefix::f32 => 0x8c,
                    WasmPrefix::f64 => 0x9a,
                    _ => panic!("neg only supports f32/f64")
                }];
            },
            WasmMnemonic::Extends => encoded = vec![0xac],
            WasmMnemonic::Extendu => encoded = vec![0xad],
            WasmMnemonic::Wrap => encoded = vec![0xa7],
            WasmMnemonic::Promote => encoded = vec![0xbb],
            WasmMnemonic::Demote => encoded = vec![0xb6],
            WasmMnemonic::ConvertI32s => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::f32 => 0xb2,
                    WasmPrefix::f64 => 0xb7,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::ConvertI32u => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::f32 => 0xb3,
                    WasmPrefix::f64 => 0xb8,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::ConvertI64s => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::f32 => 0xb4,
                    WasmPrefix::f64 => 0xb9,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },    
            WasmMnemonic::ConvertI64u => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::f32 => 0xb5,
                    WasmPrefix::f64 => 0xba,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },    
            WasmMnemonic::TruncF32s => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::i32 => 0xa8,
                    WasmPrefix::i64 => 0xae,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::TruncF32u => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::i32 => 0xa9,
                    WasmPrefix::i64 => 0xaf,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::TruncF64s => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::i32 => 0xaa,
                    WasmPrefix::i64 => 0xb0,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::TruncF64u => {
                let prefix = self.prefix.expect("convert_i32_s expects a prefix");

                encoded = vec![match prefix {
                    WasmPrefix::i32 => 0xab,
                    WasmPrefix::i64 => 0xb1,
                    _ => panic!("illegal prefix for convert_i32_s: {}", prefix),
                }];
            },
            WasmMnemonic::Br => {
                if let Some(WasmOperand::Const(target)) = self.op1 {
                    encoded = vec![0x0c, target as u8]
                } else {
                    encoded = vec![0x0c, 0x00]
                }
            },
            WasmMnemonic::Block => encoded = vec![0x02, 0x40],
            WasmMnemonic::End => encoded = vec![0x0b],
        }

        Ok((encoded, None))
        //todo!("ygen doesn't support wasm instruction encoding yet")
    }
}

/// A webassembly mnemonic (prefix.mnemonic)
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Eq,
    Ne,
    Gt,
    Gts,
    Gtu,
    Lt,
    Lts,
    Ltu,
    Ge,
    Ges,
    Geu,
    Le,
    Les,
    Leu,

    And,
    Xor,
    Or,
    Shl,
    Shrs,
    Shru,

    Neg,

    Extends,
    Extendu,

    Wrap,
    Promote,
    Demote,

    ConvertI32s,
    ConvertI32u,
    ConvertI64s,
    ConvertI64u,

    TruncF32s,
    TruncF32u,
    TruncF64s,
    TruncF64u,

    Br,

    Block,
    End,
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
            "eq" => WasmMnemonic::Eq,
            "ne" => WasmMnemonic::Ne,
            "gt" => WasmMnemonic::Gt,
            "gt_s" => WasmMnemonic::Gts,
            "gt_u" => WasmMnemonic::Gtu,
            "lt" => WasmMnemonic::Lt,
            "lt_s" => WasmMnemonic::Lts,
            "lt_u" => WasmMnemonic::Ltu,
            "le" => WasmMnemonic::Le,
            "le_s" => WasmMnemonic::Les,
            "le_u" => WasmMnemonic::Leu,
            "ge_s" => WasmMnemonic::Ges,
            "ge_u" => WasmMnemonic::Geu,
            "and" => WasmMnemonic::And,
            "xor" => WasmMnemonic::Xor,
            "or" => WasmMnemonic::Or,
            "shl" => WasmMnemonic::Shl,
            "shr_s" => WasmMnemonic::Shrs,
            "shr_u" => WasmMnemonic::Shru,
            "neg" => WasmMnemonic::Neg,
            "extend_i32_s" => WasmMnemonic::Extends,
            "extend_i32_u" => WasmMnemonic::Extendu,
            "wrap_i64" => WasmMnemonic::Wrap,
            "promote_f32" => WasmMnemonic::Promote,
            "demote_f32" => WasmMnemonic::Demote,
            "trunc_f32_s" => WasmMnemonic::TruncF32s,
            "trunc_f32_u" => WasmMnemonic::TruncF32u,
            "trunc_f64_s" => WasmMnemonic::TruncF64s,
            "trunc_f64_u" => WasmMnemonic::TruncF64u,
            "convert_i32_s" => WasmMnemonic::ConvertI32s,
            "convert_i32_u" => WasmMnemonic::ConvertI32u,
            "convert_i64_s" => WasmMnemonic::ConvertI64s,
            "convert_i64_u" => WasmMnemonic::ConvertI64u,
            "br" => WasmMnemonic::Br,
            "block" => WasmMnemonic::Block,
            "end" => WasmMnemonic::End,
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
            WasmMnemonic::Eq => "eq",
            WasmMnemonic::Ne => "ne",
            WasmMnemonic::Gt => "gt",
            WasmMnemonic::Gts => "gt_s",
            WasmMnemonic::Gtu => "gt_u",
            WasmMnemonic::Lt => "lt",
            WasmMnemonic::Lts => "lt_s",
            WasmMnemonic::Ltu => "lt_u",
            WasmMnemonic::Ge => "ge",
            WasmMnemonic::Ges => "ge_s",
            WasmMnemonic::Geu => "ge_u",
            WasmMnemonic::Le => "le",
            WasmMnemonic::Les => "le_s",
            WasmMnemonic::Leu => "le_u",
            WasmMnemonic::And => "and",
            WasmMnemonic::Xor => "xor",
            WasmMnemonic::Or => "or",
            WasmMnemonic::Shl => "shl",
            WasmMnemonic::Shrs => "shr_s",
            WasmMnemonic::Shru => "shr_u",
            WasmMnemonic::Neg => "neg",
            WasmMnemonic::Extends => "extend_i32_s",
            WasmMnemonic::Extendu => "extend_i32_u",
            WasmMnemonic::Wrap => "wrap_i64",
            WasmMnemonic::Promote => "promote_f32",
            WasmMnemonic::Demote => "demote_f32",
            WasmMnemonic::ConvertI32s => "convert_i32_s",
            WasmMnemonic::ConvertI32u => "convert_i32_u",
            WasmMnemonic::ConvertI64s => "convert_i64_s",
            WasmMnemonic::ConvertI64u => "convert_i64_u",
            WasmMnemonic::TruncF32s => "trunc_f32_s",
            WasmMnemonic::TruncF32u => "trunc_f32_u",
            WasmMnemonic::TruncF64s => "trunc_f64_s",
            WasmMnemonic::TruncF64u => "trunc_f64_u",
            WasmMnemonic::Br => "br",
            WasmMnemonic::Block => "block",
            WasmMnemonic::End => "end",
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
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum WasmOperand {
    Var(i32),
    Const(f64),
    BlockLink(/*target*/String),
}

impl PartialEq for WasmOperand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Var(l0), Self::Var(r0)) => l0 == r0,
            (Self::Const(l0), Self::Const(r0)) => l0 == r0,
            (Self::BlockLink(l0), Self::BlockLink(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for WasmOperand {}

impl Display for WasmOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WasmOperand::Var(var) => format!("{var}"),
            WasmOperand::Const(imm) => format!("{:.5}", imm), 
            WasmOperand::BlockLink(target) => format!("{target}"),
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

        if let Some(op1) = &self.op1 {
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

impl<'a> Into<wasm_encoder::Instruction<'a>> for WasmMCInstr {
    fn into(self) -> wasm_encoder::Instruction<'a> {
        use wasm_encoder::Instruction;
        match self.mnemonic {
            WasmMnemonic::Get => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::Local => Instruction::LocalGet(match self.op1.unwrap() { WasmOperand::Var(num) => num as u32, _ => panic!()}),
                WasmPrefix::Global => Instruction::GlobalGet(match self.op1.unwrap() { WasmOperand::Var(num) => num as u32, _ => panic!()}),
                _ => panic!(),
            }},
            WasmMnemonic::Set => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::Local => Instruction::LocalSet(match self.op1.unwrap() { WasmOperand::Var(num) => num as u32, _ => panic!()}),
                WasmPrefix::Global => Instruction::GlobalSet(match self.op1.unwrap() { WasmOperand::Var(num) => num as u32, _ => panic!()}),
                _ => panic!(),
            }},
            WasmMnemonic::Const => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Const(match self.op1.unwrap() { WasmOperand::Const(num) => num as i32, _ => panic!()}),
                WasmPrefix::i64 => Instruction::I64Const(match self.op1.unwrap() { WasmOperand::Const(num) => num as i64, _ => panic!()}),
                WasmPrefix::f32 => Instruction::F32Const(match self.op1.unwrap() { WasmOperand::Const(num) => num as f32, _ => panic!()}),
                WasmPrefix::f64 => Instruction::F64Const(match self.op1.unwrap() { WasmOperand::Const(num) => num as f64, _ => panic!()}),
                _ => panic!(),
            }},
            WasmMnemonic::Add => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Add,
                WasmPrefix::i64 => Instruction::I64Add,
                WasmPrefix::f32 => Instruction::F32Add,
                WasmPrefix::f64 => Instruction::F64Add,
                _ => panic!()
            }},
            WasmMnemonic::Mul => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Mul,
                WasmPrefix::i64 => Instruction::I64Mul,
                WasmPrefix::f32 => Instruction::F32Mul,
                WasmPrefix::f64 => Instruction::F64Mul,
                _ => panic!()
            }},
            WasmMnemonic::Sub => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Sub,
                WasmPrefix::i64 => Instruction::I64Sub,
                WasmPrefix::f32 => Instruction::F32Sub,
                WasmPrefix::f64 => Instruction::F64Sub,
                _ => panic!()
            }},
            WasmMnemonic::Div => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Div,
                WasmPrefix::f64 => Instruction::F64Div,
                _ => panic!()
            }},
            WasmMnemonic::Divs => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32DivS,
                WasmPrefix::i64 => Instruction::I64DivS,
                _ => panic!()
            }},
            WasmMnemonic::Divu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32DivU,
                WasmPrefix::i64 => Instruction::I64DivU,
                _ => panic!()
            }},
            WasmMnemonic::Rems => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32RemS,
                WasmPrefix::i64 => Instruction::I64RemS,
                _ => panic!()
            }},
            WasmMnemonic::Remu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32RemU,
                WasmPrefix::i64 => Instruction::I64RemU,
                _ => panic!()
            }},
            WasmMnemonic::Return => Instruction::Return,
            WasmMnemonic::Eq => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Eq,
                WasmPrefix::i64 => Instruction::I64Eq,
                WasmPrefix::f32 => Instruction::F32Eq,
                WasmPrefix::f64 => Instruction::F64Eq,
                _ => panic!()
            }},
            WasmMnemonic::Ne => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Ne,
                WasmPrefix::i64 => Instruction::I64Ne,
                WasmPrefix::f32 => Instruction::F32Ne,
                WasmPrefix::f64 => Instruction::F64Ne,
                _ => panic!()
            }},
            WasmMnemonic::Gt => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Gt,
                WasmPrefix::f64 => Instruction::F64Gt,
                _ => panic!()
            }},
            WasmMnemonic::Gts => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32GtS,
                WasmPrefix::i64 => Instruction::I64GtS,
                _ => panic!()
            }},
            WasmMnemonic::Gtu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32GtU,
                WasmPrefix::i64 => Instruction::I64GtU,
                _ => panic!()
            }},
            WasmMnemonic::Lt => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Lt,
                WasmPrefix::f64 => Instruction::F64Lt,
                _ => panic!()
            }},
            WasmMnemonic::Lts => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32LtS,
                WasmPrefix::i64 => Instruction::I64LtS,
                _ => panic!()
            }},
            WasmMnemonic::Ltu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32LtU,
                WasmPrefix::i64 => Instruction::I64LtU,
                _ => panic!()
            }},
            WasmMnemonic::Ge => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Ge,
                WasmPrefix::f64 => Instruction::F64Ge,
                _ => panic!()
            }},
            WasmMnemonic::Ges => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32GeS,
                WasmPrefix::i64 => Instruction::I64GeS,
                _ => panic!()
            }},
            WasmMnemonic::Geu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32GeU,
                WasmPrefix::i64 => Instruction::I64GeU,
                _ => panic!()
            }},
            WasmMnemonic::Le => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Le,
                WasmPrefix::f64 => Instruction::F64Le,
                _ => panic!()
            }},
            WasmMnemonic::Les => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32LeS,
                WasmPrefix::i64 => Instruction::I64LeS,
                _ => panic!()
            }},
            WasmMnemonic::Leu => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32LeU,
                WasmPrefix::i64 => Instruction::I64LeU,
                _ => panic!()
            }},
            WasmMnemonic::And => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32And,
                WasmPrefix::i64 => Instruction::I64And,
                _ => panic!()
            }},
            WasmMnemonic::Xor => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Xor,
                WasmPrefix::i64 => Instruction::I64Xor,
                _ => panic!()
            }},
            WasmMnemonic::Or => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Or,
                WasmPrefix::i64 => Instruction::I64Or,
                _ => panic!()
            }},
            WasmMnemonic::Shl => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32Shl,
                WasmPrefix::i64 => Instruction::I64Shl,
                _ => panic!()
            }},
            WasmMnemonic::Shrs => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32ShrS,
                WasmPrefix::i64 => Instruction::I64ShrS,
                _ => panic!()
            }},
            WasmMnemonic::Shru => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32ShrU,
                WasmPrefix::i64 => Instruction::I64ShrU,
                _ => panic!()
            }},
            WasmMnemonic::Neg => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32Neg,
                WasmPrefix::f64 => Instruction::F64Neg,
                _ => panic!(),
            }},
            WasmMnemonic::Extends => Instruction::I64Extend32S,
            WasmMnemonic::Extendu => Instruction::I64ExtendI32U,
            WasmMnemonic::Wrap => Instruction::I32WrapI64,
            WasmMnemonic::Promote => Instruction::F64PromoteF32,
            WasmMnemonic::Demote => Instruction::F32DemoteF64,
            WasmMnemonic::ConvertI32s => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32ConvertI32S,
                WasmPrefix::f64 => Instruction::F64ConvertI32S,
                _ => panic!(),
            }},
            WasmMnemonic::ConvertI32u => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32ConvertI32U,
                WasmPrefix::f64 => Instruction::F64ConvertI32U,
                _ => panic!(),
            }},
            WasmMnemonic::ConvertI64s => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32ConvertI64S,
                WasmPrefix::f64 => Instruction::F64ConvertI64S,
                _ => panic!(),
            }},
            WasmMnemonic::ConvertI64u => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::f32 => Instruction::F32ConvertI64U,
                WasmPrefix::f64 => Instruction::F64ConvertI64U,
                _ => panic!(),
            }},
            WasmMnemonic::TruncF32s => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32TruncF32S,
                WasmPrefix::i64 => Instruction::I64TruncF32S,
                _ => panic!(),
            }},
            WasmMnemonic::TruncF32u => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32TruncF32U,
                WasmPrefix::i64 => Instruction::I64TruncF32U,
                _ => panic!(),
            }},
            WasmMnemonic::TruncF64s => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32TruncF64S,
                WasmPrefix::i64 => Instruction::I64TruncF64S,
                _ => panic!(),
            }},
            WasmMnemonic::TruncF64u => { let Some(prefix) = self.prefix else { unreachable!()}; match prefix {
                WasmPrefix::i32 => Instruction::I32TruncF64U,
                WasmPrefix::i64 => Instruction::I64TruncF64U,
                _ => panic!(),
            }},
            WasmMnemonic::Br => { if let Some(WasmOperand::Const(target)) = &self.op1 { Instruction::Br(*target as u32) } else { unreachable!()} },
            WasmMnemonic::Block => Instruction::Block(BlockType::Empty),
            WasmMnemonic::End => Instruction::End,
        }
    }
}