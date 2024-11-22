use std::collections::HashMap;

use crate::{ydbg, IR::{ir, Block, BlockId, Function}};

use super::BlockBranchAnalysis;

/// Analysis which blocks are the successors and predecessors
/// of the block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFGAnalysis {
    /// successors  - blocks which follows
    succesors: HashMap<BlockId, Vec<BlockId>>,
    /// predecessors - blocks which were before
    predecessors: HashMap<BlockId, Vec<BlockId>>,
}

impl CFGAnalysis {
    /// Analyses the controll flow graph of the function
    pub fn analyze(func: &Function) -> Self {
        let mut analyzer = Self {
            succesors: HashMap::new(),
            predecessors: HashMap::new()
        };

        for block in &func.blocks {
            analyzer.analyze_block(block);
        }

        analyzer
    }

    fn analyze_block(&mut self, block: &Block) {
        let mut insert = |br_to_block: &BlockId, head: &Block| {
                // block -> br.block (so the successor  of block is br.block)
                // block -> br.block (so the predecessor of br.block is block)

                self.succesors.entry(head.id()).or_insert_with(|| Vec::new()).push(br_to_block.to_owned());
                self.predecessors.entry(br_to_block.to_owned()).or_insert_with(|| Vec::new()).push(head.id());
        };

        for node in &block.nodes {
            if let Some(br) = node.as_any().downcast_ref::<ir::Br>() {
                insert(&br.inner1, block);
            }

            if let Some(br_cond) = node.as_any().downcast_ref::<ir::BrCond>() {
                insert(&br_cond.inner2, block);
                insert(&br_cond.inner3, block);
            }
        }
    }

    /// Returns the sucessors for the given block
    pub fn successors(&self, block: &BlockId) -> &Vec<BlockId> {
        let Some(succs) = self.succesors.get(block) else {
            panic!("unanalysed block: {}", block.name);
        };

        succs
    }

    /// Returns the predeccessors for the given block
    pub fn predeccessors(&self, block: &BlockId) -> &Vec<BlockId> {
        let Some(preds) = self.predecessors.get(block) else {
            panic!("unanalysed block: {}", block.name);
        };

        preds
    }

    /// Returns if the block `dominator` dominates the block `block`
    pub fn dominates(&self, dominator: &BlockId, block: &BlockId) -> bool {
        let Some(succs) = self.succesors.get(block) else {
            panic!("unanalyzed block: {}", block.name)
        };  

        succs.contains(dominator)
    }

    /// Returns if the block `predecator` predecates the block `block`
    pub fn predicates(&self, predecator: &BlockId, block: &BlockId) -> bool {
        let Some(preds) = self.predecessors.get(block) else {
            panic!("unanalyzed block: {}", block.name)
        };  

        preds.contains(predecator)
    }

    /// Returns if the given block branches to the block
    pub fn branches_to(&self, from: &BlockId, source: &BlockId) -> bool {
        let mut branch = false;
        
        for succs in self.successors(from) {
            for succ in self.successors(succs) {

            }
        }

        branch
    }

    /// A back edge exists if a successor block (header) dominates its predecessor (current).
    pub fn back_edge(&self, block: &BlockId) -> bool {
        // block -> itself 

        let mut back_edge = false;
        
        let succs = self.successors(block);
        for succ in succs {
            if succ != block {
                if self.dominates(succ, block) && self.branches_to(block, succ) {
                    back_edge = true;
                    break;
                }
            }
        } 

        if back_edge {
            ydbg!("[CFG] the block {} has a back edge", block.name);
        }

        back_edge
    }

    /// Orders the block vector
    pub fn order(blocks: &Vec<Block>) -> Vec<Block> {
        let mut ordered = Vec::new();

        ordered
    }
}