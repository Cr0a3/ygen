use crate::CodeGen::{MachineInstr, MachineOperand};
use crate::Target::x64Reg;
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
        let tmp = || Operand::Reg( x64Reg::Rax.sub_ty(instr.meta) );

        sink.push(
            X64MCInstr::with2(Mnemonic::Lea, tmp(), Operand::Mem(x64Reg::Rbp - offset as u32))
        );
        sink.push(
            X64MCInstr::with2(Mnemonic::Mov, out, tmp())
        )
    } else {
        sink.push(
            X64MCInstr::with2(Mnemonic::Lea, out, Operand::Mem(x64Reg::Rbp - offset as u32))
        )
    }
}

pub(crate) fn x64_lower_store(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let ptr = instr.out.expect("stack stores need a output");
    let value = instr.operands.get(0).expect("stack stores need one operand");

    let ptr = ptr.into();
    let value = (*value).into();

    if let Operand::Reg(ptr) = ptr {
        if let Operand::Reg(_) = value {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }), value)
            )
        } else {
            sink.push(
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value)
            );
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }), Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
            )
        }
    } else {
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value)
        );
    
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, ptr, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
        );
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
            }
        }
    }
    else {
        if instr.meta.float() {
            if instr.meta == TypeMetadata::f32 {

            } else {
                
            }
        } else {
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta)), ptr)
            );
        
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta)))
            );
        }
    }

}
