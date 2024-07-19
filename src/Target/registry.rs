use std::{collections::{HashMap, VecDeque}, error::Error, fmt::Display};

use crate::prelude::{Block, Function};

use super::{Arch, CallConv, TargetBackendDescr, Triple};

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
            Ok(
                org.buildAsm.unwrap()(
                    &block, &funct, 
                    &org.init.unwrap()(triple.getCallConv()?).call, // Unnessecary (and slow) but prevents
                    &mut org.init.unwrap()(triple.getCallConv()?))  // lifetime issues 
                )
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(triple.arch) 
            ))
        }
    }

    /// Builds the ir of the given triple into machine code
    pub fn buildMachineCodeForTarget(&mut self, triple: Triple, block: &Block, funct: &Function) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(org) = self.targets.get_mut(&triple.arch) {

            let call = (org.init.unwrap()(triple.getCallConv()?)).call;

            println!("{:?}", call);

            let asm: VecDeque<String> = org.buildAsm.unwrap()(
                &block, &funct, 
                &call,
                &mut org.init.unwrap()(triple.getCallConv()?)).into();

            println!("{:#?}", asm);

            let mut res = vec![];

            for instr in &asm {
                let lexed = org.lexer().lex(instr.clone())?;
            
                let mut comp = org.compiler().new(lexed);
                comp.parse()?;

                res.extend_from_slice(&comp.out());
            }

            Ok(res)
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