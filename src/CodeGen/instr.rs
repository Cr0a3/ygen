use std::error::Error;
use std::fmt::{Debug, Display};
use std::any::Any;
use crate::prelude::CmpMode;
use crate::Obj::Link;
use crate::IR::{BlockId, Type, TypeMetadata};

use super::reg::Reg;
use super::{CompilationHelper, VarLocation};

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

    /// turns the instruction into an floating point instruction if needed (ops are floats)
    pub fn turn_into_float_if_needed(&mut self) {
        let uses_fp = if 
            self.meta == TypeMetadata::f32 || 
            self.meta == TypeMetadata::f64 { true } else { false };

        if uses_fp {
            self.mnemonic = match self.mnemonic {
                MachineMnemonic::Move => MachineMnemonic::FMove,
                MachineMnemonic::Add => MachineMnemonic::FAdd,
                MachineMnemonic::And => MachineMnemonic::FAnd,
                MachineMnemonic::Div => MachineMnemonic::FDiv,
                MachineMnemonic::Mul => MachineMnemonic::FMul,
                MachineMnemonic::Or =>  MachineMnemonic::FOr,
                MachineMnemonic::Sub => MachineMnemonic::FSub,
                MachineMnemonic::Xor => MachineMnemonic::FXor,
                MachineMnemonic::Rem => MachineMnemonic::FRem,
                MachineMnemonic::Neg => MachineMnemonic::FNeg,
                MachineMnemonic::Shl => MachineMnemonic::FShl,
                MachineMnemonic::Shr => MachineMnemonic::FShr,
                MachineMnemonic::Compare(mode) => MachineMnemonic::FCompare(mode),
                _ => todo!("{}", self)
            }
        }
    }

    /// Fixes the instruction imm based on the rules
    /// 
    /// Returns the fixed machine instr (maybe some got added so)
    pub fn fix_const_imm(&mut self, helper: &mut CompilationHelper, module: &mut crate::prelude::Module) -> Vec<MachineInstr> {
        if !self.meta.float() {
            return vec![self.to_owned()];
        }

        let mut out = Vec::new();

        let mut index = 0;

        for operand in &mut self.operands {
            if let MachineOperand::Imm(imm) = operand {
                let imm = *imm;

                let constant = module.addConst(&format!(".cimm{}", index));
                constant.private();
                constant.data = Vec::from(imm.to_ne_bytes());

                let stack_off = helper.alloc.alloc_stack(self.meta);

                let mut adrm = MachineInstr::new(MachineMnemonic::AdressLoad(constant.name.to_owned()));
                adrm.set_out(stack_off.into());

                adrm.meta = TypeMetadata::ptr;

                out.push(adrm);

                *operand = stack_off.into();

                index += 1;
            }
        }

        out.push(self.to_owned());
        
        out
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
#[derive(Debug, Clone, Copy)]
pub enum MachineOperand {
    /// a number
    Imm(f64),
    /// a register
    Reg(Reg),
    /// stack offset
    Stack(i64),
}

impl PartialEq for MachineOperand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Imm(l0), Self::Imm(r0)) => l0 == r0,
            (Self::Reg(l0), Self::Reg(r0)) => l0 == r0,
            (Self::Stack(l0), Self::Stack(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for MachineOperand {}

impl std::hash::Hash for MachineOperand {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Display for MachineOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MachineOperand::Imm(imm) => format!("{:#x?}", imm),
            MachineOperand::Reg(reg) => format!("{:?}", reg),
            MachineOperand::Stack(off) => format!("sp - {:#x?}", off),
        })
    }
}

impl From<VarLocation> for MachineOperand {
    fn from(location: VarLocation) -> Self {
        match location {
            VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            VarLocation::Mem(mem) => MachineOperand::Stack(mem),
        }
    }
}

impl From<&VarLocation> for MachineOperand {
    fn from(value: &VarLocation) -> Self {
        (*value).into()
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
    Rem,
    Neg,
    Shl,
    Shr,

    FMove,
    FAdd,
    FAnd,
    FDiv,
    FMul,
    FOr,
    FSub,
    FXor,
    FRem,
    FNeg,
    FShl,
    FShr,
    FCompare(CmpMode),

    BrCond(/*if yes*/String, /*if no*/String),
    Compare(CmpMode),

    Zext,
    Downcast,

    Call(String),
    Br(String),
    Return,

    AdressLoad(String),
    StackAlloc,

    Store,
    Load,

    Prolog,
    Epilog,

    /// stack arg
    Push,

    /// stack arg cleanup
    PushCleanup,

    CallStackPrepare,
    CallStackRedo,

    AdrMove,

    Switch(Vec<(Type, BlockId)>),

    /// out: out
    /// op0: variable
    /// op1: thingy to get moved
    MovIfZero,

    /// out: out
    /// op0: variable
    /// op1: thingy to get moved
    MovIfNotZero,
}

impl MachineMnemonic {
    /// Returns the name of the mnemonic
    pub fn name(&self) -> String {
        match self {
            MachineMnemonic::Move =>                "move",
            MachineMnemonic::Add =>                 "add",
            MachineMnemonic::And =>                 "and",
            MachineMnemonic::Div =>                 "div",
            MachineMnemonic::Mul =>                 "mul",
            MachineMnemonic::Or =>                  "or",
            MachineMnemonic::Sub =>                 "sub",
            MachineMnemonic::Xor =>                 "xor",
            MachineMnemonic::Rem =>                 "rem",
            MachineMnemonic::Zext =>                "zext",
            MachineMnemonic::Downcast =>            "dwcast",
            MachineMnemonic::Call(_) =>             "call",
            MachineMnemonic::Return =>              "return",
            MachineMnemonic::AdressLoad(_) =>       "adrload",
            MachineMnemonic::Br(_) =>               "br",
            MachineMnemonic::BrCond(_, _) =>        "comparebr",
            MachineMnemonic::Compare(_) =>          "compare",
            MachineMnemonic::Prolog =>              "prolog",
            MachineMnemonic::Epilog =>              "epilog",
            MachineMnemonic::StackAlloc =>          "salloc",
            MachineMnemonic::Store =>               "store",
            MachineMnemonic::Load =>                "load",
            MachineMnemonic::Push =>                "push",
            MachineMnemonic::PushCleanup =>         "clean_push",
            MachineMnemonic::CallStackPrepare =>    "callsprep",
            MachineMnemonic::CallStackRedo =>       "callspred",
            MachineMnemonic::AdrMove =>             "adrmov",    
            MachineMnemonic::Switch(_) =>           "switch",
            MachineMnemonic::Neg =>                 "neg",
            MachineMnemonic::MovIfZero =>           "cmovz",
            MachineMnemonic::MovIfNotZero =>        "cmovnz",
            MachineMnemonic::Shl =>                 "shl",
            MachineMnemonic::Shr =>                 "shr",
            MachineMnemonic::FMove =>               "fmove",
            MachineMnemonic::FAdd =>                "fadd",
            MachineMnemonic::FAnd =>                "fand",
            MachineMnemonic::FDiv =>                "fdiv",
            MachineMnemonic::FMul =>                "fmul",
            MachineMnemonic::FOr =>                 "for",
            MachineMnemonic::FSub =>                "fsub",
            MachineMnemonic::FXor =>                "fxor",
            MachineMnemonic::FRem =>                "fren",
            MachineMnemonic::FNeg =>                "fneg",
            MachineMnemonic::FShl =>                "fshl",
            MachineMnemonic::FShr =>                "fshr",
            MachineMnemonic::FCompare(_) =>         "fcompare",
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

    /// 
    fn clone_box(&self) -> Box<dyn MCInstr>;
}

impl Clone for Box<dyn MCInstr> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn MCInstr> {
    fn eq(&self, other: &Self) -> bool {
        self.dump().unwrap_or(vec![]) == other.dump().unwrap_or(vec![])
    }
}

impl Eq for Box<dyn MCInstr> {}


/// a doc comment in the generated assembly code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCDocInstr {
    msg: String,
}

impl MCDocInstr {
    /// creates a new documentation instruction
    pub fn doc(msg: String) -> Box<dyn MCInstr> {
        Box::new( Self {
            msg: msg
        } )
    }
}

impl Display for MCDocInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "// {}", self.msg)
    }
}

impl MCInstr for MCDocInstr {
    fn dump(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![format!("// {}", self.msg)])
    }

    fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn Error>> {
        Ok((vec![], None))
    }

    fn clone_box(&self) -> Box<dyn MCInstr> {
        Box::new( self.clone() )
    }
}