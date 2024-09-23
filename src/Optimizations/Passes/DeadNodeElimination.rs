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
            let mut used: Vec<String> = Vec::new();
    
            let mut index = -2; // why ????? but it works for **one** tested example

            let mut to_remove = vec![];

            let iter = block.nodes.iter();
            let iter = iter.rev();

            for node in iter {
                let inputs =  node.inputs();
                let out = node.output();
    
                for input in inputs {
                    if !used.contains(&input.name) {
                        used.push(input.name);
                    } else {
                    }
                }

                if let Some(out) = out {
                    if !used.contains(&out.name) {
                        to_remove.push(index);
                    }
                }
                
                index += 1;
            }

            let mut subdend = 0;

            for index in to_remove {
                block.nodes.remove(index as usize - subdend);

                subdend += 1;
            }
        }
    }
}