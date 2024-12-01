//use opt::X86BasicOpt;

use std::collections::HashMap;

use crate::ydbg;
use crate::CodeGen::dag::DagNode;
use crate::CodeGen::reg::TargetReg;
use crate::CodeGen::regalloc_iterated_col::ItRegCoalAlloc;
use crate::IR::BlockId;
use crate::{CodeGen::dag, Target::instr::McInstr};
use super::asm::*;

#[allow(warnings)]
mod auto_gen {
    use super::super::asm::X86Instr as Asm;
    use super::super::asm::X86Mnemonic as Mnemonic;
    use super::super::asm::X86MemDispl as MemoryDispl;
    use super::super::asm::X86MemOption as MemoryOption;
    use super::super::asm::X86Operand as Operand;
    use super::super::reg::X86Reg as Reg;
    use super::super::reg::X86Reg;
    use crate::CodeGen::dag::*;
    use crate::CodeGen::dag;
    use super::super::asm::*;
    use crate::Target::x86::operation::X86OperationHandler as OperationHandler;

    use crate::CodeGen::dag::OperationHandler as oph;

    fn lower_br(asm: &mut Vec<Asm>, node: DagNode) {
        let DagOpCode::Br(target) = node.get_opcode() else { unreachable!() };
        asm.push( Asm::with1(Mnemonic::Jmp, Operand::Rel(crate::Target::x86::add_rel(target), true)));
    }

    fn lower_breq(asm: &mut Vec<Asm>, node: DagNode) {
        let DagOpCode::BrIfEq(target) = node.get_opcode() else { unreachable!() };
        asm.push( Asm::with1(Mnemonic::Je, Operand::Rel(crate::Target::x86::add_rel(target), true)));
    }

    /// Lowers the end of intenger division
    fn lower_divi(asm: &mut Vec<Asm>, node: DagNode) {
        // At the start our assembly looks like this:

        // mov %t1, $2;
        // mov rax, $1;

        let prep = if node.ty.signed() {
            let mnemonic = match node.ty.byteSize() {
                1 => X86Mnemonic::Cbw,
                2 => X86Mnemonic::Cwd,
                4 => X86Mnemonic::Cdq,
                8 => X86Mnemonic::Cqo,
                _ => panic!(),
            };

            X86Instr::with0(mnemonic)
        } else { 
            X86Instr::with2(Mnemonic::Xor, X86Operand::Reg(X86Reg::Rdx()), X86Operand::Reg(X86Reg::Rdx())) 
        };
        asm.push(prep);

        // we now implemented the rdx reset
        // and only need to add following assembly lines:
        // idiv %t1
        // mov $out, rax
        let div_mnemonic = match node.ty.signed() {
            true => X86Mnemonic::Idiv,
            false => X86Mnemonic::Div,
        };
        asm.push(X86Instr::with1(div_mnemonic, X86Operand::Tmp(1)));
        asm.push(X86Instr::with2(X86Mnemonic::Mov, node.get_out().into(), X86Operand::Reg(X86Reg::Rax())));
    }

    /// lowers the end of intenger rem
    fn lower_remi(asm: &mut Vec<Asm>, node: DagNode) {
        // it's nearly the same only the last instruction is different
        // so we're just going to prentend like it's a division
        lower_divi(asm, node.clone());
        // and then change the last instruction
        asm.pop();
        asm.push(X86Instr::with2(X86Mnemonic::Mov, node.get_out().into(), X86Operand::Reg(X86Reg::Rdx())));
    }

    include!("dag.def");
}

pub(super) fn x86_lower(func: &mut dag::DagFunction, alloc: &mut ItRegCoalAlloc, module: &mut crate::IR::Module) -> Vec<(BlockId, Vec<Box<dyn McInstr>>)> {
    let mut blocks = Vec::new();
    
    for (name, nodes) in &mut func.blocks {
        let mut asm: Vec<X86Instr> = Vec::new();
        for node in nodes {
            alloc.apply(node);

            let mut overwrittes = HashMap::new();
            
            for overwrite in auto_gen::overwrittes(&node) {
                // 1. Check if the value which is overwritten is currently in use
                if !alloc.regs.contains(&crate::CodeGen::reg::Reg::new_x86(overwrite)) {
                    continue;
                }

                // 2. Allocate new spill location
                // we feed random values into the mem processor cuz they will be ignored
                let stack = super::alloc::mem_proc(alloc, &DagNode::new(dag::DagOpCode::Copy, crate::IR::TypeMetadata::Void), crate::IR::TypeMetadata::i64);

                // We now create our instruction
                let instr = super::asm::X86Instr::with2(X86Mnemonic::Mov, stack.into(), X86Operand::Reg(overwrite));

                asm.push(instr);

                overwrittes.insert(overwrite, stack);

                alloc.regs = alloc.regs.iter().filter(|x| {
                    if let TargetReg::X86(x86) = x.reg {
                        x86.variant != overwrite.variant
                    } else { true }
                }).map(|x| x.to_owned()).collect::<Vec<crate::CodeGen::reg::Reg>>();
            }
            
            let mut node_asm: Vec<X86Instr> = Vec::new();
            auto_gen::compile(&mut node_asm, node.to_owned(), module);

            //super::asm::opt::X86BasicOpt::opt(&mut node_asm);

            let tmps = x86_tmps(node);
            super::alloc::resolve(node, tmps, &mut node_asm, alloc);

            asm.extend_from_slice(&node_asm);

            // restore overwritten registers

            for (reg, stack_location) in overwrittes {
                let instr = super::asm::X86Instr::with2(X86Mnemonic::Mov, X86Operand::Reg(reg), stack_location.into());

                asm.push(instr);

                alloc.regs.push(crate::CodeGen::reg::Reg::new_x86(reg));
            }

        };

        //super::asm::opt::X86BasicOpt::opt(&mut asm);

        // now turn the asemmbly into `dyn McInstr`
        let mut mc_instrs: Vec<Box<dyn McInstr>> = Vec::new();

        for instr in asm {
                mc_instrs.push(Box::new(instr));
        }

        blocks.push((name.to_owned(), mc_instrs));
    };

    blocks
}

pub(super) fn x86_tmps(node: &DagNode) -> Vec<dag::DagTmpInfo> {
    let tmps = auto_gen::tmps(node);

    ydbg!("[X86] tmps for `{node}`: {tmps:?}");

    tmps
}

pub(super) fn ov_proc(node: &DagNode) -> Vec<crate::CodeGen::reg::Reg> {
    let overwrittes = auto_gen::overwrittes(node);

    let mut reg_ov = Vec::new();

    for ov in overwrittes {
        reg_ov.push(crate::CodeGen::reg::Reg::new_x86(ov));
    }

    reg_ov
}