use std::any::TypeId;

use super::*;

impl Ir for Return<Type> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("ret {} {}", metadata, self.inner1.val())
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("{} {} {}", 
            profile.markup("ret", ColorClass::Instr),
            profile.markup(&metadata.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.val().to_string(), ColorClass::Var),
        )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_ret_ty(&self, module)
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_ret_ty(&self, block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        None
    }
}

impl EvalOptVisitor for Return<Type> {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Ir for Return<Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        format!("ret {} {}", self.inner1.ty, self.inner1.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} {} {}", 
            profile.markup("ret", ColorClass::Instr), 
            profile.markup(&self.inner1.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.name.to_string(), ColorClass::Var),
        )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.ty.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_ret_var(&self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_ret_var(&self, &block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![self.inner1.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.inner1]
    }
    
    fn output(&self) -> Option<Var> {
        None
    }
}

impl EvalOptVisitor for Return<Var> {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(constant) = const_values.get(&self.inner1.name) {
            Some( Return::new(*constant) )
        } else { None }
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl<T> Return<T> where 
    T: Clone + AsAny + 'static
{
    /// Returns the node a constant type?
    pub fn isRetConst(&self) -> bool {
        self.inner1.type_id() == TypeId::of::<Type>()
    }

    /// Returns the node a variable?
    pub fn isRetVar(&self) -> bool {
        self.inner1.type_id() == TypeId::of::<Var>()
    }

    /// Returns the constant the node returns (else panics)
    pub fn getRetConst(&self) -> Type {
        self.inner1.as_any().downcast_ref::<Type>().unwrap().to_owned()
    }

    /// Returns the variable the node returns (else panics)
    pub fn getRetVar(&self) -> Var {
        self.inner1.as_any().downcast_ref::<Var>().unwrap().to_owned()
    }
}

/// Trait for the return instruction
/// Used for overloading the BuildRet function
pub trait BuildReturn<T> {
    /// Returns specified value
    fn BuildRet(&mut self, val: T);
}

impl BuildReturn<Type> for Function {
    fn BuildRet(&mut self, val: Type) {
        self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(val))
    }
}

impl BuildReturn<Var> for Function {
    fn BuildRet(&mut self, var: Var) {
        self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(var))
    }
}