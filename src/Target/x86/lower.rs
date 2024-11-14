use std::collections::HashMap;

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

pub(super) fn x86_lower(func: dag::DagFunction) -> HashMap<BlockId, Vec<Box<dyn McInstr>>> {
    let mut blocks = HashMap::new();
    
    for (name, nodes) in &func.blocks {
        let mut asm: Vec<X64Instr> = Vec::new();
        for node in nodes {
            auto_gen::compile(&mut asm, node.to_owned());
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