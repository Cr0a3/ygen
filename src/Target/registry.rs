use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{prelude::{Block, Function}, Obj::Link};

use super::{instr::Mnemonic, Arch, CallConv, TargetBackendDescr, Triple};

/// The target registry: manages different targets
pub struct TargetRegistry<'a> {
    targets: HashMap<Arch, TargetBackendDescr<'a>>
}

impl<'a> TargetRegistry<'a> {
    /// Creates an new backend registry
    pub fn new() -> Self {
        Self {
            targets: HashMap::new()
        }
    }

    /// Adds an new target architecture
    pub fn add(&mut self, arch: Arch, descr: TargetBackendDescr<'a>) {
        self.targets.insert(arch, descr);
    }

    /// Sets the calling convention to use for the specified architecture
    /// If it isn't found the function does noting
    pub fn setCallingConventionForTarget(&mut self, arch: Arch, call: CallConv) {
        if let Some(target) = self.targets.get_mut(&arch) {
            target.call = call;
        }
    }

    /// returns the `TargetBackendDescr` for the triple (also it adjusts it's calling convention ...)
    pub fn getBasedOnTriple(&mut self, triple: Triple) -> Result<&mut TargetBackendDescr<'a>, Box<dyn Error>> {
        if let Some(descr) = self.targets.get_mut(&triple.arch) {
            *descr = descr.init.unwrap()(triple.getCallConv()?);

            Ok(descr)
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(triple.arch) 
            ))
        }
    }

    /// Builds the ir of the given triple into text assembly code
    pub fn buildAsmForTarget(&mut self, triple: Triple, block: &Block, funct: &Function) -> Result<Vec<String>, Box<dyn Error>> {
        if let Some(org) = self.targets.get_mut(&triple.arch) {
            let instrs = org.build_instrs(&block, &funct, &triple);
            let instrs = org.lower(instrs);

            let mut asm = vec![];

            for instr in instrs {
                asm.push(
                    instr.to_string()
                )
            }

            Ok(asm)
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(triple.arch) 
            ))
        }
    }

    /// Builds the ir of the given triple into machine code
    pub fn buildMachineCodeForTarget(&mut self, triple: Triple, block: &Block, funct: &Function) -> Result<(Vec<u8>, Vec<Link>), Box<dyn Error>> {
        if let Some(org) = self.targets.get_mut(&triple.arch) {

            let call = (org.init.unwrap()(triple.getCallConv()?)).call;

            let instrs = org.build_instrs(&block, &funct, &triple);
            let instrs = org.lower(instrs);

            let mut res = vec![];
            let mut links = vec![];

            for instr in &instrs {
                if instr.mnemonic == Mnemonic::Debug {
                    continue;
                };

                let (encoded, link) = &instr.encode()?;
                res.extend_from_slice(&encoded);

                if let Some(link) = link {
                    let mut link = link.clone();

                    link.from = funct.name.to_string();
                    link.at = res.len();

                    links.push(link);
                }
            }

            Ok((res, links))
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(triple.arch) 
            ))
        }
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