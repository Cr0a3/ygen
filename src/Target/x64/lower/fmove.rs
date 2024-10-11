use crate::CodeGen::MachineInstr;
use crate::Target::x64::instr::*;
use crate::Target::x64Reg;
use crate::IR::TypeMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FInstrVariant {
    /// mov xmm, xmm
    Fp,
    /// mov xmm, r
    FpReg,
    /// mov xmm, imm
    FpImm,
    /// mov r, xmm
    RegFp,
    /// mov xmm, [mem]
    FpMem,
    /// mov [mem], xmm
    MemFp,
    /// mov r, r
    Normal,
    /// mov [mem], imm
    MemImm,
}

pub(crate) fn x64_lower_fmove(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let op1: Operand = (*op1).into();
    let out: Operand = out.into();

    let mut variant = FInstrVariant::Normal;

    if let Operand::Reg(op1) = op1 {
        if let Operand::Reg(out) = out {
            if op1.is_xmm() {
                if out.is_xmm() {
                    // mov xmm, xmm
                    variant = FInstrVariant::Fp;
                } else {
                    // mov r, xmm
                    variant = FInstrVariant::RegFp;
                }
            } else {
                if out.is_xmm() {
                    // mov xmm, r
                    variant = FInstrVariant::FpReg;
                } else {
                    // mov r, r
                    variant = FInstrVariant::Normal;
                }
            }
        } else if let Operand::Mem(_) = out {
            if op1.is_xmm() {
                // mov [mem], xmm
                variant = FInstrVariant::MemFp;
            } else {
                // mov [mem], r
                variant = FInstrVariant::Normal;
            }
        }
    } else if let Operand::Mem(_) = op1 {
        if let Operand::Reg(out) = out {
            if out.is_xmm() {
                // mov xmm, [mem]
                variant = FInstrVariant::FpMem;
            } else {
                // mov r, [mem]
                variant = FInstrVariant::Normal;
            }
        } else if let Operand::Mem(_) = out {
            variant = FInstrVariant::Normal;
        }
    } else if let Operand::Imm(_) = op1 {
        if let Operand::Reg(out) = out {
            if out.is_xmm() {
                // mov xmm, imm
                variant = FInstrVariant::FpImm;
            } else {
                // mov r, imm
                variant = FInstrVariant::Normal;
            }
        } else if let Operand::Mem(_) = out {
            variant = FInstrVariant::MemImm;
        }
    }

    if TypeMetadata::f32 == instr.meta { 
        // f32
        match variant {
            FInstrVariant::Fp =>     sink.push(X64MCInstr::with2(Mnemonic::Movups,   out, op1)),
            FInstrVariant::FpReg =>  sink.push(X64MCInstr::with2(Mnemonic::Movd,     out, op1)),
            FInstrVariant::RegFp =>  sink.push(X64MCInstr::with2(Mnemonic::Movd,     out, op1)),
            FInstrVariant::FpMem =>  sink.push(X64MCInstr::with2(Mnemonic::Movss,    out, op1)),
            FInstrVariant::MemFp =>  sink.push(X64MCInstr::with2(Mnemonic::Movss,    out, op1)),
            FInstrVariant::FpImm =>  {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                sink.push(X64MCInstr::with2(Mnemonic::Movd, out, Operand::Reg(x64Reg::Rax)));
            },
            FInstrVariant::Normal => sink.push(X64MCInstr::with2(Mnemonic::Mov, out, op1)),
            FInstrVariant::MemImm => {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax)));
            },
        };
    } else {
        // f64
        match variant {
            FInstrVariant::Fp =>     sink.push(X64MCInstr::with2(Mnemonic::Movupd,   out, op1)),
            FInstrVariant::FpReg =>  sink.push(X64MCInstr::with2(Mnemonic::Movq,     out, op1)),
            FInstrVariant::RegFp =>  sink.push(X64MCInstr::with2(Mnemonic::Movq,     out, op1)),
            FInstrVariant::FpMem =>  sink.push(X64MCInstr::with2(Mnemonic::Movsd,    out, op1)),
            FInstrVariant::MemFp =>  sink.push(X64MCInstr::with2(Mnemonic::Movsd,    out, op1)),
            FInstrVariant::FpImm =>  {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                sink.push(X64MCInstr::with2(Mnemonic::Movq, out, Operand::Reg(x64Reg::Rax)));
            },
            FInstrVariant::Normal => sink.push(X64MCInstr::with2(Mnemonic::Mov, out, op1)),
            FInstrVariant::MemImm => {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax)));
            },
        };
    }
}