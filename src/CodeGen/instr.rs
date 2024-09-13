use std::error::Error;
use std::fmt::{Debug, Display};
use std::any::Any;
use crate::prelude::CmpMode;
use crate::Obj::Link;
use crate::IR::TypeMetadata;

use super::reg::Reg;

/// a low level instruction which is portable over platforms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineInstr {
    pub(crate) operands: Vec<MachineOperand>,
    pub(crate) out: Option<MachineOperand>,
    pub(crate) mnemonic: MachineMnemonic,
    pub(crate) meta: TypeMetadata,
}

impl MachineInstr {
    /// Creates a new machine instr
    pub fn new(mne: MachineMnemonic) -> Self {
        Self {
            mnemonic: mne,
            operands: vec![],
            out: None,
            meta: TypeMetadata::Void,
        }
    }

    /// Adds an operand
    pub fn add_operand(&mut self, op: MachineOperand) {
        self.operands.push( op );
    }

    /// Sets the output of the instr
    pub fn set_out(&mut self, out: MachineOperand) {
        self.out = Some(out);
    }
}

impl Display for MachineInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ops = String::new();

        let mut before = false;

        for op in &self.operands {
            let a = if before { ", " } else { "" };

            ops.push_str(&format!("{}{}", a, op));
            before = true
        }

        let mut out_fmt = String::new();

        if let Some(out) = self.out {
            out_fmt = format!(" => {}", out);
        }

        write!(f, "{} {}{}", self.mnemonic, ops, out_fmt)
    }
}

/// a low level operand which is portable over platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineOperand {
    /// a number
    Imm(i64),
    /// a register
    Reg(Reg),
}

impl Display for MachineOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MachineOperand::Imm(imm) => format!("{:#x?}", imm),
            MachineOperand::Reg(reg) => format!("{:?}", reg),
        })
    }
}

/// The mnemonic to use
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MachineMnemonic {
    Move,
    
    Add,
    And,
    Div,
    Mul,
    Or,
    Sub,
    Xor,

    BrCond(/*if yes*/String, /*if no*/String),
    Compare(CmpMode),

    Zext,
    Downcast,

    Call(String),
    Br(String),
    Return,

    AdressLoad(String),
}

impl MachineMnemonic {
    /// Returns the name of the mnemonic
    pub fn name(&self) -> String {
        match self {
            MachineMnemonic::Move => "move",
            MachineMnemonic::Add => "add",
            MachineMnemonic::And => "and",
            MachineMnemonic::Div => "div",
            MachineMnemonic::Mul => "mul",
            MachineMnemonic::Or => "or",
            MachineMnemonic::Sub => "sub",
            MachineMnemonic::Xor => "xor",
            MachineMnemonic::Zext => "zext",
            MachineMnemonic::Downcast => "dwcast",
            MachineMnemonic::Call(_) => "call",
            MachineMnemonic::Return => "return",
            MachineMnemonic::AdressLoad(_) => "adrload",
            MachineMnemonic::Br(_) => "br",
            MachineMnemonic::BrCond(_, _) => "comparebr",
            MachineMnemonic::Compare(_) => "compare",
        }.to_string()
    }
}

impl Display for MachineMnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MachineMnemonic::Call(target) => format!("{} {}", self.name(), target),
            MachineMnemonic::AdressLoad(adr) => format!("{} {}", self.name(), adr),
            _ => self.name()
        })
    }
}

/// a platform specifc instruction
pub trait MCInstr: Any + Debug + Display {
    /// dumps the instruction into a assembly string
    fn dump(&self) -> Result<Vec<String>, Box<dyn Error>>;

    /// encodes the instruction
    fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn Error>>;
}