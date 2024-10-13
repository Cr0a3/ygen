use crate::CodeGen::MachineInstr;
use crate::Target::x64::instr::*;
use crate::Target::x64::x64Reg;
use super::fmove::FInstrVariant;
use crate::IR::TypeMetadata;

macro_rules! FMathLowerImpl {
    ($func:tt, $f32m:expr, $f64m:expr) => {
        pub(crate) fn $func(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
            let out = instr.out.expect("expected valid out for math instrs");
            let op1 = instr.operands.get(0).expect("expected valid op1 for math instrs");
            let op2 = instr.operands.get(1).expect("expected valid op2 for math instrs");
            
            let op1: Operand = (*op1).into();
            let op2: Operand = (*op2).into();
            let out: Operand = out.into();

            let mut variant = FInstrVariant::Normal;

            if let Operand::Reg(op1) = op1 {
                if let Operand::Reg(op2) = op2 {
                    if op1.is_xmm() {
                        if op2.is_xmm() {
                            // mov xmm, xmm
                            variant = FInstrVariant::Fp;
                        } else {
                            // mov r, xmm
                            variant = FInstrVariant::RegFp;
                        }
                    } else {
                        if op2.is_xmm() {
                            // mov xmm, r
                            variant = FInstrVariant::FpReg;
                        } else {
                            // mov r, r
                            variant = FInstrVariant::Normal;
                        }
                    }
                } else if let Operand::Mem(_) = op2 {
                    if op1.is_xmm() {
                        // mov [mem], xmm
                        variant = FInstrVariant::MemFp;
                    } else {
                        // mov [mem], r
                        variant = FInstrVariant::Normal;
                    }
                } else if let Operand::Imm(_) = op2 {
                    if op1.is_xmm() {
                        // mov xmm, imm
                        variant = FInstrVariant::FpImm;
                    } else {
                        // mov xmm, r
                        variant = FInstrVariant::FpReg;
                    }
                }
            } else if let Operand::Mem(_) = op1 {
                if let Operand::Reg(op2) = op2 {
                    if op2.is_xmm() {
                        // mov xmm, [mem]
                        variant = FInstrVariant::FpMem;
                    } else {
                        // mov r, [mem]
                        variant = FInstrVariant::Normal;
                    }
                } else if let Operand::Mem(_) = op2 {
                    variant = FInstrVariant::Normal;
                }
            } else if let Operand::Imm(_) = op1 {
                if let Operand::Reg(op2) = op2 {
                    if op2.is_xmm() {
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
                FInstrVariant::Fp =>     {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpReg =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movd, Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::RegFp =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movd,     Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpMem =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss,    Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::MemFp =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss,    Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpImm =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss,    Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2(Mnemonic::Mov,      Operand::Reg(x64Reg::Eax), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movd,     out.clone(), Operand::Reg(x64Reg::Eax)));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), out.clone()));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::Normal => {},
                FInstrVariant::MemImm => {
                    sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                    sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Xmm15), Operand::Reg(x64Reg::Rax)));
                    sink.push(X64MCInstr::with2($f32m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, out, Operand::Reg(x64Reg::Xmm15)));
                },
            };
        } else {
            // f64
            match variant {
                FInstrVariant::Fp =>     {
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpReg =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movq, Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::RegFp =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movq,     Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpMem =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss,    Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::MemFp =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd,    Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2(Mnemonic::Movq,     out.clone(), op2));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), out.clone()));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::FpImm =>  {
                    sink.push(X64MCInstr::with2(Mnemonic::Movss, Operand::Reg(x64Reg::Xmm15), op1));
                    sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op2));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), Operand::Reg(x64Reg::Rax)));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
                FInstrVariant::Normal => {},
                FInstrVariant::MemImm => {
                    sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op1));
                    sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Xmm15), Operand::Reg(x64Reg::Rax)));
                    sink.push(X64MCInstr::with2($f64m, Operand::Reg(x64Reg::Xmm15), op2));
                    sink.push(X64MCInstr::with2(Mnemonic::Movsd, out, Operand::Reg(x64Reg::Xmm15)));
                },
            };
        }
        }
    };
}

FMathLowerImpl!(x64_lower_fadd, Mnemonic::Addss, Mnemonic::Addsd);
FMathLowerImpl!(x64_lower_fdiv, Mnemonic::Divss, Mnemonic::Divsd);
FMathLowerImpl!(x64_lower_fmul, Mnemonic::Mulss, Mnemonic::Mulsd);
FMathLowerImpl!(x64_lower_fsub, Mnemonic::Subss, Mnemonic::Subsd);