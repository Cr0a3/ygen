use crate::prelude::Ir;
use crate::IR::{BlockId, Function};
use crate::{ydbg, YGEN_DEBUG};

use super::CfgAnalysis;

/// A loop info contains all the neccesary informations for the loop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopInfo {
    /// A block which has a edge to the loop
    preheader_block: Option<BlockId>,
    /// The loop body
    body: Vec<BlockId>,
    /// The loop condition
    cond_block: Option<BlockId>,

    /// A block which is called after the loop finished
    exit_block: Option<BlockId>,
    /// A block which branches to the exit 
    exititing_block: Option<BlockId>,

    /// The exact preheader node
    preheader: Option<Box<dyn Ir>>,
    /// The exact condition node
    cond: Option<Box<dyn Ir>>,

    /// The exact node which branches to the exit block
    exit: Option<Box<dyn Ir>>,

    /// other loop in the body
    pub nested: Option<Box<LoopInfo>>,
}

impl LoopInfo {
    /// Creates a new loop info
    pub fn new() -> Self {
        Self {
            preheader_block: None,
            body: Vec::new(),
            cond_block: None,
            exit_block: None,
            preheader: None,
            cond: None,
            exit: None,
            nested: None,
            exititing_block: None,
        }
    }

    /// Returns if the loop body contains the block
    /// or the condition is the block
    pub fn contains(&self, block: &BlockId) -> bool {
        self.body.contains(&block) || matches!(&self.cond_block, Some(cond) if cond == block)
    }

    /// Returns the body of the loop
    pub fn body(&self) -> &Vec<BlockId> {
        &self.body
    }

    /// Returns the prheader block of the loop
    pub fn preheader_block(&self) -> &BlockId {
        self.preheader_block.as_ref().expect("expected preheader")
    }

    /// Returns the cond block of the loop
    pub fn condition_block(&self) -> &BlockId {
        self.cond_block.as_ref().expect("expected condition")
    }

    /// Returns the cond block of the loop
    pub fn condition(&self) -> &Box<dyn Ir> {
        self.cond.as_ref().expect("expected condition")
    }

    /// Returns the exit block
    pub fn exit_block(&self) -> &BlockId {
        self.exit_block.as_ref().expect("expected exit block")
    }

    /// Returns the exiting block
    pub fn exiting_block(&self) -> &BlockId {
        self.exititing_block.as_ref().expect("expected exiting block")
    }

    /// Returns the exit node
    pub fn exit(&self) -> &Box<dyn Ir> {
        self.exit.as_ref().expect("expected exit")
    }

    /// Returns if it has a nested loop inside
    pub fn has_sub_loop(&self) -> bool {
        self.nested.is_some()
    }
}

/// Loop analysis anlysis pass
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopAnalysis<'a> {
    loops: Vec<LoopInfo>,
    cfg: CfgAnalysis::CFGAnalysis,
    func: &'a Function,
}

impl<'a> LoopAnalysis<'a> {
    /// Returns a new loop analysis pass
    pub fn new(func: &'a Function) -> Self {
        Self {
            loops: Vec::new(),
            cfg: CfgAnalysis::CFGAnalysis::analyze(func),
            func: func,
        }
    }

    /// Analyzes for loops in the given function
    pub fn analyze(&mut self) {
        ydbg!("[LoopAnalysis] running loop analysis on function {}", self.func.name);
        // This function looks for all loops in the given function
        // for that we need to first analyze all the dominators which is already done
        // using the cfg analysis in the `new` function 

        // then we need to check for all control flow edges where:
        //   A -> B
        //   B -> A
        // If this condition is meet, we know that a is the condition and b is the
        // last block of the body and exit block

        for cond in &self.func.blocks {
            let cond_id = cond.id();

            for body_block in self.cfg.predeccessors(&cond_id) {
                if !self.cfg.dominates(&cond_id, &body_block) {
                    // A -> B (not met)
                    // B -> A (met)
                    continue;
                }

                // Now we know the loop condition and exit block

                let mut just_body = false;

                // we now check if the condition is actually the condition of another existing 
                // loop, if yes we simply add the block to the loop body

                for li in &mut self.loops {
                    if li.contains(&cond_id) {
                        li.body.push(body_block.to_owned());
                        just_body = true;
                        break;
                    }

                    if li.contains(&body_block) {
                        just_body = true;
                        break;
                    }
                }

                if just_body {
                    continue;
                }

                // now we know that this is a custom loop so we add it to the loop list
                let mut li = LoopInfo::new();
                li.cond_block = Some(cond_id.clone());
                li.preheader_block = Some(body_block.clone());

                self.loops.push(li);
            }
        }

        // now where we found the rough loops we need to look for nested ones
        // which is TODO

        // now we can check for the exit blocks
        self.analyze_exit_blocks();

        // we now need to compute the exact nodes for the loop condition, exit
        self.analyze_nodes();

        // now we need to log all found loops
        if unsafe { YGEN_DEBUG } {
            for li in &self.loops {
                ydbg!("[LoopAnalysis] found loop: {:#?}", li);
            }
        }

    }

    /// Analyzes the exact nodes of the loop 
    fn analyze_nodes(&mut self) {
        for li in &mut self.loops {
            // Check node for cond
            let cond =  self.func.get_block_for(li.condition_block()).expect("expected valid condition block");
            let branch = self.cfg.branch_in_block(cond).expect("expected correct condition ir node");
            li.cond = Some(branch.to_owned());

            // Check node for preheader
            let preheader =  self.func.get_block_for(li.preheader_block()).expect("expected valid preheader block");
            let branch = self.cfg.branch_in_block(preheader).expect("expected correct preheader ir node");
            li.preheader = Some(branch.to_owned());

            // Check node for exit
            let exit =  self.func.get_block_for(li.exiting_block()).expect("expected valid exit block");
            let branch = self.cfg.branch_in_block(exit).expect("expected correct exit ir node");
            li.exit = Some(branch.to_owned());
        }
    }

    /// Analyzes the exit blocks for the loops
    fn analyze_exit_blocks(&mut self) {
        // 1. analyze exit block
        for li in &mut self.loops {
            for block in &self.func.blocks {
                let block = block.id();

                let mut branches = false; // does the loop even branches to the block
                let header = true;

                // Let's check if the block is not in the loop

                // body
                if li.body.contains(&block) { continue; }

                // cond
                if li.condition_block() == &block { continue; }

                // We now check if the loop can even branch to the block
                for body in &li.body {
                    if self.cfg.successors_direct(&body).contains(&block) {
                        branches = true;
                    }
                }

                if self.cfg.successors_direct(&li.condition_block()).contains(&block) {
                    branches = true;
                }

                if branches && header {
                    li.exit_block = Some(block)
                }
            }
        }
        
        // 2. analyze exiting block
        for li in &mut self.loops {
            let exit = li.exit_block();

            let mut exiting = None;

            for block in &li.body {
                if self.cfg.successors_direct(block).contains(exit) {
                    exiting = Some(block.to_owned());
                }
            }

            if self.cfg.successors_direct(li.condition_block()).contains(exit) {
                exiting = Some(li.cond_block.clone().unwrap());
            }

            li.exititing_block = exiting;
        }
    }

    /// Returns all loops
    pub fn loops(&self) -> Vec<LoopInfo> {
        self.loops.to_owned()
    }
}