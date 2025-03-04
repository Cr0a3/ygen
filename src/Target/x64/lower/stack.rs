use crate::CodeGen::{MachineInstr, MachineOperand};
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;
use crate::IR::TypeMetadata;

pub(crate) fn x64_lower_salloc(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack allocations need outputs");
    let offset = instr.operands.get(0).expect("stack allocations need one operand");

    let offset = match offset {
        MachineOperand::Imm(imm) => *imm,
        _ => panic!("stack allocations require one operand of type imm")
    };

    let out = out.into();

    if let Operand::Mem(_) = out {
        let tmp = || Operand::Reg( X64Reg::Rax.sub_ty(instr.meta) );

        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Lea, tmp(), Operand::Mem(X64Reg::Rbp - offset as u32)),
            X64MCInstr::with2(Mnemonic::Mov, out, tmp())
        ])
    } else {
        sink.push(
            X64MCInstr::with2(Mnemonic::Lea, out, Operand::Mem(X64Reg::Rbp - offset as u32))
        )
    }
}

pub(crate) fn x64_lower_store(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let ptr = instr.out.expect("stack stores need a output");
    let value = instr.operands.get(0).expect("stack stores need one operand");

    let ptr = ptr.into();
    let value = (*value).into();

    if let Operand::Reg(ptr) = ptr {
        let ptr = Operand::Mem(MemOp {
            base: Some(ptr),
            index: None,
            scale: 1,
            displ: 0,
            rip: false,
        }); 

        if let Operand::Reg(_) = value {
            if instr.meta.float() {
                if instr.meta == TypeMetadata::f32 {
                    sink.push(
                        X64MCInstr::with2(Mnemonic::Movd, ptr, value)
                    );
                } else { // needs to be f64
                    sink.push(
                        X64MCInstr::with2(Mnemonic::Movq, ptr, value)
                    );
                }
            } else {
                sink.push( 
                    X64MCInstr::with2(Mnemonic::Mov, ptr, value)
                )
            }
        } else {
            if instr.meta.float() {
                if instr.meta == TypeMetadata::f32 {
                    sink.extend_from_slice(&[
                        X64MCInstr::with2(Mnemonic::Movd, Operand::Reg(X64Reg::Xmm15), value),
                        X64MCInstr::with2(Mnemonic::Movd, ptr, Operand::Reg(X64Reg::Xmm15))
                    ]);
                } else { // needs to be f64
                    sink.extend_from_slice(&[
                        X64MCInstr::with2(Mnemonic::Movq, Operand::Reg(X64Reg::Xmm15), value),
                        X64MCInstr::with2(Mnemonic::Movq, ptr, Operand::Reg(X64Reg::Xmm15))
                    ]);
                }
            } else {
                sink.extend_from_slice(&[
                    X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value),
                    X64MCInstr::with2(Mnemonic::Mov, ptr, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)))
                ])
            }
        }
    } else {
        if instr.meta.float() {
            if instr.meta == TypeMetadata::f32 {
                if let Operand::Reg(_) = value {
                    sink.push(
                        X64MCInstr::with2(Mnemonic::Movd, ptr, value),
                    );
                } else {
                    sink.extend_from_slice(&[
                        X64MCInstr::with2(Mnemonic::Movd, Operand::Reg(X64Reg::Xmm15), value),
                        X64MCInstr::with2(Mnemonic::Movd, ptr, Operand::Reg(X64Reg::Xmm15))
                    ]);
                }
            } else { // needs to be f64
                if let Operand::Reg(_) = value {
                    sink.push(
                        X64MCInstr::with2(Mnemonic::Movq, ptr, value),
                    );
                } else {
                    sink.extend_from_slice(&[
                        X64MCInstr::with2(Mnemonic::Movq, Operand::Reg(X64Reg::Xmm15), value),
                        X64MCInstr::with2(Mnemonic::Movq, ptr, Operand::Reg(X64Reg::Xmm15))
                    ]);
                }
            }
        } else {
            sink.extend_from_slice(&[
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value),
                X64MCInstr::with2(Mnemonic::Mov, ptr, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)))
            ]);
        }
    }

}

pub(crate) fn x64_lower_load(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack stores need a output");
    let ptr = instr.operands.get(0).expect("stack stores need one operand");

    let ptr: Operand = (*ptr).into();
    
    let out = out.into();

    if let Operand::Reg(out_reg) = out {
        if !out_reg.is_xmm() {
            if let Operand::Reg(ptr) = ptr {
                sink.push( 
                    X64MCInstr::with2(Mnemonic::Mov, out, Operand::Mem(MemOp {
                        base: Some(ptr),
                        index: None,
                        scale: 1,
                        displ: 0,
                        rip: false,
                    }))
                )
            } else {
                sink.push( 
                    X64MCInstr::with2(Mnemonic::Mov, out, ptr)
                );
            }
        } else {
            if instr.meta == TypeMetadata::f32 {
                if let Operand::Reg(ptr) = ptr {
                    sink.push( 
                        X64MCInstr::with2(Mnemonic::Movd, out, Operand::Mem(MemOp {
                            base: Some(ptr),
                            index: None,
                            scale: 1,
                            displ: 0,
                            rip: false,
                        }))
                    )
                } else {
                    sink.push( 
                        X64MCInstr::with2(Mnemonic::Movd, out, ptr)
                    );
                }
            } else { // xmm now should have a f64 cuz of check for f32
                if let Operand::Reg(ptr) = ptr {
                    sink.push( 
                        X64MCInstr::with2(Mnemonic::Movq, out, Operand::Mem(MemOp {
                            base: Some(ptr),
                            index: None,
                            scale: 1,
                            displ: 0,
                            rip: false,
                        }))
                    )
                } else {
                    sink.push( 
                        X64MCInstr::with2(Mnemonic::Movq, out, ptr)
                    );
                }
            }
        }
    }
    else {
        if instr.meta.float() {
            if instr.meta == TypeMetadata::f32 {
                sink.push( 
                    X64MCInstr::with2(Mnemonic::Movd, Operand::Reg(X64Reg::Xmm15), ptr)
                );
                sink.push(
                    X64MCInstr::with2(Mnemonic::Movd, out, Operand::Reg(X64Reg::Xmm15))
                );
            } else {
                sink.push( 
                    X64MCInstr::with2(Mnemonic::Movq, Operand::Reg(X64Reg::Xmm15), ptr)
                );
                sink.push(
                    X64MCInstr::with2(Mnemonic::Movq, out, Operand::Reg(X64Reg::Xmm15))
                );
            }
        } else {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta)), ptr)
            );
        
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta)))
            );
        }
    }

}
