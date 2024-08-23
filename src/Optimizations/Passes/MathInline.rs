/*use crate::{prelude::*, Optimizations::Pass};
use std::collections::HashMap;

/// ## Pass Pre compute value <br>
/// Precomputes math expressions <br>
/// For example: <br>
///  * The expression 5 * 7 gets evalutated to 35
pub struct InlineConstValue {}

/// Creates a new InlineConstValue pass which is heap allocated
pub fn InlineConstValue() -> Box<InlineConstValue> {
    Box::from( InlineConstValue {} )
}

impl Pass for InlineConstValue {
    fn run(&self, block: &mut crate::prelude::Block) {
        let mut vars: HashMap<Var, (Type, /*write node*/&Box<dyn Ir>, /*already read*/bool)> = HashMap::new();

        for node in &block.nodes {
            block.isVarUsedAfterNode(node, node.inn)
        }
    }
}*/