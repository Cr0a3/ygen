use crate::CodeGen::{MachineInstr, MachineOperand};
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_salloc(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack allocations need outputs");
    let offset = instr.operands.get(0).expect("stack allocations need one operand");

    let offset = match offset {
        MachineOperand::Imm(imm) => *imm,
        _ => panic!("stack allocations require one operand of type imm")
    };

    let out = out.into();

    if let Operand::Mem(_) = out {
        let tmp = || Operand::Reg( x64Reg::Rax );

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
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), value)
            );
            sink.push( 
                X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }), Operand::Reg(x64Reg::Rax))
            )
        }
    } else {
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), value)
        );
    
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, ptr, Operand::Reg(x64Reg::Rax))
        );
    }

}

pub(crate) fn x64_lower_load(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("stack stores need a output");
    let ptr = instr.operands.get(0).expect("stack stores need one operand");

    let ptr = (*ptr).into();
    
    let out = out.into();

    if let Operand::Reg(ptr) = ptr {
        if let Operand::Reg(_) = out {
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
                X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Mem(MemOp {
                    base: Some(ptr),
                    index: None,
                    scale: 1,
                    displ: 0,
                    rip: false,
                }))
            );

            sink.push(
                X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
            );
        }
    } else {
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), ptr)
        );
    
        sink.push( 
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)))
        );
    }

}
