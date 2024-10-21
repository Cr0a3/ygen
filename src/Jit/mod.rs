mod func;
mod link;
mod map;

use std::fmt::Display;
use std::error::Error;

pub use map::*;
pub use func::*;
pub use link::*;

use crate::Obj::Decl;
use crate::Target::{Arch, TargetRegistry, Triple};
use crate::IR::Module;

impl Triple {
    /// Returns if the triple supports jit
    pub fn supports_jit(&self) -> bool {
        match self.arch {
            Arch::X86_64 | Arch::X86 => true,
            _ => false, // is either not implemented or stuff like wasm which doesn't support jit
        }
    }
}

impl Module {
    /// Builds all functions into an jit map which can be called
    pub fn jitMap(&mut self, backend: &mut TargetRegistry) -> Result<JitMap, Box<dyn Error>> {

        if !Triple::host().supports_jit() {
            Err(JitError::HostDoesntSupportJit)?
        }

        let mut map = JitMap::new();

        let (obj, _) = self.emitMachineCode(Triple::host(), backend, false)?;

        for (name, data) in &obj.defines {
            let mut typ = Decl::Function;
            for (decl_name, decl, _) in &obj.decls {
                if decl_name == name {
                    typ = *decl;
                }
            }

            match typ {
                Decl::Function => map.define_func(name, data.to_owned()),
                Decl::Data => map.define_data(name, data.to_owned()),
                Decl::Constant => map.define_data(name, data.to_owned()),
            }
        }

        for reloc in &obj.links {
            map.reloc(reloc.to_owned());
        }

        map.deal_with_abs_symbols = match Triple::host().arch {
            Arch::X86_64 | Arch::X86 => Some(Box::new(crate::Target::x64::abs_jit::X64AbsSymDealer {})),
            _ => None,
        };

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
    /// The host target does not support jit
    HostDoesntSupportJit,
}

impl Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            JitError::TripleDoesntMatchHostOnes => "for constructing a jit map, the triple needs to be the host ones",
            JitError::UnsupportedImports => "jit maps don't support extern symbol imports",
            JitError::HostDoesntSupportJit => "the host doesn't support jit",
        })
    }
}

impl Error for JitError {}