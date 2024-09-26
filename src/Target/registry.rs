use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{prelude::Function, CodeGen::MachineInstr, Obj::Link, IR::Block};

use super::{Arch, CallConv, TargetBackendDescr, Triple};

/// The target registry: manages different targets
pub struct TargetRegistry {
    targets: HashMap<Arch, TargetBackendDescr>,

    epilogs: HashMap<String, bool>,
    pub(crate) stacks: HashMap<String, i64>,

    triple: Triple,
}

impl TargetRegistry {
    /// Creates an new backend registry
    pub fn new(triple: Triple) -> Self {
        Self {
            targets: HashMap::new(),
            epilogs: HashMap::new(),
            stacks: HashMap::new(),
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
    pub fn buildMachineInstrsForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<MachineInstr>, Box<dyn Error>> {
        let triple = self.triple;

        let mut epilog = if let Some(ep) = self.epilogs.get(&funct.name) {
            *ep
        } else { false };

        let stack = self.stacks.get(&funct.name).cloned();

        let org = self.getBasedOnArch(arch)?;
        org.epilog = epilog;

        if let Some(stack_off) = stack {
            org.helper.as_mut().unwrap().stack_off = stack_off;
        };

        org.epilog = epilog;

        org.block = Some(block.clone());
        let instrs = org.build_instrs(&funct, &triple);

        epilog = org.epilog;

        let stack = org.helper.as_ref().unwrap().stack_off;

        org.reset();

        if epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert( funct.name.to_owned(), epilog );
            }
        }

        if let Some(stacks) = self.stacks.get_mut(&funct.name) {
            *stacks = stack;
        } else {
            self.stacks.insert(funct.name.to_owned(), stack);
        }

        Ok(instrs)
    }

    /// Builds the ir of the given triple into text assembly code <br>
    /// **warning**: Does not add a prolog
    pub fn buildAsmForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<String>, Box<dyn Error>> {
       let triple = self.triple;
       let mut epilog = if let Some(ep) = self.epilogs.get(&funct.name) {
            *ep
        } else { false };

        let stack = self.stacks.get(&funct.name).cloned();

        let org = self.getBasedOnArch(arch)?;
        org.epilog = epilog;

        if let Some(stack_off) = stack {
            org.helper.as_mut().unwrap().stack_off = stack_off;
        };

        org.block = Some(block.clone());

        let instrs = org.build_instrs_with_ir_debug(&funct, &triple);
        let instrs = org.lower_debug(instrs)?;

        let mut asm = vec![];

        for instr in instrs {
            asm.push(
                instr.to_string()
            )
        }

        let stack = org.helper.as_ref().unwrap().stack_off;

        epilog = org.epilog;
        org.reset();

        if epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert(funct.name.to_string(), epilog);
            }
        }

        if let Some(stacks) = self.stacks.get_mut(&funct.name) {
            *stacks = stack;
        } else {
            self.stacks.insert(funct.name.to_owned(), stack);
        }

        Ok(asm)
    }

    /// Builds the ir of the given triple into machine code <br>
    /// **warning**: Does not add a prolog
    pub fn buildMachineCodeForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<(Vec<u8>, Vec<Link>), Box<dyn Error>> {
        let triple = self.triple;
        let mut epilog = if let Some(ep) = self.epilogs.get(&funct.name) {
             *ep
         } else { false };

         let stack = self.stacks.get(&funct.name).cloned();
 
         let org = self.getBasedOnArch(arch)?;
         org.epilog = epilog;
 
         if let Some(stack_off) = stack {
            org.helper.as_mut().unwrap().stack_off = stack_off;
         };

        org.epilog = epilog;
        org.block = Some(block.clone());

        let instrs = org.build_instrs(&funct, &triple);
        let instrs = org.lower(instrs)?;

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

        let stack = org.helper.as_ref().unwrap().stack_off;

        epilog = org.epilog;

        org.reset();


        if epilog {
            if !self.epilogs.contains_key(&funct.name) {
                self.epilogs.insert(funct.name.to_string(), epilog);
            }
        }

        if let Some(stacks) = self.stacks.get_mut(&funct.name) {
            *stacks = stack;
        } else {
            self.stacks.insert(funct.name.to_owned(), stack);
        }

        Ok((res, links))
    }

    /// returns if the function needs to get an added prolog
    pub fn requires_prolog(&self, funct: &Function) -> bool {
        if self.epilogs.contains_key(&funct.name) {
            true
        } else { false }
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