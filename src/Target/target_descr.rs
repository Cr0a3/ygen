use std::error::Error;
use crate::debug::DebugLocation;
use crate::prelude::{ir::*, Block, Var};
use crate::CodeGen::{IrCodeGenArea, IrCodeGenHelper, MCDocInstr, MCInstr};
use crate::CodeGen::{compilation::CompilationHelper, MachineInstr};
use crate::IR::{Const, Module, Type};

use super::{AsmPrinter, Triple, WhiteList};
use super::{CallConv, Compiler, Lexer};

/// The TargetBackendDescr is used to store all the functions/information to compile ir nodes into assembly
#[allow(unused)]
#[derive(Clone)]
pub struct TargetBackendDescr {
    pub(crate) init: Option<fn(CallConv)->TargetBackendDescr>,

    pub(crate) lexer: Option<Box<dyn Lexer>>,
    pub(crate) compile: Option<Box<dyn Compiler>>,

    pub(crate) helper: Option<CompilationHelper>,

    pub(crate) block: Option<Block>,
    pub(crate) call: CallConv,

    pub(crate) sink: Vec<MachineInstr>,

    pub(crate) epilog: bool,

    pub(crate) whitelist: WhiteList,

    pub(crate) printer: Option<Box<dyn AsmPrinter>>,
}

macro_rules! compile_func {
    ($name:ident, $func:ident, $($node:tt)*) => { 
        impl TargetBackendDescr { 
            /// gets the callback for compiling the  ir node into asm
            #[allow(unused)]
            pub(crate) fn $name(&mut self, node: &$($node)*, module: &mut Module) {
                if let Some(helper) = &mut self.helper {
                    if let Some(block) = &self.block {
                        let mut vsink = Vec::new();
                        helper.$func(node, &mut vsink, block, module);

                        let mut vsink2 = Vec::new();
                        for inst in &mut vsink {
                            inst.turn_into_float_if_needed();
                            vsink2.extend_from_slice(&inst.fix_const_imm(helper, module));
                        }

                        self.sink.extend_from_slice(&vsink2);
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
            epilog: false,
            printer: None,
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
    pub fn build_instrs(&mut self, triple: &Triple, module: &mut Module) -> Vec<MachineInstr> {
        let areas = self.build_instrs_with_ir_debug(triple, module);

        let mut merged = vec![];

        for area in areas {
            merged.extend_from_slice(&area.compiled);
        }

        merged
    }

    /// builds the instruction with ir debug metadata
    pub fn build_instrs_with_ir_debug(&mut self, triple: &Triple, module: &mut Module) -> Vec<IrCodeGenArea> {
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

        let mut ir_helper = IrCodeGenHelper::new(helper.to_owned());

        for node in block.nodes.to_owned() {
            if ir_helper.helper.alloc.epilog {
                self.epilog = true;
            }

            // VERY UGLY CODE WHICH SYNCS THE MAX STACK_OFF 
            // OF EITHER ir_helper or helper (the one who has the biggest gets selected)
            if helper.alloc.stack_off < ir_helper.helper.alloc.stack_off {
                helper.alloc.stack_off = ir_helper.helper.alloc.stack_off;
            } else if ir_helper.helper.alloc.stack_off < helper.alloc.stack_off {
                ir_helper.helper.alloc.stack_off = helper.alloc.stack_off;
            }

            if let Some(node) = node.as_any().downcast_ref::<Return>() {
                ir_helper.compile_ret(node, &block, module);

                if self.epilog {
                    let mut epilog_instrs = vec![];
                    helper.compile_epilog(&mut epilog_instrs);
    
                    if let Some(last) = ir_helper.compiled.last_mut() {
                        let backup = last.compiled.clone();
                        last.compiled = epilog_instrs;
                        last.compiled.extend_from_slice(&backup);
                    } else { unreachable!() }

                }
            } else {
                node.compile_dir(&mut ir_helper, &block, module);
            }
        }

        *helper = ir_helper.helper;

        ir_helper.compiled
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
            self.epilog = false;
        }
    }

    /// Used for lowering machine instructions into dyn MCInstr
    pub fn lower(&self, instrs: Vec<MachineInstr>) -> Result<Vec<Box<dyn MCInstr>>, Box<dyn Error>> {
        if let Some(helper) = &self.helper {
            self.whitelist.check_for_forbidden_mnemonics(&instrs)?;

            let mut mc_instrs = vec![];

            if let Some(lower) = helper.lower {
                let lowered = lower(self.call, instrs);

                mc_instrs.extend_from_slice(&lowered);
            } else {
                todo!("the target architecture {:?} doesn't support instruction lowering", helper.arch)
            };

            Ok(mc_instrs)
        } else {
            todo!("no helper was registered");
        }
    }

    /// loweres the machine instructions into dyn MCInstr but with debug information
    pub fn lower_debug(&self, areas: Vec<IrCodeGenArea>) -> Result< Vec<(Vec<Box<dyn MCInstr>>, DebugLocation)>, Box<dyn Error>> {
        if let Some(helper) = &self.helper {
            let mut debug = Vec::new();

            for area in areas {
                let mut mc_instrs = vec![];

                if let Some(node) = area.node {
                    mc_instrs.push( MCDocInstr::doc(node.dump()) );
                }

                let instrs = area.compiled;

                self.whitelist.check_for_forbidden_mnemonics(&instrs)?;
    
    
                if let Some(lower) = helper.lower {
                    let lowered = lower(self.call, instrs);
    
                    mc_instrs.extend_from_slice(&lowered);
                } else {
                    todo!("the target architecture {:?} doesn't support instruction lowering", helper.arch)
                };

                if let Some(debug_info) = area.debug_info {
                    debug.push((mc_instrs, debug_info));
                } else {
                    debug.push((mc_instrs, DebugLocation {
                        line: 0,
                        col: 0,
                        epilog: false,
                        prolog: false,
                        adr: 0,
                    }));
                }
            }

            Ok(debug)
        } else {
            todo!("no helper was registered");
        }       
    }
}

compile_func!(compile_add, compile_add, Add);
compile_func!(compile_and, compile_and, And);
compile_func!(compile_div, compile_div, Div);
compile_func!(compile_mul, compile_mul, Mul);
compile_func!(compile_or,  compile_or,  Or);
compile_func!(compile_sub, compile_sub, Sub);
compile_func!(compile_xor, compile_xor, Xor);
compile_func!(compile_rem, compile_rem, Rem);
compile_func!(compile_shl, compile_shl, Shl);
compile_func!(compile_shr, compile_shr, Shr);

compile_func!(compile_ret, compile_ret, Return);

compile_func!(compile_cast_var, compile_cast, Cast);

compile_func!(compile_call, compile_call, Call);

compile_func!(compile_assign_var_type, compile_assign_var_type, Assign<Var, Type>);
compile_func!(compile_assign_var_var, compile_assign_var_var, Assign<Var, Var>);
compile_func!(compile_assign_var_const, compile_assign_var_const, Assign<Var, Const>);

compile_func!(compile_br, compile_br, Br);
compile_func!(compile_br_cond, compile_br_cond, BrCond);

compile_func!(compile_cmp, compile_cmp, Cmp);

compile_func!(compile_alloca, compile_alloca, Alloca);
compile_func!(compile_store, compile_store, Store);
compile_func!(compile_load, compile_load, Load);

compile_func!(compile_switch, compile_switch, Switch);

compile_func!(compile_neg, compile_neg, Neg);

compile_func!(compile_select, compile_select, Select);

compile_func!(compile_getelemptr, compile_getelemptr, GetElemPtr);