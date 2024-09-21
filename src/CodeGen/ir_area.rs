use crate::prelude::Ir;
use crate::CodeGen::MachineInstr;

use crate::IR::{ir, Block, Const, Function, Type, TypeMetadata, Var};

use super::CompilationHelper;

/// The instructions for a single ir node
#[derive(Debug, Clone, Eq)]
pub struct IrCodeGenArea {
    pub(crate) node: Option<Box<dyn Ir>>,
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
    helper: CompilationHelper,
}

impl IrCodeGenHelper {
    /// creates a new `IrCodeGenHelper`
    pub fn new(compiler: CompilationHelper) -> Self {
        Self {
            compiled: vec![],
            helper: compiler,
        }
    }
}

macro_rules! ir_codegen_wrap {
    ($func:ident,  $comment:expr, $($node:tt)*) => {
        #[doc = $comment]
        pub fn $func(&mut self, node: &$($node)*, block: &Block) {
            let mut area = IrCodeGenArea {
                node: Some(node.clone_box()),
                compiled: Vec::new(),
            };

            self.helper.$func(node, &mut area.compiled, block);

            self.compiled.push(area);
        }
    };
}

impl IrCodeGenHelper {
    ir_codegen_wrap!(
        compile_add_var_var, 
        "Loweres the add node", 
        ir::Add<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_sub_var_var, 
        "Loweres the sub node", 
        ir::Sub<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_and_var_var, 
        "Loweres the and node", 
        ir::And<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_div_var_var, 
        "Loweres the div node", 
        ir::Div<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_mul_var_var, 
        "Loweres the mul node", 
        ir::Mul<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_or_var_var, 
        "Loweres the or node", 
        ir::Or<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_xor_var_var, 
        "Loweres the xor node", 
        ir::Xor<Var, Var, Var>
    );
    ir_codegen_wrap!(
        compile_add_var_type, 
        "Loweres the add node", 
        ir::Add<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_sub_var_type, 
        "Loweres the sub node", 
        ir::Sub<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_and_var_type, 
        "Loweres the and node", 
        ir::And<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_div_var_type, 
        "Loweres the div node", 
        ir::Div<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_mul_var_type, 
        "Loweres the mul node", 
        ir::Mul<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_or_var_type, 
        "Loweres the or node", 
        ir::Or<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_xor_var_type, 
        "Loweres the xor node", 
        ir::Xor<Var, Type, Var>
    );
    ir_codegen_wrap!(
        compile_add_type_type, 
        "Loweres the add node", 
        ir::Add<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_sub_type_type, 
        "Loweres the sub node", 
        ir::Sub<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_and_type_type, 
        "Loweres the and node", 
        ir::And<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_div_type_type, 
        "Loweres the div node", 
        ir::Div<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_mul_type_type, 
        "Loweres the mul node", 
        ir::Mul<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_or_type_type, 
        "Loweres the or node", 
        ir::Or<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_xor_type_type, 
        "Loweres the xor node", 
        ir::Xor<Type, Type, Var>
    );
    ir_codegen_wrap!(
        compile_alloca, 
        "Loweres the alloca node", 
        ir::Alloca<Var, TypeMetadata>
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
        ir::Br<Box<Block>>
    );
    ir_codegen_wrap!(
        compile_br_cond, 
        "Loweres the br cond node", 
        ir::BrCond<Var, Block, Block>
    );
    ir_codegen_wrap!(
        compile_call, 
        "Loweres the call node", 
        ir::Call<Function, Vec<Var>, Var>
    );
    ir_codegen_wrap!(
        compile_cast, 
        "Loweres the cast node", 
        ir::Cast<Var, TypeMetadata, Var>
    );
    ir_codegen_wrap!(
        compile_cmp, 
        "Loweres the cmp node", 
        ir::Cmp
    );
    ir_codegen_wrap!(
        compile_load, 
        "Loweres the load node", 
        ir::Load<Var, Var, TypeMetadata>
    );
    ir_codegen_wrap!(
        compile_ret_ty, 
        "Loweres the ret node", 
        ir::Return<Type>
    );
    ir_codegen_wrap!(
        compile_ret_var, 
        "Loweres the ret node", 
        ir::Return<Var>
    );
    ir_codegen_wrap!(
        compile_store, 
        "Loweres the store node", 
        ir::Store<Var, Var>
    );
    ir_codegen_wrap!(
        compile_store_ty, 
        "Loweres the store node", 
        ir::Store<Var, Type> 
    );
}