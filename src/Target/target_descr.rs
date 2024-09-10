use std::error::Error;

use crate::prelude::{ir::*, Block, Var};
use crate::CodeGen::MCInstr;
use crate::CodeGen::{compilation::CompilationHelper, MachineInstr};
use crate::IR::{Const, Function, Type, TypeMetadata};

use super::{Triple, WhiteList};
use super::{CallConv, Compiler, Lexer};

/// The TargetBackendDescr is used to store all the functions/information to compile ir nodes into assembly
#[allow(unused)]
pub struct TargetBackendDescr {
    pub(crate) init: Option<fn(CallConv)->TargetBackendDescr>,

    pub(crate) lexer: Option<Box<dyn Lexer>>,
    pub(crate) compile: Option<Box<dyn Compiler>>,

    pub(crate) helper: Option<CompilationHelper>,

    pub(crate) block: Option<Block>,
    pub(crate) call: CallConv,

    pub(crate) sink: Vec<MachineInstr>,

    pub(crate) whitelist: WhiteList,
}

macro_rules! compile_func {
    ($name:ident, $func:ident, $($node:tt)*) => { 
        impl TargetBackendDescr { 
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

impl TargetBackendDescr {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            init: None,
            lexer: None,
            compile: None,
            block: None,
            call: CallConv::SystemV,
            helper: None,
            whitelist: WhiteList::new(),
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

    /// builds all ir nodes of the current block into a vector of MachineInstr
    pub fn build_instrs(&mut self, func: &Function, triple: &Triple) -> Vec<MachineInstr> {
        let helper = if let Some(helper) = &mut self.helper { helper }
        else { panic!("no current compilation helper"); };

        if helper.arch != triple.arch {
            panic!("the architecture of the triple {:?} isn't the same as the one of the compilation helper {:?}", triple.arch, helper.arch)
        }

        let block = if let Some(block) = &self.block {
            block.clone()
        } else {
            panic!("no current block");
        };

        helper.build_argument_preprocessing(func);

        for node in block.nodes {
            node.compile(self);
        }

        let out = self.sink.clone();

        out
    }

    /// Resets all values to "factory standart"
    pub fn reset(&mut self) {
        if let Some(init) = self.init {
            let reference = init(self.call);
            self.init = reference.init;
            self.lexer = reference.lexer;
            self.compile = reference.compile;
            self.helper = reference.helper;
            self.block = reference.block;
            assert_eq!(self.call, reference.call);
            self.call = reference.call;
            self.sink = reference.sink;
            self.whitelist = reference.whitelist;
        }
    }

    /// Used for lowering machine instructions into dyn MCInstr
    pub fn lower(&self, instrs: Vec<MachineInstr>) -> Result<Vec<Box<dyn MCInstr>>, Box<dyn Error>> {
        if let Some(helper) = &self.helper {
            self.whitelist.check_for_forbidden_mnemonics(&instrs)?;

            if let Some(lower) = helper.lower {
                Ok(lower(self.call, instrs))
            } else {
                todo!("the target architecture {:?} doesn't support instruction lowering", helper.arch)
            }
        } else {
            todo!("no helper was registered");
        }
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

compile_func!(compile_ret_ty, compile_ret_ty, Return<Type>);
compile_func!(compile_ret_var, compile_ret_var, Return<Var>);

compile_func!(compile_cast_var, compile_cast, Cast<Var, TypeMetadata, Var>);

compile_func!(compile_call, compile_call, Call<Function, Vec<Var>, Var>);

compile_func!(compile_assign_var_type, compile_assign_var_type, Assign<Var, Type>);
compile_func!(compile_assign_var_var, compile_assign_var_var, Assign<Var, Var>);
compile_func!(compile_assign_var_const, compile_assign_var_const, Assign<Var, Const>);

compile_func!(compile_br, compile_br, Br<Box<Block>>);