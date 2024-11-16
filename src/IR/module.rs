use gimli::DwLang;

use crate::debug::DebugRegistry;
use crate::init_ygen;
use crate::Obj::{Decl, Linkage, ObjectBuilder};
use crate::Optimizations::PassManager;
use crate::Support::{ColorClass, ColorProfile};
use crate::Target::{TargetRegistry, Triple};

use super::{func::FunctionType, Const, Function, VerifyError};
use std::{collections::HashMap, error::Error, fmt::Debug, fs::OpenOptions, io::Write, path::Path};

/// ## The Module
/// The main class for handeling functions
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) funcs: HashMap<String, Function>,
    pub(crate) consts: HashMap<String, Const>,
    pub(crate) dbg_registry: Option<DebugRegistry>,

    pub(crate) debug_passes: bool,
}

impl Module {
    /// Creates a new module
    pub fn new() -> Self {
        init_ygen();
        Self {
            funcs: HashMap::new(),
            consts: HashMap::new(),
            dbg_registry: None,
            debug_passes: false,
        }
    }

    /// Initializes debugging metadata
    pub fn init_dbg(&mut self, producer: String, lang: DwLang, infile: &Path) {
        self.dbg_registry = Some(DebugRegistry::new(producer, lang, infile));
    }

    /// Makes, that debugging information is outputed from the passes
    pub fn activate_pass_dbg(&mut self) {
        self.debug_passes = true;
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

            string += &format!("{} const {} = {}\n", match consta.linkage {
                Linkage::Extern => "import",
                Linkage::External => "extern",
                Linkage::Internal => "intern",
            }, consta.name,  bytes);
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

            string += &format!("{} {} {} = {}\n", 
                profile.markup(&match consta.linkage {
                    Linkage::Extern => "import",
                    Linkage::External => "extern",
                    Linkage::Internal => "intern",
                }, ColorClass::Instr), 
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
        for pass in &mngr.passes {
            if self.debug_passes {
                eprintln!("Running pass: {}", pass.name());
            }

            for (_, func) in &mut self.funcs {
                pass.run_func(func);
    
                for block in &mut func.blocks {
                    pass.run(block);
                }
            }
        }
    }

    /// emits the machine code of the module into an object file (in the form of an object builder)
    pub fn emitMachineCode(&mut self, triple: Triple, registry: &mut TargetRegistry, debug: bool) -> Result<(ObjectBuilder, Option<DebugRegistry>), Box<dyn Error>> {
        let mut obj = ObjectBuilder::new(triple);

        for (name, func) in self.funcs.clone() {
            obj.decl( (&name, Decl::Function, func.linkage));

            let (comp, links) = if debug {
                if let Some(dbg) = self.dbg_registry.as_mut() {
                    registry.compile_dbg_fn(&func, dbg)
                } else { panic!("debugging needs to be initialized in order to use debugging information")}
            } else {
                registry.compile_fn(&func)
            };

            for link in links {
                obj.link(link);
            }

            obj.define(&name, comp);
        }

        // NOT CHANGE THE ORDER CUZ FOR SOME ARCHS (LIKE X86) FPs ARE MADE USING CONSTS
        // WHICH WOULD LEED TO A PANIC
        for (_, consta) in &self.consts {
            obj.decl((consta.name.as_str(), Decl::Constant, consta.linkage));
            obj.define(&consta.name, consta.data.clone());
        }

        Ok((obj, self.dbg_registry.to_owned()))
    }

    /// emits all function into one asm file
    pub fn emitToAsmFile(&mut self, triple: Triple, registry: &mut TargetRegistry, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).write(true)
                                .open(path)?;

        let lines = self.emitAsm(triple, registry)?;

        file.write_all(lines.as_bytes())?;

        Ok(())
    }

    /// emits all function into one asm string
    pub fn emitAsm(&mut self, triple: Triple, registry: &mut TargetRegistry) -> Result<String, Box<dyn Error>> {
        registry.make_current(&triple);
        let lines = registry.print_asm(self);

        Ok(lines)
    }
}

/// Creates a new module
pub fn Module() -> Module {
    Module::new()
}
