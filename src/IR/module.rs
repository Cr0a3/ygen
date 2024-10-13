use gimli::DwLang;

use crate::{debug::{DebugLocation, DebugRegistry}, prelude::Triple, CodeGen::MachineInstr, Obj::{Decl, Link, Linkage, ObjectBuilder}, Optimizations::PassManager, Support::{ColorClass, ColorProfile}, Target::TargetRegistry};

use super::{func::FunctionType, Const, Function, VerifyError};
use std::{collections::HashMap, error::Error, fmt::Debug, fs::OpenOptions, io::Write, path::Path};

/// ## The Module
/// The main class for handeling functions
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) funcs: HashMap<String, Function>,
    pub(crate) consts: HashMap<String, Const>,
    pub(crate) dbg_registry: Option<DebugRegistry>,
}

impl Module {
    /// Creates a new module
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            consts: HashMap::new(),
            dbg_registry: None,
        }
    }

    /// Initializes debugging metadata
    pub fn init_dbg(&mut self, producer: String, lang: DwLang, infile: &Path) {
        self.dbg_registry = Some(DebugRegistry::new(producer, lang, infile));
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
    pub fn emitMachineCode(&mut self, triple: Triple, registry: &mut TargetRegistry, debug: bool) -> Result<(ObjectBuilder, Option<DebugRegistry>), Box<dyn Error>> {
        let mut obj = ObjectBuilder::new(triple);

        for (name, func) in self.funcs.clone() {
            obj.decl( (&name, Decl::Function, func.linkage));

            let mut blocks: Vec<(String, (Vec<u8>, Vec<Link>))> = Vec::new();

            let mut index = 0;

            let mut comp = vec![];

            for block in &func.blocks {
                let (compiled, links) = registry.buildMachineCodeForTarget(triple.arch, &block, &func, self)?;
                
                if registry.requires_prolog(&func) && index == 0 {
                    let mut helper = registry.getBackendForFuncOrFork(triple.arch, &func).helper.expect("expected valid helper");
                    
                    let mut prolog = vec![];

                    helper.compile_prolog(&mut prolog);

                    let mc_instrs = helper.lower.unwrap()(triple.getCallConv()?, prolog);
                    for instr in mc_instrs {
                        comp.extend_from_slice(&instr.encode()?.0);
                    }

                    if debug {
                        if let Some(reg) = self.dbg_registry.as_mut() {
                            reg.add_location(&func.name, DebugLocation {
                                line: 0,
                                col: 0,
                                epilog: false,
                                prolog: true,
                                adr: 0,
                            });
                        } else {
                            panic!("you need to initialize debugging information for the registry in order to use debugging information")
                        }
                    }
                }

                if debug {
                    let mut debug_info = registry.buildDebugInfo(triple.arch, &block, &func, self)?;

                    for dbg in &mut debug_info {
                        dbg.adr += comp.len() as u64 + 1;

                        if let Some(reg) = self.dbg_registry.as_mut() {
                            reg.add_location(&func.name, *dbg);
                        } else {
                            panic!("you need to initialize debugging information for the registry in order to use debugging information")
                        }
                    }
                }

                blocks.push((block.name.to_owned(), (compiled, links)));

                index += 1;
            }

            let mut block_links: Vec<((i64, i64, i64, i64), String, String, i64)> = vec![];

            let mut block_adrs = HashMap::new();

            for (name, (data, links)) in blocks {
                let prev_len = comp.len();

                block_adrs.insert(name, prev_len);

                comp.extend_from_slice(&data);

                for link in links {
                    if link.special { // block to block link
                        let adr = |idx| {
                            link.at as i64 + (prev_len as i64) + link.addend + idx
                        };

                        block_links.push(((adr(0), adr(1), adr(2), adr(3)), link.to, link.from, link.at as i64 + link.addend - 1
                    ));
                    } else {
                        obj.link(Link { 
                            from: link.from, 
                            to: link.to, 
                            at: link.at + prev_len - 1, 
                            addend: link.addend,
                            special: false,
                        });
                    }
                }
            }

            for (idx, target, source, off) in block_links {
                let mut target_adr = *block_adrs.get(&target).expect("hmm i made a programming error") as i64;
                let source_adr = *block_adrs.get(&source).expect("hmm i made a programming error") as i64 + 5;

                target_adr -= source_adr + off;

                let bytes = target_adr.to_be_bytes();

                let mut set_byte = |idx: i64, to: u8| {
                    *comp.get_mut(idx as usize).unwrap() = to;
                };
                
                set_byte(idx.0, bytes[7]);
                set_byte(idx.1, bytes[6]);
                set_byte(idx.2, bytes[5]);
                set_byte(idx.3, bytes[4]);
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

    /// emits machine instrs for target
    /// note: machine instrs are portable over all platforms
    pub fn emitMachineInstrs(&mut self, triple: Triple, registry: &mut TargetRegistry) -> Result<Vec<(String, Vec<MachineInstr>)>, Box<dyn Error>> {
        let mut out = Vec::new();

        for (name, func) in self.funcs.clone() {
            let mut instrs = vec![];

            for block in &func.blocks {
                instrs.extend_from_slice(&
                    registry.buildMachineInstrsForTarget(triple.arch, &block,  &func, self)?
                );
            }

            if registry.requires_prolog(&func) {
                let backup = instrs.clone();
                
                let mut helper = registry.getBackendForFuncOrFork(triple.arch, &func).helper.expect("expected valid helper");

                let mut prolog = vec![];

                helper.compile_prolog(&mut prolog);

                instrs = prolog;
                instrs.extend_from_slice(&backup);
            }

            out.push((name.to_string(), instrs));
        }

        Ok(out)
    }

    /// emits all function into one asm string
    pub fn emitAsm(&mut self, triple: Triple, registry: &mut TargetRegistry) -> Result<String, Box<dyn Error>> {
        let mut lines = String::new();
        lines.push_str(&format!("// made using {} v{}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        lines.push_str(&format!("// by {}\n\n", env!("CARGO_PKG_AUTHORS").replace(";", " ")));
        lines.push_str("section .text\n\n");

        for (name, func) in self.funcs.clone() {
            if func.linkage == Linkage::Extern {
                lines += &format!("global {}\n", name);
                continue;
            }

            if func.linkage == Linkage::External {
                lines += &format!("global {}\n", name);
            } 

            lines += &format!("{}:\n", name);

            let mut index = 0;

            for block in &func.blocks {
                lines += &format!(" {}:\n", block.name);
                
                let asm_lines = registry.buildAsmForTarget(triple.arch, &block, &func, self)?;
                
                if registry.requires_prolog(&func) && index == 0 {
                    let mut helper = registry.getBackendForFuncOrFork(triple.arch, &func).helper.expect("expected valid helper");

                    let mut prolog = vec![];

                    helper.compile_prolog(&mut prolog);

                    let mc_instrs = helper.lower.unwrap()(triple.getCallConv()?, prolog);

                    for instr in mc_instrs {
                        for line in instr.dump()?  {
                            lines += &format!("\t{}\n", line);
                        }
                    }
                }

                for line in asm_lines {
                    if line.starts_with("#") { // debug
                        lines.pop(); // \n
                    }

                    lines += &format!("\t{}\n", line);
                }
            
                index += 1;
            }
        }

        lines.push_str("section .rodata\n\n");

        for (_, consta) in &self.consts {
            lines.push_str(&format!("{}: {:02X?} # {}\n", consta.name, consta.data, consta.data.iter()                                      
                                                                                                .filter_map(|&byte| {
                                                                                                    if byte >= 32 && byte <= 126 {
                                                                                                        Some(byte as char)
                                                                                                    } else {
                                                                                                        None
                                                                                                    }
                                                                                                }).collect::<String>()));
        }

        Ok(lines)
    }
}

/// Creates a new module
pub fn Module() -> Module {
    Module::new()
}
