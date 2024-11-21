use std::collections::HashMap;

use crate::CodeGen::dag_builder::DagBuilder;
use crate::CodeGen::dag_lower::DagLower;
use crate::CodeGen::regalloc_iterated_col::ItRegCoalAllocBase;

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
    /// The register allocator
    pub allocator: ItRegCoalAllocBase,
}

/// The target registry is the "main hub" for compiling functions ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRegistry {
    triple: super::Triple,

    dag_lower_backends: HashMap<super::Arch, DagLower>,
    mc_compile_backends: HashMap<super::Arch, McCompile>,
    asm_printers: HashMap<super::Arch, AsmPrinter>,
    asm_parser: HashMap<super::Arch, AsmParser>,
    allocators: HashMap<super::Arch, ItRegCoalAllocBase>,
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
            allocators: HashMap::new(),
        }
    }

    /// Inserts an new backend for the given architecture into the registry
    pub fn insert(&mut self, arch: super::Arch, backend: BackendInfos) {
        self.dag_lower_backends.insert(arch, backend.dag);
        self.mc_compile_backends.insert(arch, backend.mc);
        self.asm_printers.insert(arch, backend.asm_printer);
        self.asm_parser.insert(arch, backend.parser);
        self.allocators.insert(arch, backend.allocator);
    }

    /// Sets the current target triple (used for selection of the backend to use)
    pub fn make_current(&mut self, triple: &super::Triple) {
        self.triple = *triple;
    }

    /// compiles the given function
    pub fn compile_fn(&self, func: &crate::IR::Function) -> (Vec<u8>, Vec<crate::Obj::Link>) {
        let mut dag = DagBuilder::build(&self.triple.arch, func);

        // let dag = DagOptimizer::optimize(dag);
        
        let Some(lower) = self.dag_lower_backends.get(&self.triple.arch) else { 
            panic!("unregistered dag lowering backend for {}", self.triple.arch)
        };

        // Run reg alloc

        let Some(alloc) = self.allocators.get(&self.triple.arch) else {
            panic!("unregistered register allocator for {}", self.triple.arch)
        };

        let mut alloc = alloc.fork(&func);

        let mc = lower.lower(&mut dag, &mut alloc);

        for (name, instrs) in &mc {
            println!(".{}", name.name);
            for instr in instrs {
                println!("  {}", instr.asm());
            }
        }

        // now we do block linking

        let mut positions = HashMap::new();

        let mut machine_code = HashMap::new();
        let mut machine_code_len = 0;

        let mut links = Vec::new();
        let mut block_links = Vec::new();

        for (block, instrs) in mc {
            positions.insert(block.to_owned(), machine_code_len);

            let mut encoded = Vec::new();
            for instr in instrs {
                encoded.extend_from_slice(&instr.encode());

                if let Some(mut reloc) = instr.relocation() {
                    reloc.at += encoded.len();
                    
                    links.push(reloc);
                }

                if let Some(mut reloc) = instr.branch_to_block() {
                    reloc.at += encoded.len();
                    block_links.push((reloc, block.to_owned()));
                }
            }

            machine_code_len += encoded.len();
            machine_code.insert(block.to_owned(), encoded);
        }

        for (reloc, start) in block_links {
            let bytes = machine_code.get_mut(&start).unwrap();

            let to = *positions.get(&crate::IR::BlockId{ name: reloc.to }).unwrap() as i32;
            let from = *positions.get(&start).unwrap() as i32;
            
            let mut target = to - from;
            target += reloc.at as i32;
            target += reloc.addend as i32;
            target -= 1;

            let target = target.to_be_bytes();

            let pos = reloc.at as i32 + reloc.addend as i32;

            for idx in 0..4 {
                bytes[(pos + idx) as usize] = target[(4 - idx) as usize];
            }
        }

        todo!();
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