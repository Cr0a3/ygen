use crate::{prelude::Call, Optimizations::Pass, IR::{FuncId, Var}};

/// ## Pass DeadNodeElimination <br>
/// deletes unused nodes
pub(crate) struct DeadNodeElimination_;

/// Creates a new DeadNodeElimination pass which is heap allocated
pub fn DeadNodeElimination() -> Box<dyn Pass> {
    Box::from( DeadNodeElimination_ {} )
}

impl Pass for DeadNodeElimination_ {
    fn run_func(&self, func: &mut crate::prelude::Function) {
        let mut used: Vec<String> = Vec::new();

        let mut to_remove = Vec::new();

        for block in func.blocks.iter().rev() {
            let iter = block.nodes.iter();
            let iter = iter.rev();

            let mut index = iter.len() as i32;

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
            let mut off = 0;

            for (target_block, node) in &to_remove {
                if target_block == &block.name {
                    block.nodes.remove((*node - off) as usize);

                    off += 1;
                }
            }
        }
    }
}