use std::collections::HashMap;

use crate::Optimizations::Pass;

/// ## Pass DeadNodeElimination <br>
/// deletes unused nodes
pub(crate) struct DeadNodeElimination {
    recursion_steps: u32, // only used later one
}

/// Creates a new DeadNodeElimination pass which is heap allocated
pub fn DeadNodeElimination() -> Box<dyn Pass> {
    Box::from( DeadNodeElimination {
        recursion_steps: 4,
    } )
}

impl Pass for DeadNodeElimination {
    fn run(&self, block: &mut crate::prelude::Block) {
        for _ in 0..self.recursion_steps {
            let mut used: HashMap<String, /*position*/usize> = HashMap::new();
    
            let mut index = 0;
    
            let iter = block.nodes.to_owned();
            let iter = iter.iter().rev();

            for node in iter {
                index += 1;

                println!("{}", node.dump());
                let inputs =  node.inputs();
                let out = node.output();
    
                for input in inputs {
                    if !used.contains_key(&input.name) {
                        println!("used just got an addition {}", input.name);
                        used.insert(input.name, index);
                    } else {
                        println!("used already contains {}", input.name);
                    }
                }

                if let Some(out) = out {
                    if !used.contains_key(&out.name) {
                        index -= 1;
                        block.nodes.remove(index);
                    }
                }
            }
        }
    }
}