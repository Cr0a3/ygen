use crate::{prelude::*, PassManager::Pass};
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
        let mut vars: HashMap<Var, Type> = HashMap::new(); // no use of

        for node in block.nodes.iter_mut() {
            if &node.name() == "AddTypeType" {
                let any = node.as_any();
                let add_node = any.downcast_ref::<Add<Type, Type, Var>>().unwrap().clone();
                let var = add_node.inner3;
                vars.insert(var.name.clone(), var.clone());
            }

            if &node.name() == "AddVarVar" {
                let any = node.as_any();
                let add_node = any.downcast_ref::<Add<Var, Var, Var>>().unwrap().clone();
                let op0 = add_node.inner1;
                let op1 = add_node.inner2;
                let res = add_node.inner3;

                if vars.contains_key(&op0.name) && vars.contains_key(&op1.name) { // not the most performant order but it's ok 
                    node.replace(ConstAssign::new(res, ))
                } else if vars.contains_key(&op0.name) || vars.contains_key(&op1.name) {

                }
            }
        }
    }
}