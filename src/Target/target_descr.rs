use crate::prelude::{ir::*, Block, Var};
use crate::CodeGen::{compilation::CompilationHelper, MachineInstr};

use super::{CallConv, Compiler, instr::Instr, Lexer};

pub(crate) type CompileFunc<T> = fn(&T, &mut TargetBackendDescr) -> Vec<Instr>;

/// The TargetBackendDescr is used to store all the functions/information to compile ir nodes into assembly
#[allow(unused)]
pub struct TargetBackendDescr<'a> {
    pub(crate) init: Option<fn(CallConv)->TargetBackendDescr<'a>>,

    pub(crate) lexer: Option<Box<dyn Lexer>>,
    pub(crate) compile: Option<Box<dyn Compiler>>,

    pub(crate) helper: Option<CompilationHelper>,

    pub(crate) block: Option<&'a Block>,
    pub(crate) call: CallConv,

    pub(crate) sink: Vec<MachineInstr>,
}

macro_rules! compile_func {
    ($name:ident, $func:ident, $($node:tt)*) => { 
        impl<'a> TargetBackendDescr<'a> { 
            /// gets the callback for compiling the  ir node into asm
            #[allow(unused)]
            pub(crate) fn $name(&mut self, node: &$($node)*) {
                if let Some(helper) = &mut self.helper {
                    if let Some(block) = &self.block {
                        helper.$func(node, &mut self.sink, block);
                    } else {
                        todo!("no current block");
                    }
                } else {
                    todo!("no compilation helper was registered");
                }
            }
        }
    };
}

impl<'a> TargetBackendDescr<'a> {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            init: None,
            lexer: None,
            compile: None,
            block: None,
            call: CallConv::SystemV,
            helper: None,
            sink: vec![],
        }
    }
    /// Returns the lexer to use with the TargetBackendDescr
    pub fn lexer(&self) -> Box<dyn Lexer> {
        self.lexer.clone().unwrap()
    }

    /// Returns the compiler to use with the TargetBackendDescr
    pub fn compiler(&self) -> Box<dyn Compiler> {
        self.compile.clone().unwrap()
    }
}

compile_func!(compile_add_var_var, compile_add_var_var, Add<Var, Var, Var>);
