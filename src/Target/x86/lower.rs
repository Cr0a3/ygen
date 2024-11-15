use std::collections::HashMap;

use crate::ydbg;
use crate::CodeGen::dag::DagNode;
use crate::CodeGen::regalloc_iterated_col::ItRegCoalAlloc;
use crate::IR::BlockId;
use crate::{CodeGen::dag, Target::instr::McInstr};
use super::asm::*;

#[allow(warnings)]
mod auto_gen {
    use super::super::asm::X64Instr as Asm;
    use super::super::asm::X64Mnemonic as Mnemonic;
    use crate::CodeGen::dag::*;
    use super::super::asm::*;
    include!("dag.def");
}

pub(super) fn x86_lower(func: &mut dag::DagFunction, alloc: &mut ItRegCoalAlloc) -> HashMap<BlockId, Vec<Box<dyn McInstr>>> {
    let mut blocks = HashMap::new();
    
    for (name, nodes) in &mut func.blocks {
        let mut asm: Vec<X64Instr> = Vec::new();
        for node in nodes {
            let tmps = x86_tmps(node);

            alloc.apply(node);

            let mut node_asm: Vec<X64Instr> = Vec::new();
            auto_gen::compile(&mut asm, node.to_owned());

            super::alloc::resolve(tmps, &mut node_asm);

        };

        // now turn the asemmbly into `dyn McInstr`
        let mut mc_instrs: Vec<Box<dyn McInstr>> = Vec::new();

        for instr in asm {
                mc_instrs.push(Box::new(instr));
        }

        blocks.insert(name.to_owned(), mc_instrs);
    };

    blocks
}

pub(super) fn x86_tmps(node: &DagNode) -> Vec<dag::DagTmpInfo> {
    ydbg!("[X86] requesting tmps for {}", node.get_opcode());
    
    Vec::new()
}