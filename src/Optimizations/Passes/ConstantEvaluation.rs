use std::collections::HashMap;

use crate::{prelude::*, Optimizations::Pass};

/// ## Pass ConstantEvaluation <br>
/// precomputes constant values
pub(crate) struct ConstantEvaluation {}

/// Creates a new ConstantEvaluation pass which is heap allocated
pub fn ConstantEvaluation() -> Box<dyn Pass> {
    Box::from( ConstantEvaluation {} )
}

impl Pass for ConstantEvaluation {
    fn name(&self) -> &'static str {
        "ConstantEvaluation"
    }

    fn run(&self, block: &mut crate::prelude::Block) {
        let mut const_values = HashMap::new();

        for node in block.nodes.iter_mut() {
            if let Some(inlined) = node.maybe_inline(&const_values) {
                node.replace( inlined )
            }

            if let Some(eval) = node.eval() {
                node.replace( eval );
            }

            if let Some(node) = node.as_any().downcast_ref::<Assign<Var, Type>>() {
                const_values.insert(node.inner1.name.to_owned(), node.inner2);

            }    
        }
    }
}