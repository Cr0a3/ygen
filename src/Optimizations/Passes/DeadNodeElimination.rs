use crate::{prelude::{Call, Phi}, Optimizations::Pass, IR::{FuncId, Var}};

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
        for _ in 0..2 { // iterate two times, cuz then we can remove dependants with a dept of 1
            let mut used: Vec<String> = Vec::new();

            let mut to_remove = Vec::new();

            // first iterate over all phis

            for block in func.blocks.iter() {
                for node in &block.nodes {
                    if let Some(phi) = node.as_any().downcast_ref::<Phi>() {
                        for (_, reciver) in &phi.recive_from_blocks {
                            used.push(reciver.name.to_owned());
                        }
                    }
                }
            }

            // now we can iterate over all normal nodes

            for block in func.blocks.iter().rev() {
                let iter = block.nodes.iter();
                let iter = iter.rev();

                let mut index = iter.len();

                for node in iter {
                    let inputs =  node.inputs();
                    let out = node.output();
        
                    for input in inputs {
                        if !used.contains(&input.name) {
                            used.push(input.name);
                        }
                    }

                    if let Some(out) = out {
                        if !used.contains(&out.name) {
                            if let Some(_) = node.as_any().downcast_ref::<Call<FuncId, Vec<Var>, Var>>() {} else {
                                // node isn't a call
                                to_remove.push((block.name.clone(), index - 1));
                            }
                        }
                    }

                    index -= 1;
                }
            }

            for block in &mut func.blocks {
                let off = 0;

                for (target_block, node) in &to_remove {
                    if target_block == &block.name {
                        block.nodes.remove((*node - off) as usize);

                        //off += 1;
                    }
                }
            }
        }  
    }
}