use std::any::TypeId;

use super::*;

impl Ir for Assign<Var, Type> {
    fn dump(&self) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", self.inner1.name, meta, self.inner2.val())
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&meta.to_string(), ColorClass::Instr), 
            profile.markup(&self.inner2.val().to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        let op0Ty = self.inner1.ty;
        let op1Ty = self.inner2.into();
        if op0Ty != op1Ty {
            Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_assign_var_type(&self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_assign_var_type(&self, &block, module)
    }

    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl EvalOptVisitor for Assign<Var, Type> {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Ir for Assign<Var, Var> {
    fn dump(&self) -> String {
        let meta: TypeMetadata = self.inner2.ty;
        format!("{} = {} {}", self.inner1.name, meta, self.inner2.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let meta: TypeMetadata = self.inner2.ty;
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&meta.to_string(), ColorClass::Instr), 
            profile.markup(&self.inner2.name.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        let op0Ty = self.inner1.ty;
        let op1Ty = self.inner2.ty;
        if op0Ty != op1Ty {
            Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_assign_var_var(&self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else if *var == self.inner2 { true }
        else { false }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_assign_var_var(&self, &block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![self.inner2.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.inner2]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl EvalOptVisitor for Assign<Var, Var> {
    fn maybe_inline(&self, values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(lhs) = values.get(&self.inner2.name) {
            Some(Assign::new(self.inner1.to_owned(), *lhs))
        } else { None }
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Ir for Assign<Var, Const> {
    fn dump(&self) -> String {
        format!("{} = ptr {}", self.inner1.name, self.inner2.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup("ptr", ColorClass::Ty), 
            profile.markup(&self.inner2.name.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_assign_var_const(&self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_assign_var_const(&self, &block, module)
    }
    
    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl EvalOptVisitor for Assign<Var, Const> {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl<U> Assign<Var, U> where 
    U: AsAny + Debug + Clone + PartialEq + Eq + 'static
{
    /// Returns the type of the assignment
    pub fn getType(&self) -> TypeMetadata {
        self.inner1.ty
    }

    /// Returns the output variable
    pub fn getOut(&self) -> Var {
        self.inner1.to_owned()
    }

    /// Returns if the operand is an constant
    pub fn isOpConstVal(&self) -> bool {
        self.inner2.type_id() == TypeId::of::<Type>()
    }

    /// Returns if the operand is an constant ptr load
    pub fn isOpConstAdr(&self) -> bool {
        self.inner2.type_id() == TypeId::of::<Const>()
    }

    /// Returns the operand as a constant value
    pub fn getOpConstVal(&self) -> Type {
        self.inner2.as_any().downcast_ref::<Type>().unwrap().clone()
    }

    /// Returns the operand as a constant adr (Const data type)
    pub fn getOpConstAdr(&self) -> Const {
        self.inner2.as_any().downcast_ref::<Const>().unwrap().clone()
    }
}

/// Trait used for overloading the BuildAssign function
pub trait BuildAssign<T> {
    /// builds an assignment
    fn BuildAssign(&mut self, value: T) -> Var;
}
impl BuildAssign<Type> for Function {
    fn BuildAssign(&mut self, value: Type) -> Var {
        let block = self.blocks.get_mut(self.curr_block).expect("invalid current block");
        
        let out = Var::new(block, value.into());

        block.push_ir(Assign::new(out.clone(), value));

        out
    }
}

impl BuildAssign<Var> for Function {
    fn BuildAssign(&mut self, value: Var) -> Var {
        let block = self.blocks.get_mut(self.curr_block).expect("invalid current block");
        
        let out = Var::new(block, value.ty);

        block.push_ir(Assign::new(out.clone(), value));

        out
    }
}

impl BuildAssign<&Const> for Function {
    fn BuildAssign(&mut self, value: &Const) -> Var {
        let block = self.blocks.get_mut(self.curr_block).expect("invalid current block");
        
        let out = Var::new(block, TypeMetadata::ptr);

        block.push_ir(Assign::new(out.clone(), value.clone()));

        out
    }
}
