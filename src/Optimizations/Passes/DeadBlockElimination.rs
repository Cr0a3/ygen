use crate::ydbg;
use crate::Optimizations::Pass;
use crate::IR::{ir::Br, ir::BrCond, ir::Switch};

/// ## Pass DeadBlockElimination <br>
/// deletes unused blocks
pub(crate) struct DeadBlockElimination {
}

/// Creates a new DeadBlockElimination pass which is heap allocated
pub fn DeadBlockElimination() -> Box<dyn Pass> {
    Box::from( DeadBlockElimination {} )
}

impl Pass for DeadBlockElimination {
    fn name(&self) -> &'static str {
        "DeadBlockElimination"
    }
    
    fn run_func(&self, func: &mut crate::prelude::Function) {
        ydbg!("[DBE] running dead block elimination");
        let mut used_blocks = Vec::new();

        // CHECK FOR ALL USED BLOCKS

        for block in &func.blocks {
            for node in &block.nodes {
                if let Some(br) = node.as_any().downcast_ref::<Br>() {
                    used_blocks.push(br.inner1.name.to_owned());
                }

                if let Some(br) = node.as_any().downcast_ref::<BrCond>() {
                    used_blocks.push(br.inner2.name.to_owned());
                    used_blocks.push(br.inner3.name.to_owned());
                }

                if let Some(switch) = node.as_any().downcast_ref::<Switch>() {
                    for (_, case) in &switch.cases {
                        used_blocks.push(case.name.to_owned());
                    }
                }
            }
        }

        // REMOVE UNUSED BLOCKS
        
        let mut index = 0;

        for block in func.blocks.clone() {
            if !used_blocks.contains(&block.name) && index != 0 { // do not remove first block
                func.blocks.remove(index);
            } else { // no removed block
                index += 1;
            }
        }
    }
}