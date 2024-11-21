use crate::ydbg;
use crate::Optimizations::Analysis::{LeafFunctionAnalysis, LivenessAnalysis};
use crate::Optimizations::Pass;
use crate::IR::ir;

/// Removes unused calls
pub(crate) struct UnusedCallRemovementPass;

/// Removes calls to leaf functions whose result is not used
pub fn UnusedCallRemovement() -> Box<dyn Pass> {
    Box::new( UnusedCallRemovementPass {} )
}

impl Pass for UnusedCallRemovementPass {
    fn name(&self) -> &'static str {
        "UnusedCallRemovement"
    }

    fn run_mod(&self, module: &mut crate::prelude::Module) {
        let leafs = LeafFunctionAnalysis::analyze(&module);
        
        for (_, func) in &mut module.funcs {
            let livness = LivenessAnalysis::analayze(&func);

            for block in &mut func.blocks {
                let mut index = 0;

                let mut to_remove = Vec::new();

                for node in &block.nodes {
                    if node.is_call() {
                        let Some(output) = node.output() else {
                            panic!("Call nodes need to have a output")
                        };

                        let Some(call) = node.as_any().downcast_ref::<ir::Call>() else {
                            unreachable!("if a node says that it is call and isn't ...")
                        };

                        if livness.is_dead(&output) && leafs.is_leaf(&call.func) {
                            ydbg!("[UCR] removed call to leaf function in {}.{}", func.name, block.name);
                            to_remove.push(index);
                        } else { index += 1; }
                    }
                }

                for index in to_remove {
                    block.nodes.remove(index);
                }
            }
        }
    }
}