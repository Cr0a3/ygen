use crate::{prelude::Triple, Obj::{Decl, Linkage, ObjectBuilder}, Optimizations::PassManager, Support::{ColorClass, ColorProfile}, Target::TargetRegistry};

use super::{func::FunctionType, Const, Function, VerifyError};
use std::{collections::HashMap, error::Error, fs::OpenOptions, io::Write, path::Path};

/// ## The Module
/// The main class for handeling functions
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) funcs: HashMap<String, Function>,
    pub(crate) consts: HashMap<String, Const>,
}

impl Module {
    /// Creates a new module
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            consts: HashMap::new(),
        }
    }

    /// Adds a new function to the module
    pub fn add(&mut self, name: &str, ty: &FunctionType) -> &mut Function {
        self.funcs
            .insert(name.to_string(), Function::new(name.to_string(), ty.to_owned()));
        self.funcs.get_mut(name).unwrap()
    }

    /// Adds an already defined function to the module
    pub fn add_raw(&mut self, func: Function) {
        self.funcs.insert(func.name.to_string(), func);
    }

    /// Adds a new constant to the module
    pub fn addConst(&mut self, name: &str) -> &mut Const {
        self.consts
            .insert(name.to_string(), Const::new(name.to_string()));
        self.consts.get_mut(name).unwrap()
    }

    /// Adds an already defined const to the module
    pub fn add_raw_const(&mut self, constant: Const) {
        self.consts.insert(constant.name.to_string(), constant);
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

        for (_, consta) in &self.consts {
            let mut bytes = String::from("[ ");

            for byte in &consta.data { 
                bytes.push_str(&format!("{}, ", *byte));
            }

            if consta.data.len() > 0 {
                bytes.remove(bytes.chars().count() - 1);
                bytes.remove(bytes.chars().count() - 2);
            }

            bytes.push(']');

            string += &format!("const {} = {}\n", consta.name,  bytes);
        }

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.dump());
        }

        string
    }
    
    /// Emits the ir of the entire module into a colored string
    /// Maybe output to stdout
    pub fn dumpColored(&self, profile: ColorProfile) -> String {
        let mut string = String::new();

        for (_, consta) in &self.consts {
            let mut bytes = String::from("[ ");

            for byte in &consta.data { 
                bytes.push_str(&format!("{}, ", *byte));
            }

            if consta.data.len() > 0 {
                bytes.remove(bytes.chars().count() - 1);
                bytes.remove(bytes.chars().count() - 2);
            }

            bytes.push(']');

            string += &format!("{} {} = {}\n", 
                profile.markup("const", ColorClass::Instr), 
                profile.markup(&consta.name, ColorClass::Name), 
                profile.markup(&bytes, ColorClass::Value)
            );
        }

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.dumpColored(profile));
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
    pub fn runPassMngr(&mut self, mngr: PassManager) {
        for (_, func) in &mut self.funcs {
            func.runPassMngr(&mngr)
        }
    }

    /// emits the machine code of the module into an object file (in the form of an object builder)
    pub fn emitMachineCode(&self, triple: Triple, registry: &mut TargetRegistry) -> Result<ObjectBuilder, Box<dyn Error>> {
        let mut obj = ObjectBuilder::new(triple);

        for (_, consta) in &self.consts {
            obj.decl((consta.name.as_str(), Decl::Constant, consta.linkage));
            obj.define(&consta.name, consta.data.clone());
        }

        for (name, func) in &self.funcs {
            obj.decl( (&name, Decl::Function, func.linkage));

            let mut comp = vec![];

            for block in &func.blocks {
                let (compiled, links) = &registry.buildMachineCodeForTarget(triple, block, &func)?;

                comp.extend_from_slice(&compiled);

                for link in links {
                    obj.link(link.to_owned())
                }
            }

            obj.define(&name, comp);
        }

        Ok(obj)
    }

    /// emits all function into one asm file
    pub fn emitToAsmFile(&self, triple: Triple, registry: &mut TargetRegistry, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).write(true)
                                .open(path)?;

        let lines = self.emitAsm(triple, registry)?;

        file.write_all(lines.as_bytes())?;

        Ok(())
    }

    /// emits all function into one asm file
    pub fn emitAsm(&self, triple: Triple, registry: &mut TargetRegistry) -> Result<String, Box<dyn Error>> {
        let mut lines = String::new();
        lines.push_str(&format!("// made using {} v{}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        lines.push_str(&format!("// by {}\n\n", env!("CARGO_PKG_AUTHORS").replace(";", " ")));
        lines.push_str("section .rodata\n\n");

        for (_, consta) in &self.consts {
            lines.push_str(&format!("{}: {:?}\n", consta.name, consta.data));
        }
        lines.push_str("section .text\n\n");

        for (name, func) in &self.funcs {
            if func.linkage == Linkage::Extern {
                lines += &format!("global {}\n", name);
                continue;
            }

            if func.linkage == Linkage::External {
                lines += &format!("global {}\n", name);
            } 

            lines += &format!("{}:\n", name);

            for block in &func.blocks {
                if block.name.to_lowercase() != "entry" {
                    lines += &format!("  {}:\n", block.name)
                }

                let asm_lines = registry.buildAsmForTarget(triple, block, func)?;

                for line in asm_lines {
                    lines += &format!("\t{}\n", line);
                }
            }
        }

        Ok(lines)
    }
}

/// Creates a new module
pub fn Module() -> Module {
    Module::new()
}
