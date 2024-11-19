use crate::Optimizations::Analysis::LivenessAnalysis;
use crate::Optimizations::Pass;

/// ## Pass DeadNodeElimination <br>
/// deletes unused nodes
pub(crate) struct DeadNodeElimination_;

/// Creates a new DeadNodeElimination pass which is heap allocated
pub fn DeadNodeElimination() -> Box<dyn Pass> {
    Box::from( DeadNodeElimination_ {} )
}

impl Pass for DeadNodeElimination_ {
    fn name(&self) -> &'static str {
        "DeadNodeElimination"
    }
    
    fn run_func(&self, func: &mut crate::prelude::Function) {
        let liveness = LivenessAnalysis::analayze(func);

        // now we can iterate over all normal nodes

        let mut index = 0;

        for block in &mut func.blocks {
            for node in block.nodes.clone() {

                let mut removed = false;

                if let Some(output) = node.output() {
                    if liveness.is_dead(&output) && !node.is_call() {
                        block.nodes.remove(index);
                        removed = true;
                    }
                }

                if !removed {
                    index += 1;
                }
            }
        } 
    }
}