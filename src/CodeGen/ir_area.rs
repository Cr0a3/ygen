use crate::debug::DebugLocation;
use crate::prelude::{DebugNode, Ir};
use crate::CodeGen::MachineInstr;

use crate::IR::{ir, Block, Const, Type, Var};

use super::CompilationHelper;

/// The instructions for a single ir node
#[derive(Debug, Clone, Eq)]
pub struct IrCodeGenArea {
    pub(crate) node: Option<Box<dyn Ir>>,
    pub(crate) debug_info: Option<DebugLocation>,
    pub(crate) compiled: Vec<MachineInstr>, 
}

impl PartialEq for IrCodeGenArea {
    fn eq(&self, other: &Self) -> bool {
        &self.node == &other.node && self.compiled == other.compiled
    }
}

/// `CompilationHelper` but with node metadata
pub struct IrCodeGenHelper {
    pub(crate) compiled: Vec<IrCodeGenArea>,
    pub(crate) helper: CompilationHelper,
    pub(crate) debug_program: Option<DebugNode>,
}

impl IrCodeGenHelper {
    /// creates a new `IrCodeGenHelper`
    pub fn new(compiler: CompilationHelper) -> Self {
        Self {
            compiled: vec![],
            helper: compiler,
            debug_program: None,
        }
    }

    pub(crate) fn set_location_node(&mut self, node: &DebugNode) {
        self.debug_program = Some(node.to_owned());
    }

    pub(crate) fn get_location(&self) -> Option<DebugLocation> {
        if let Some(prog) = &self.debug_program {
            Some(
                DebugLocation { 
                    line: prog.line as u64, 
                    col: prog.coloumn as u64, 
                    epilog: false, 
                    prolog: false,
                    adr: 0
                }
            )
        } else { None }
    }
}

macro_rules! ir_codegen_wrap {
    ($func:ident,  $comment:expr, $($node:tt)*) => {
        #[doc = $comment]
        pub fn $func(&mut self, node: &$($node)*, block: &Block, module: &mut crate::prelude::Module) {
            let mut area = IrCodeGenArea {
                node: Some(node.clone_box()),
                compiled: Vec::new(),
                debug_info: self.get_location(),
            };

            self.helper.$func(node, &mut area.compiled, block, module);

            let mut fixed = Vec::new();

            for inst in &mut area.compiled {
                inst.turn_into_float_if_needed();
                fixed.extend_from_slice(
                    &inst.fix_const_imm(&mut self.helper, module)
                );
            }

            area.compiled = fixed;

            self.compiled.push(area);
        }
    };
}

impl IrCodeGenHelper {
    ir_codegen_wrap!(
        compile_add, 
        "Loweres the add node", 
        ir::Add
    );
    ir_codegen_wrap!(
        compile_sub, 
        "Loweres the sub node", 
        ir::Sub
    );
    ir_codegen_wrap!(
        compile_xor, 
        "Loweres the xor node", 
        ir::Xor
    );
    ir_codegen_wrap!(
        compile_or, 
        "Loweres the or node", 
        ir::Or
    );
    ir_codegen_wrap!(
        compile_and, 
        "Loweres the and node", 
        ir::And
    );
    ir_codegen_wrap!(
        compile_mul, 
        "Loweres the mul node", 
        ir::Mul
    );
    ir_codegen_wrap!(
        compile_div, 
        "Loweres the div node", 
        ir::Div
    );
    ir_codegen_wrap!(
        compile_rem, 
        "Loweres the rem node", 
        ir::Rem
    );
    ir_codegen_wrap!(
        compile_shl, 
        "Loweres the shl node", 
        ir::Shl
    );
    ir_codegen_wrap!(
        compile_shr, 
        "Loweres the shr node", 
        ir::Shr
    );
    ir_codegen_wrap!(
        compile_alloca, 
        "Loweres the alloca node", 
        ir::Alloca
    );
    ir_codegen_wrap!(
        compile_assign_var_type, 
        "Loweres the assign node", 
        ir::Assign<Var, Type>
    );
    ir_codegen_wrap!(
        compile_assign_var_var, 
        "Loweres the assign node", 
        ir::Assign<Var, Var>
    );
    ir_codegen_wrap!(
        compile_assign_var_const, 
        "Loweres the assign node", 
        ir::Assign<Var, Const>
    );
    ir_codegen_wrap!(
        compile_br, 
        "Loweres the xor node", 
        ir::Br
    );
    ir_codegen_wrap!(
        compile_br_cond, 
        "Loweres the br cond node", 
        ir::BrCond
    );
    ir_codegen_wrap!(
        compile_call, 
        "Loweres the call node", 
        ir::Call
    );
    ir_codegen_wrap!(
        compile_cast, 
        "Loweres the cast node", 
        ir::Cast
    );
    ir_codegen_wrap!(
        compile_cmp, 
        "Loweres the cmp node", 
        ir::Cmp
    );
    ir_codegen_wrap!(
        compile_load, 
        "Loweres the load node", 
        ir::Load
    );
    ir_codegen_wrap!(
        compile_ret, 
        "Loweres the ret node", 
        ir::Return
    );
    ir_codegen_wrap!(
        compile_store, 
        "Loweres the store node", 
        ir::Store
    );
    ir_codegen_wrap!(
        compile_switch, 
        "Loweres the switch node", 
        ir::Switch
    );
    ir_codegen_wrap!(
        compile_neg, 
        "Loweres the neg node", 
        ir::Neg
    );
    ir_codegen_wrap!(
        compile_select, 
        "Loweres the select ty ty node", 
        ir::Select
    );
    ir_codegen_wrap!(
        compile_getelemptr, 
        "Loweres the getelemptr node", 
        ir::GetElemPtr
    );
}

impl Into<Vec<MachineInstr>> for IrCodeGenHelper {
    fn into(self) -> Vec<MachineInstr> {
        let mut merged = vec![];

        for area in &self.compiled {
            merged.extend_from_slice(&area.compiled);
        }

        merged
    }
}