use crate::prelude::{Ir, Replace, Block, Function};
use crate::Optimizations::Pass;

/// The instruction combine pass is used to combine multiple instructions into one
pub(crate) struct InstrCombinePass;

/// The instruction combine pass is used to combine multiple instructions into one
pub fn InstrCombine() -> Box<dyn Pass> {
    Box::new( InstrCombinePass {})
}

impl Pass for InstrCombinePass {
    fn name(&self) -> &'static str {
        "InstrCombine"
    }

    fn run_func(&self, func: &mut crate::prelude::Function) {
        InstrCombinePass::opt_func(func);

        for block in &mut func.blocks {
            InstrCombinePass::opt_block(block);

            for node in &mut block.nodes {
                if let Some(opt) = InstrCombinePass::opt1(node) {
                    node.replace(opt);
                    continue;
                }
            }
        }

    }
}

macro_rules! opt1_impl {
    ($node:expr, $check:tt, $func:expr, $($ty:tt)*) => {
        if let Some(node) = $node.as_any().downcast_ref::<$($ty)*>() {
            if let Some(optimized) = $func(node) {
                return Some(optimized);
            }
        }
    };
}

impl InstrCombinePass {
    /// Optimizes a single instruction into a more performant one
    pub(crate) fn opt1(node: &Box<dyn Ir>) -> Option<Box<dyn Ir>> {
        use crate::IR::ir::*;

        opt1_impl!(node, is_select, InstrCombinePass::opt_select, Select<IROperand, IROperand>);

        None
    }

    /// Optimizes the block by combining instructions
    pub(crate) fn opt_block(_block: &mut Block) {
        // TODO
    }

    /// Optimizes the function by combining instructions
    pub(crate) fn opt_func(_func: &mut Function) {
        // TODO
    }
}