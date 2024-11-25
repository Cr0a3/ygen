use crate::{ydbg, Obj::Linkage, IR::{ir, Const, FuncId, Module, Var}};

/// Analyses which functions are leaf functions
/// 
/// ### What's a leaf function?
/// 
/// A leaf function is a function which does not call non-leaf-functions
/// and does not modify global data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafFunctionAnalysis {
    leaf_funcs: Vec<FuncId>,
}

impl LeafFunctionAnalysis {
    /// Analyzes which functions are leaf functions
    pub fn analyze(module: &Module) -> Self {
        let mut leaf_funcs = Vec::new();

        for (_, func) in &module.funcs {
            // first check here:
            // is func a extern function

            if func.linkage != Linkage::Extern {
                leaf_funcs.push(func.id());
            }
        }

        LeafFunctionAnalysis::filter_call(&mut leaf_funcs, module);
        LeafFunctionAnalysis::filter_store(&mut leaf_funcs, module);

        for leaf in &leaf_funcs {
            ydbg!("[LeafFuncs] The function {} is a leaf function", leaf.name);
        }

        Self {
            leaf_funcs: leaf_funcs,
        }
    }

    pub(crate) fn filter_call(list: &mut Vec<FuncId>, module: &Module) {
        let mut index = 0;

        for (_, func) in &module.funcs {
            if !list.contains(&func.id()) { continue; }

            let mut condition_met = true;

            for block in &func.blocks {
                for node in &block.nodes {
                    if node.is_call() {
                        condition_met = false;
                    }
                }
            }

            if !condition_met {
                list.remove(index);
            } else { index += 1;}
        }
    }

    pub(crate) fn filter_store(list: &mut Vec<FuncId>, module: &Module) {
        let mut index = 0;

        for (_, func) in &module.funcs {
            if !list.contains(&func.id()) { continue; }

            let mut condition_met = true;

            let mut ptr_constants = Vec::new();

            // Check which variables are referencing a pointer 

            for block in &func.blocks {
                for node in &block.nodes {
                    if let Some(node) = node.as_any().downcast_ref::<ir::Assign<Var, Const>>() {
                        ptr_constants.push(node.inner1.to_owned());
                    }

                    for input in node.inputs() {
                        if ptr_constants.contains(&input) {
                            if let Some(output) = node.output() {
                                ptr_constants.push(output);
                            }
                        }
                    }
                }
            }

            for block in &func.blocks {
                for node in &block.nodes {
                    if node.is_store() {
                        for input in node.inputs() {
                            if ptr_constants.contains(&input) {
                                condition_met = false;
                            }
                        }
                    }
                }
            }

            if !condition_met {
                list.remove(index);
            } else { index += 1;}
        }
    }

    /// Returns if the given function is a leaf function
    pub fn is_leaf(&self, func: &FuncId) -> bool {
        self.leaf_funcs.contains(func)
    }
}