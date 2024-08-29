use crate::prelude::{ir::*, Block, Var};
use crate::CodeGen::{compilation::CompilationHelper, MachineInstr};
use crate::IR::Type;

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
compile_func!(compile_and_var_var, compile_and_var_var, And<Var, Var, Var>);
compile_func!(compile_div_var_var, compile_div_var_var, Div<Var, Var, Var>);
compile_func!(compile_mul_var_var, compile_mul_var_var, Mul<Var, Var, Var>);
compile_func!(compile_or_var_var,  compile_or_var_var,  Or<Var, Var, Var>);
compile_func!(compile_sub_var_var, compile_sub_var_var, Sub<Var, Var, Var>);
compile_func!(compile_xor_var_var, compile_xor_var_var, Xor<Var, Var, Var>);

compile_func!(compile_add_var_type, compile_add_var_type, Add<Var, Type, Var>);
compile_func!(compile_and_var_type, compile_and_var_type, And<Var, Type, Var>);
compile_func!(compile_div_var_type, compile_div_var_type, Div<Var, Type, Var>);
compile_func!(compile_mul_var_type, compile_mul_var_type, Mul<Var, Type, Var>);
compile_func!(compile_or_var_type,  compile_or_var_type,  Or<Var, Type, Var>);
compile_func!(compile_sub_var_type,  compile_sub_var_type,  Sub<Var, Type, Var>);
compile_func!(compile_xor_var_type, compile_xor_var_type, Xor<Var, Type, Var>);

compile_func!(compile_add_type_type, compile_add_type_type, Add<Type, Type, Var>);
compile_func!(compile_and_type_type, compile_and_type_type, And<Type, Type, Var>);
compile_func!(compile_div_type_type, compile_div_type_type, Div<Type, Type, Var>);
compile_func!(compile_mul_type_type, compile_mul_type_type, Mul<Type, Type, Var>);
compile_func!(compile_or_type_type,  compile_or_type_type,  Or<Type, Type, Var>);
compile_func!(compile_sub_type_type, compile_sub_type_type, Sub<Type, Type, Var>);
compile_func!(compile_xor_type_type, compile_xor_type_type, Xor<Type, Type, Var>);
