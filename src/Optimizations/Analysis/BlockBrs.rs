use std::collections::HashMap;
use crate::IR::{Block, BlockId, ir::*};

/// analyzes which blocks can branch to which other blocks
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockBranchAnalysis<'a> {
    pub(crate) branches: HashMap<&'a String, Vec<&'a BlockId>>,
}

impl<'a> BlockBranchAnalysis<'a> {
    /// analyzes which blocks could branch to other blocks
    /// 
    /// #### NOTE:
    /// 
    /// It does not ignore dead code:
    /// ```no-run
    /// ret void 0
    /// br some_block
    /// ```
    /// 
    /// Would still be identified as a branch to block `some_block`
    pub fn analyze(blocks: &'a Vec<Block>) -> Self {
        let mut branches = HashMap::new();

        for block in blocks {
            let mut brs = Vec::new();

            for node in &block.nodes {
                if let Some(br) = node.as_any().downcast_ref::<Br>() {
                    brs.push(&br.inner1);
                } else if let Some(br) = node.as_any().downcast_ref::<BrCond>() {
                    brs.push(&br.inner2);
                    brs.push(&br.inner3);
                } else if let Some(switch) = node.as_any().downcast_ref::<Switch>() {
                    brs.push(&switch.default);   
                    for (_, case) in &switch.cases {
                        brs.push(case);
                    }
                }
            }

            branches.insert(&block.name, brs);
        }

        Self {
            branches: branches
        }
    }

    /// checks if the block ... branches to block ...
    /// 
    /// #### NOTE:
    /// 
    /// `analyze` needs to be run first
    pub fn branches_to(&self, from: &BlockId, to: &BlockId) -> bool {
        let mut branches = false;

        for (br_start, br_to) in &self.branches {
            if br_start == &&from.name {
                if br_to.contains(&to) {
                    branches = true;
                    break;
                }
            }
        }

        branches
    }

    /// Returns if the target block branches to itself (or a block which branches to it)
    /// 
    /// Helps with loop analysis
    /// 
    /// #### NOTE:
    /// 
    /// `analyze` needs to be run first
    /// 
    /// it has a customizable depth. Which means if we give it a depth of 1
    /// that then following code will be recoginized as a branch:
    /// 
    /// ```no-run
    /// block1:
    ///   br block2
    /// 
    /// block2:
    ///    br block2
    /// ```
    /// 
    /// but:
    /// ```no-run
    /// block1: 
    ///     br block2
    /// 
    /// block2:
    ///     br block3
    /// 
    /// block3:
    ///     br block4
    /// 
    /// block4:
    ///     br block1
    /// ```
    /// won't be detected
    pub fn branches_to_itself(&self, target: &BlockId, depth: i32) -> bool {
        // HOW IT WORKS:

        // 1. we create a list of blocks which branch to the target block
        // 2. we iterate over all blocks and look if it is the target block
        //    then we look if the branches branch to a branch with branches
        //    to the target branch with a given depth
        //
            
        let mut branches = false;

        let mut covered = Vec::new();

        for (br_from, br_to) in &self.branches {
            if br_to.contains(&target) {
                covered.push(BlockId(br_from.to_owned().to_owned()));
            }
        }

        for _ in 0..depth {
            for (br_from, br_to) in &self.branches {
                if covered.contains(&BlockId(br_from.to_owned().to_owned())) {
                    continue;
                }
                
                let mut coverd = false;
                for cover in &covered {
                    if br_to.contains(&cover) {
                        coverd = true;
                    }
                }
    
                if coverd {
                    covered.push(BlockId(br_from.to_owned().to_owned()));
                }
            }
        }

        for (br_from, br_to) in &self.branches {
            if br_from != &&target.name {
                for br in br_to {
                    if covered.contains(br.to_owned()) {
                        branches = true;
                        break;
                    }
                }
            }
        }

        branches
    }
}