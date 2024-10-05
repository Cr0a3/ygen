mod func;
mod link;
mod map;

use std::fmt::Display;
use std::error::Error;

pub use map::*;
pub use func::*;
pub use link::*;

use crate::Obj::{Decl, Linkage};
use crate::Target::{TargetRegistry, Triple};
use crate::IR::Module;

impl Module {
    /// Builds all functions into an jit map which can be called
    pub fn jitMap(&mut self, backend: &mut TargetRegistry) -> Result<JitMap, Box<dyn Error>> {
        let mut map = JitMap::new();

        let (obj, _) = self.emitMachineCode(Triple::host(), backend, false)?;

        for (name, data) in &obj.defines {
            let mut typ = Decl::Function;
            for (decl_name, decl, linkage) in &obj.decls {
                if decl_name == name {
                    typ = *decl;
                }

                if *linkage == Linkage::Extern {
                    Err(JitError::UnsupportedImports)?
                }
            }

            match typ {
                Decl::Function => map.define_func(name, data.to_owned()),
                Decl::Data => map.define_data(name, data.to_owned()),
                Decl::Constant => map.define_data(name, data.to_owned()),
            }
        }

        Ok(map)
    }
}

/// Errors which can occure durring jit construction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JitError {
    /// The target triple needs to be the one of the host but was not
    TripleDoesntMatchHostOnes,
    /// Found an import but jit doesn't support function imports
    UnsupportedImports,
}

impl Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            JitError::TripleDoesntMatchHostOnes => "for constructing a jit map, the triple needs to be the host ones",
            JitError::UnsupportedImports => "jit maps don't support extern symbol imports",
        })
    }
}

impl Error for JitError {}