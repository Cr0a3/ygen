use crate::Target::{x64Reg, Reg};

use super::instr::MemOp;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RexPrefix {
    pub w: bool,
    pub r: bool,
    pub x: bool,
    pub b: bool,
}

impl RexPrefix {
    pub fn sync(&self, other: RexPrefix) -> Self {
        RexPrefix {
            w: self.w || other.w,
            r: self.r || other.r,
            x: self.r || other.r,
            b: self.b || other.b,
        }
    }

    pub fn none() -> Self {
        RexPrefix { w: false, r: false, x: false, b: false }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandatoryPrefix {
    t16BitOps,
}

pub(crate) fn buildOpcode(mandatory: Option<MandatoryPrefix>, rex: Option<RexPrefix>, op: Vec<u8>) -> Vec<u8> {
    let mut out = vec![];

    if let Some(man) = mandatory {
        out.extend_from_slice(&match man {
            MandatoryPrefix::t16BitOps => vec![0x66],
        })
    }

    if let Some(rex) = rex {
        out.push({
            let mut enc = 1 << 6;
            if rex.w { enc |= 1 << 3}
            if rex.r { enc |= 1 << 2}
            if rex.x { enc |= 1 << 1}
            if rex.b { enc |= 1 << 0}
            enc
        })
    }

    out.extend_from_slice(&op);

    out
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModRm {}

impl ModRm {
    pub fn reg2(reg1: x64Reg, reg2: x64Reg) -> Vec<u8> {
        vec![0b11 << 6 | reg2.enc() << 3 | reg1.enc()]
    }

    pub fn regM(reg: x64Reg, mem: MemOp) -> Vec<u8> {
        let enc = mem.encode(Some(reg.boxed()));
        let mut out = vec![];

        if let Some(_) = mem.index {
            out = vec![enc.0 << 6 | reg.enc() << 3 | 0b100]
        }

        out.extend_from_slice(&enc.1);
        out
    }

    pub fn memR(mem: MemOp, reg: x64Reg) -> Vec<u8> {
        let mut out = vec![mem.encode(Some(reg.boxed())).0 << 6 | reg.enc() << 3 | 0b100];
        out.extend_from_slice(&mem.encode(Some(reg.boxed())).1);
        out
    }

    pub fn regWimm(i: u8, reg: x64Reg) -> Vec<u8> {
        vec![0b11 << 6 | i << 3 | reg.enc()]
    }
}