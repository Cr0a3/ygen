use std::collections::HashMap;

use crate::Target::Arch;
use crate::IR::{Block, Function};
use super::dag::*;

/// Builds the dag
pub struct DagBuilder;

impl DagBuilder {
    /// builds the dag for a function
    pub fn build(arch: &Arch, func: &Function) -> DagFunction {
        unsafe {
            super::dag_visitors::DAG_ARCH = *arch;
        }

        let mut blocks = HashMap::new();

        for block in &func.blocks {
            let dag_nodes = DagBuilder::build_block(arch, block);
            blocks.insert(block.id(), dag_nodes);
        }

        DagFunction {
            blocks: blocks,
        }
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