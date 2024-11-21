use crate::Target::Arch;
use crate::IR::{Block, Function};
use super::dag::*;

use crate::{ydbg, YGEN_DEBUG};

/// Builds the dag
pub struct DagBuilder;

impl DagBuilder {
    /// builds the dag for a function
    pub fn build(arch: &Arch, func: &Function) -> DagFunction {
        unsafe {
            super::dag_visitors::DAG_ARCH = *arch;
        }
        
        ydbg!("==== BUILD DAG FOR {} ====", func.name);

        let mut blocks = Vec::new();

        for block in &func.blocks {
            let dag_nodes = DagBuilder::build_block(arch, block);
            blocks.push((block.id(), dag_nodes));
        }


        let dag = DagFunction {
            blocks: blocks,
        };

        if unsafe { YGEN_DEBUG } {
            ydbg!("{}", dag);
        }

        dag
    }

    /// builds the dag for a block
    pub fn build_block(arch: &Arch, block: &Block) -> Vec<DagNode> {
        let mut dag = Vec::new();

        for node in &block.nodes {
            if let Some(tvisitor) = crate::Target::own_visitor(arch, node) {
                tvisitor(node, &mut dag);
            }

            node.dag_visitor(&mut dag);
        }

        dag
    }
}