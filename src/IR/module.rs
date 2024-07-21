use crate::{prelude::Triple, Obj::{Decl, Linkage, ObjectBuilder}, PassManager, Target::TargetRegistry};

use super::{func::FunctionType, Block, Function, VerifyError};
use std::{collections::HashMap, error::Error};

/// ## The Module
/// The main class for handeling functions
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) funcs: HashMap<String, Function>,
}

impl Module {
    /// Creates a new module
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    /// Adds a new function to the module
    pub fn add(&mut self, name: &str, ty: &FunctionType) -> &mut Function {
        self.funcs
            .insert(name.to_string(), Function::new(name.to_string(), ty.to_owned()));
        self.funcs.get_mut(name).unwrap()
    }

    #[allow(dead_code)]
    /// Returns a read only reference to the given function name
    /// ### Used for passes
    pub(crate) fn getFunc(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    #[allow(dead_code)]
    /// Returns a mutable reference to the given function name
    /// ### Used for passes
    pub(crate) fn getMutFunc(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.get_mut(name)
    }

    /// Emits the ir of the entire moudle into one string
    /// Maybe save to an file
    pub fn dump(&self) -> String {
        let mut string = String::new();

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.dump());
        }

        string
    }
    
    /// Emits the ir of the entire module into a colored string
    /// Maybe output to stdout
    pub fn dumpColored(&self) -> String {
        let mut string = String::new();

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.dumpColored());
        }

        string
    }

    /// Checks if every function is correct:
    ///  * Checks if the return type is the actual specified return type of the function
    ///  * Checks all ir nodes
    pub fn verify(&self) -> Result<(), VerifyError> {
        for (_, func) in &self.funcs {
            func.verify()?
        }

        Ok(())
    }

    /// Runs the pass manager over all functions
    pub fn runPassMngr(&mut self, mngr: PassManager::PassManager) {
        for (_, func) in &mut self.funcs {
            func.runPassMngr(&mngr)
        }
    }

    /// emits the machine code of the module into an object file (in the form of an object builder)
    pub fn emitMachineCode(&self, triple: Triple, registry: &mut TargetRegistry) -> Result<ObjectBuilder, Box<dyn Error>> {
        let mut obj = ObjectBuilder::new(triple);

        for (name, func) in &self.funcs {
            obj.decl( (&name, Decl::Function, Linkage::External));

            let mut comp = vec![];

            let mut positions: Vec<(Block, /*offset from 0*/ usize)> = vec![];

            for block in &func.blocks {
                let compiled = &registry.buildMachineCodeForTarget(triple, block, &func)?;

                positions.push((block.clone(), comp.len()));

                comp.extend_from_slice(&compiled);
            }

            obj.define(&name, comp);
        }

        Ok(obj)
    }
}

/// Creates a new module
pub fn Module() -> Module {
    Module::new()
}
