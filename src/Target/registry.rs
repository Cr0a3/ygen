use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{debug::DebugLocation, prelude::Function, CodeGen::MachineInstr, Obj::Link, IR::Block};

use super::{Arch, CallConv, TargetBackendDescr, Triple};

/// The target registry: manages different targets
pub struct TargetRegistry {
    targets: HashMap<Arch, TargetBackendDescr>,
    funcs: HashMap<String, TargetBackendDescr>,

    pub(crate) epilogs: HashMap<String, bool>,

    triple: Triple,
}

impl TargetRegistry {
    /// Creates an new backend registry
    pub fn new(triple: Triple) -> Self {
        Self {
            targets: HashMap::new(),
            funcs: HashMap::new(),
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

    pub(crate) fn getBackendForFuncOrFork(&mut self, arch: Arch, funct: &Function) -> TargetBackendDescr {
        if let Some(backend) = self.funcs.get(&funct.name) {
            let backend = backend.to_owned();

            return backend;
        } else if let Some(to_fork) = self.targets.get(&arch) {
            let to_fork = to_fork.to_owned();
            self.funcs.insert(funct.name.to_owned(), to_fork);
            self.getBackendForFuncOrFork(arch, funct)
        } else { panic!("the arch: {:?} wasn't initialized", arch); }
    }

    fn updateFuncBackend(&mut self, name: &String, backend: TargetBackendDescr) {
        if let Some(original) = self.funcs.get_mut(name) {
            *original = backend;
        }
    }

    /// emits machine instrs for target <br>
    /// **note**: machine instrs are portable over all platforms <br>
    /// **warning**: Does not add a prolog
    pub fn buildMachineInstrsForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<MachineInstr>, Box<dyn Error>> {
        let triple = self.triple;

        let run_alloc = if let Some(_) = self.funcs.get(&funct.name) { false } else { true };
        
        let mut backend = self.getBackendForFuncOrFork(arch, funct);

        if run_alloc {
            if let Some(helper) = &mut backend.helper {
                helper.run_alloc(&funct);
            }
        }

        backend.block = Some(block.clone());
        let instrs = backend.build_instrs(&triple);

        if backend.epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert( funct.name.to_owned(), true );
            }
        }

        self.updateFuncBackend(&funct.name, backend);

        Ok(instrs)
    }

    /// Builds the ir of the given triple into text assembly code <br>
    /// **warning**: Does not add a prolog
    pub fn buildAsmForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<String>, Box<dyn Error>> {
       let triple = self.triple;

       let run_alloc = if let Some(_) = self.funcs.get(&funct.name) { false } else { true };

        let mut backend = self.getBackendForFuncOrFork(arch, funct);

        if run_alloc {
            if let Some(helper) = &mut backend.helper {
                helper.run_alloc(&funct);
            }
        }

        backend.block = Some(block.clone());

        let instrs = backend.build_instrs_with_ir_debug(&triple);
        let instrs = backend.lower_debug(instrs)?;

        let mut asm = vec![];

        for (instrs, _) in instrs {
            for instr in instrs {
                asm.push(
                    instr.to_string()
                )
            }
        }

        if backend.epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert(funct.name.to_string(), true);
            }
        }

        self.updateFuncBackend(&funct.name, backend);

        Ok(asm)
    }

    /// Builds the ir of the given triple into machine code <br>
    /// **warning**: Does not add a prolog
    pub fn buildMachineCodeForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<(Vec<u8>, Vec<Link>), Box<dyn Error>> {
        let triple = self.triple;

        let run_alloc = if let Some(_) = self.funcs.get(&funct.name) { false } else { true };

        let mut backend = self.getBackendForFuncOrFork(arch, funct);

        if run_alloc {
            if let Some(helper) = &mut backend.helper {
                helper.run_alloc(&funct);
            }
        }

        backend.block = Some(block.clone());

        let instrs = backend.build_instrs(&triple);
        let instrs = backend.lower(instrs)?;

        let mut res = vec![];
        let mut links = vec![];

        for instr in &instrs {
            let (encoded, link) = &instr.encode()?;
            res.extend_from_slice(&encoded);

            if let Some(link) = link {
                let mut link = link.clone();

                if link.special {
                    link.from = block.name.to_owned();
                } else {
                    link.from = funct.name.to_string();
                }
                link.at = res.len();

                links.push(link);
            }
        }

        if backend.epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert(funct.name.to_string(), true);
            }
        }

        self.updateFuncBackend(&funct.name, backend);

        Ok((res, links))
    }

    /// returns if the function needs to get an added prolog
    pub fn requires_prolog(&self, funct: &Function) -> bool {
        if self.epilogs.contains_key(&funct.name) {
            true
        } else { false }
    }

    /// build the debugging information for an function
    pub fn buildDebugInfo(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<DebugLocation>, Box<dyn Error>> {
        let triple = self.triple;

        let run_alloc = if let Some(_) = self.funcs.get(&funct.name) { false } else { true };

        let mut backend = self.getBackendForFuncOrFork(arch, funct);

        if run_alloc {
            if let Some(helper) = &mut backend.helper {
                helper.run_alloc(&funct);
            }
        }

        backend.block = Some(block.clone());

        let instrs = backend.build_instrs_with_ir_debug(&triple);
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

        if backend.epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert(funct.name.to_string(), true);
            }
        }

        self.updateFuncBackend(&funct.name, backend);

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