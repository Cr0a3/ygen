use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{debug::DebugLocation, prelude::Function, CodeGen::MachineInstr, Obj::Link, IR::Module};

use super::{Arch, CallConv, TargetBackendDescr, Triple};

/// The target registry: manages different targets
pub struct TargetRegistry {
    targets: HashMap<Arch, TargetBackendDescr>,

    pub(crate) epilogs: HashMap<String, bool>,

    triple: Triple,
}

impl TargetRegistry {
    /// Creates an new backend registry
    pub fn new(triple: Triple) -> Self {
        Self {
            targets: HashMap::new(),
            epilogs: HashMap::new(),
            triple: triple,
        }
    }

    /// Adds an new target architecture
    pub fn add(&mut self, arch: Arch, descr: TargetBackendDescr) {
        self.targets.insert(arch, descr);
    }

    /// Sets the calling convention to use for the specified architecture
    /// If it isn't found the function does noting
    pub fn setCallingConventionForTarget(&mut self, arch: Arch, call: CallConv) {
        if let Some(target) = self.targets.get_mut(&arch) {
            target.call = call;
        }
    }

    /// returns the `TargetBackendDescr` for the arch (also it adjusts it's calling convention ...)
    pub fn getBasedOnArch(&mut self, arch: Arch) -> Result<&mut TargetBackendDescr, Box<dyn Error>> {
        if let Some(descr) = self.targets.get_mut(&arch) {
            Ok(descr)
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(arch) 
            ))
        }
    }

    /// emits machine instrs for target <br>
    /// **note**: machine instrs are portable over all platforms <br>
    /// **warning**: Does not add a prolog
    pub fn buildMachineInstrsForTarget(&mut self, arch: Arch, func: &Function, module: &mut Module) -> Result<Vec<MachineInstr>, Box<dyn Error>> {
        let mut backend = self.targets.get(&arch).expect(&format!("unregistered target: {:?}", arch)).clone();

        let instrs = backend.build_instrs(&self.triple, func, module);

        Ok(instrs)
    }

    /// Builds the ir of the given triple into text assembly code <br>
    /// **warning**: Does not add a prolog
    pub fn buildAsmForTarget(&mut self, arch: Arch, func: &Function, module: &mut Module) -> Result<Vec<String>, Box<dyn Error>> {
        let backend = self.targets.get(&arch).expect(&format!("unregistered target: {:?}", arch)).clone();

        let instrs = self.buildMachineInstrsForTarget(arch, func, module)?;

        let instrs = backend.lower(instrs)?;

        let mut asm = Vec::new();

        for instr in instrs {
            asm.extend_from_slice(
                &instr.dump()?
            );
        }

        Ok(asm)
    }

    /// Builds the ir of the given triple into machine code <br>
    /// **warning**: Does not add a prolog
    pub fn buildMachineCodeForTarget(&mut self, arch: Arch, funct: &Function, module: &mut Module) -> Result<(Vec<u8>, Vec<Link>), Box<dyn Error>> {
        let mut backend = self.targets.get(&arch).expect(&format!("unregistered target: {:?}", arch)).clone();

        let instrs = backend.build_instrs(&self.triple, funct, module);
        let instrs = backend.lower(instrs)?;

        let mut encoded = Vec::new();
        let mut links = Vec::new();

        for instr in instrs {
            let (encod, link) = instr.encode()?;
            
            encoded.extend_from_slice(&encod);

            if let Some(link) = &link {
                links.push(link.to_owned());
            }
        }

        Ok((encoded, links))
    }

    /// build the debugging information for an function
    pub fn buildDebugInfo(&mut self, arch: Arch, funct: &Function, module: &mut Module) -> Result<Vec<DebugLocation>, Box<dyn Error>> {
        let mut backend = self.targets.get(&arch).expect(&format!("unregistered target: {:?}", arch)).clone();

        let instrs = backend.build_instrs_with_ir_debug(&self.triple, funct, module);
        let instrs = backend.lower_debug(instrs)?;

        let mut dbgs = vec![];

        let mut offset = 0;

        for (instrs, location) in &instrs {
            dbgs.push(
                DebugLocation {
                    line: location.line,
                    col: location.line,
                    epilog: false,
                    prolog: false,
                    adr: offset,
                }
            );

            for instr in instrs {
                offset += instr.encode()?.0.len() as u64;
            }    
        }

        Ok(dbgs)
    } 
}

/// Stores errors which can occure in the `getBasedOnTriple` function in the `TargetRegistry`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryError {
    /// An unsupported architecture
    UnsuportedArch(Arch),
}

impl Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RegistryError::UnsuportedArch(arch) => format!("unsuported architecture: {:?}", arch),
        })
    }
}

impl Error for RegistryError {}