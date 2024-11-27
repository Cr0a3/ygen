//use opt::X86BasicOpt;

use crate::CodeGen::dag::DagNode;
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
    use super::super::reg::X86Reg;
    use crate::CodeGen::dag::*;
    use crate::CodeGen::dag;
    use super::super::asm::*;
    use crate::Target::x86::operation::X86OperationHandler as OperationHandler;

    use crate::CodeGen::dag::OperationHandler as oph;

    fn lower_br(asm: &mut Vec<Asm>, node: DagNode) {
        let DagOpCode::Br(target) = node.get_opcode() else { unreachable!() };
        asm.push( Asm::with1(Mnemonic::Jmp, Operand::BlockRel(crate::Target::x86::add_block_rel(target))));
    }

    include!("dag.def");
}

pub(super) fn x86_lower(func: &mut dag::DagFunction, alloc: &mut ItRegCoalAlloc, module: &mut crate::IR::Module) -> Vec<(BlockId, Vec<Box<dyn McInstr>>)> {
    let mut blocks = Vec::new();
    
    for (name, nodes) in &mut func.blocks {
        let mut asm: Vec<X86Instr> = Vec::new();
        for node in nodes {
            alloc.apply(node);
            
            let tmps = x86_tmps(node);


            let mut node_asm: Vec<X86Instr> = Vec::new();
            auto_gen::compile(&mut node_asm, node.to_owned(), module);

            //X86BasicOpt::opt(&mut node_asm);

            super::alloc::resolve(tmps, &mut node_asm, alloc);

            asm.extend_from_slice(&node_asm);

        };

        //X86BasicOpt::opt(&mut asm);

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
    auto_gen::tmps(node)
}