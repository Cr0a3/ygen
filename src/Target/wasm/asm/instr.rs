use std::fmt::Display;

use crate::CodeGen::MCInstr;

/// A wasm instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmMCInstr {
    pub(crate) mnemonic: WasmMnemonic,
    pub(crate) op1: Option<WasmOperand>,
}

impl WasmMCInstr {
    /// Creates an wasm instruction without any operands
    pub fn with0(mne: WasmMnemonic) -> Self {
        Self {
            mnemonic: mne,
            op1: None,
        }
    }

    /// Creates an wasm instruction with 1 operand
    pub fn with1(mne: WasmMnemonic, op1: WasmOperand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op1),
        }
    }

    pub(crate) fn encode(&self) -> Result<(Vec<u8>, Option<crate::Obj::Link>), Box<dyn std::error::Error>> {
        let mut encoded: Vec<u8> = Vec::new();

        match self.mnemonic {
            
        }

        Ok((encoded, None))
        //todo!("ygen doesn't support wasm instruction encoding yet")
    }
}

/// A webassembly mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmMnemonic {

}

/// A webassembly operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmOperand {

}

impl Display for WasmMCInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = String::new();

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