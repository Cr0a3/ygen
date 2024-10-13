use std::error::Error;
use crate::debug::DebugLocation;
use crate::prelude::{ir::*, Block, Var};
use crate::CodeGen::{IrCodeGenArea, IrCodeGenHelper, MCDocInstr, MCInstr};
use crate::CodeGen::{compilation::CompilationHelper, MachineInstr};
use crate::IR::{BlockId, Const, FuncId, Module, Type, TypeMetadata};

use super::{Triple, WhiteList};
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

                        for inst in &mut vsink {
                            inst.turn_into_float_if_needed();
                            inst.fix_const_imm(module);
                        }

                        self.sink.extend_from_slice(&vsink);
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
            if ir_helper.helper.epilog() {
                self.epilog = true;
            }

            // VERY UGLY CODE WHICH SINCRONICES THE MAX STACK_OFF 
            // OF EITHER ir_helper or helper (the one who has the biggest gets selected)
            if helper.alloc.stack_off < ir_helper.helper.alloc.stack_off {
                helper.alloc.stack_off = ir_helper.helper.alloc.stack_off;
            } else if ir_helper.helper.alloc.stack_off < helper.alloc.stack_off {
                ir_helper.helper.alloc.stack_off = helper.alloc.stack_off;
            }

            if let Some(node) = node.as_any().downcast_ref::<Return<Type>>() {
                ir_helper.compile_ret_ty(node, &block, module);

                if self.epilog {
                    let mut epilog_instrs = vec![];
                    helper.compile_epilog(&mut epilog_instrs);
    
                    if let Some(last) = ir_helper.compiled.last_mut() {
                        let backup = last.compiled.clone();
                        last.compiled = epilog_instrs;
                        last.compiled.extend_from_slice(&backup);
                    } else { unreachable!() }

                }
            } else if let Some(node) = node.as_any().downcast_ref::<Return<Var>>() {
                ir_helper.compile_ret_var(node, &block, module);

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

compile_func!(compile_add_var_var, compile_add_var_var, Add<Var, Var, Var>);
compile_func!(compile_and_var_var, compile_and_var_var, And<Var, Var, Var>);
compile_func!(compile_div_var_var, compile_div_var_var, Div<Var, Var, Var>);
compile_func!(compile_mul_var_var, compile_mul_var_var, Mul<Var, Var, Var>);
compile_func!(compile_or_var_var,  compile_or_var_var,  Or<Var, Var, Var>);
compile_func!(compile_sub_var_var, compile_sub_var_var, Sub<Var, Var, Var>);
compile_func!(compile_xor_var_var, compile_xor_var_var, Xor<Var, Var, Var>);
compile_func!(compile_rem_var_var, compile_rem_var_var, Rem<Var, Var, Var>);
compile_func!(compile_shl_var_var, compile_shl_var_var, Shl<Var, Var, Var>);
compile_func!(compile_shr_var_var, compile_shr_var_var, Shr<Var, Var, Var>);

compile_func!(compile_add_var_type, compile_add_var_type, Add<Var, Type, Var>);
compile_func!(compile_and_var_type, compile_and_var_type, And<Var, Type, Var>);
compile_func!(compile_div_var_type, compile_div_var_type, Div<Var, Type, Var>);
compile_func!(compile_mul_var_type, compile_mul_var_type, Mul<Var, Type, Var>);
compile_func!(compile_or_var_type,  compile_or_var_type,  Or<Var, Type, Var>);
compile_func!(compile_sub_var_type,  compile_sub_var_type,  Sub<Var, Type, Var>);
compile_func!(compile_xor_var_type, compile_xor_var_type, Xor<Var, Type, Var>);
compile_func!(compile_rem_var_type, compile_rem_var_type, Rem<Var, Type, Var>);
compile_func!(compile_shl_var_type, compile_shl_var_type, Shl<Var, Type, Var>);
compile_func!(compile_shr_var_type, compile_shr_var_type, Shr<Var, Type, Var>);

compile_func!(compile_add_type_type, compile_add_type_type, Add<Type, Type, Var>);
compile_func!(compile_and_type_type, compile_and_type_type, And<Type, Type, Var>);
compile_func!(compile_div_type_type, compile_div_type_type, Div<Type, Type, Var>);
compile_func!(compile_mul_type_type, compile_mul_type_type, Mul<Type, Type, Var>);
compile_func!(compile_or_type_type,  compile_or_type_type,  Or<Type, Type, Var>);
compile_func!(compile_sub_type_type, compile_sub_type_type, Sub<Type, Type, Var>);
compile_func!(compile_xor_type_type, compile_xor_type_type, Xor<Type, Type, Var>);
compile_func!(compile_rem_type_type, compile_rem_type_type, Rem<Type, Type, Var>);
compile_func!(compile_shl_type_type, compile_shl_type_type, Shl<Type, Type, Var>);
compile_func!(compile_shr_type_type, compile_shr_type_type, Shr<Type, Type, Var>);

compile_func!(compile_ret_ty, compile_ret_ty, Return<Type>);
compile_func!(compile_ret_var, compile_ret_var, Return<Var>);

compile_func!(compile_cast_var, compile_cast, Cast<Var, TypeMetadata, Var>);

compile_func!(compile_call, compile_call, Call<FuncId, Vec<Var>, Var>);

compile_func!(compile_assign_var_type, compile_assign_var_type, Assign<Var, Type>);
compile_func!(compile_assign_var_var, compile_assign_var_var, Assign<Var, Var>);
compile_func!(compile_assign_var_const, compile_assign_var_const, Assign<Var, Const>);

compile_func!(compile_br, compile_br, Br<BlockId>);
compile_func!(compile_br_cond, compile_br_cond, BrCond<Var, BlockId, BlockId>);

compile_func!(compile_cmp, compile_cmp, Cmp);

compile_func!(compile_alloca, compile_alloca, Alloca<Var, TypeMetadata>);
compile_func!(compile_store, compile_store, Store<Var, Var>);
compile_func!(compile_store_ty, compile_store_ty, Store<Var, Type>);
compile_func!(compile_load, compile_load, Load<Var, Var, TypeMetadata>);

compile_func!(compile_switch, compile_switch, Switch);

compile_func!(compile_neg, compile_neg, Neg<Var, Var>);

compile_func!(compile_select_tt, compile_select_tt, Select<Type, Type>);
compile_func!(compile_select_vt, compile_select_vt, Select<Var, Type>);
compile_func!(compile_select_tv, compile_select_tv, Select<Type, Var>);
compile_func!(compile_select_vv, compile_select_vv, Select<Var, Var>);

compile_func!(compile_getelemptr, compile_getelemptr, GetElemPtr);