use std::collections::HashMap;
use crate::CodeGen::dag_builder::DagBuilder;
use crate::CodeGen::dag_lower::DagLower;

use super::asm_printer::AsmPrinter;
use super::compile::McCompile;
use super::instr::McInstr;
use super::parser::AsmParser;
use super::Triple;


/// Compilation steps:
/// 1. Build dag (`crate::CodeGen::DabBuilder::build(func)`)
/// 2. Optimize dag (`crate::CodeGen::DabOptimizer::optimize(dag)`)
/// 3. Lower the dag (is target specific) using the `DagLower` struct
/// 4. Either compile the generated assembly to machine code (using `McCompile`) or print
///    its assembly out (using `AsmPrinter`)
pub struct BackendStepDocs;

/// All required structures for a "compile-complete" backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendInfos {
    /// The thing that turns the dag into assembly
    pub dag: DagLower,
    /// The thing that turns the assembly into machine code
    pub mc: McCompile,
    /// The thing that can print the assembly
    pub asm_printer: AsmPrinter,
    /// The thing that can parser target assembly
    pub parser: AsmParser,
}

/// The target registry is the "main hub" for compiling functions ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRegistry {
    triple: super::Triple,

    dag_lower_backends: HashMap<super::Arch, DagLower>,
    mc_compile_backends: HashMap<super::Arch, McCompile>,
    asm_printers: HashMap<super::Arch, AsmPrinter>,
    asm_parser: HashMap<super::Arch, AsmParser>,
}

impl TargetRegistry {
    /// Creates a new target registry
    pub fn new(triple: &Triple) -> Self {
        Self {
            triple: *triple,
            dag_lower_backends: HashMap::new(),
            mc_compile_backends: HashMap::new(),
            asm_printers: HashMap::new(),
            asm_parser: HashMap::new(),
        }
    }

    /// Inserts an new backend for the given architecture into the registry
    pub fn insert(&mut self, arch: super::Arch, backend: BackendInfos) {
        self.dag_lower_backends.insert(arch, backend.dag);
        self.mc_compile_backends.insert(arch, backend.mc);
        self.asm_printers.insert(arch, backend.asm_printer);
        self.asm_parser.insert(arch, backend.parser);
    }

    /// Sets the current target triple (used for selection of the backend to use)
    pub fn make_current(&mut self, triple: &super::Triple) {
        self.triple = *triple;
    }

    /// compiles the given function
    pub fn compile_fn(&self, func: &crate::IR::Function) -> (Vec<u8>, Vec<crate::Obj::Link>) {
        let dag = DagBuilder::build(&self.triple.arch, func);

        println!("dag: {:?}", dag);
        todo!()
    }

    /// compiles the given function with debug information
    pub fn compile_dbg_fn(&self, func: &crate::IR::Function, dbg: &mut crate::debug::DebugRegistry) -> (Vec<u8>, Vec<crate::Obj::Link>) {
        todo!()
    }

    /// Prints the assembly code of the module to a string
    pub fn print_asm(&self, module: &crate::IR::Module) -> String {
        todo!()
    }

    /// Parses the input assembly for the target
    pub fn parse_asm(&self, asm: &str) -> Result<Box<dyn McInstr>, Box<dyn std::error::Error>> {
        if let Some(parser) = self.asm_parser.get(&self.triple.arch) {
            parser.parse(asm)
        } else {
            panic!("no registered asm parser for the given target")
        }
    
    }
}