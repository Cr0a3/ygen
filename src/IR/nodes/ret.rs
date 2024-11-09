use super::*;

impl Ir for Return<IROperand> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        let metadata: TypeMetadata = self.inner1.get_ty();
        
        format!("ret {} {}", metadata, self.inner1)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let metadata: TypeMetadata = self.inner1.get_ty();
        format!("{} {} {}", 
            profile.markup("ret", ColorClass::Instr),
            profile.markup(&metadata.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.to_string(), ColorClass::Var),
        )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.get_ty();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_ret(&self, module)
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_ret(&self, block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        if let IROperand::Var(ret) = &self.inner1 { vec![ret.to_owned()] }
        else { vec![] }
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        None
    }
}

impl EvalOptVisitor for Return<IROperand> {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let IROperand::Var(var) = &self.inner1 {
            if let Some(constant) = const_values.get(&var.name) {
                return Some( Return::new(IROperand::Type(*constant)) );
            } 
        }

        None
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
        if let Some(op) = self.inner1.as_any().downcast_ref::<IROperand>() { 
            op.is_type() 
        } else { panic!() }
    }

    /// Returns the node a variable?
    pub fn isRetVar(&self) -> bool {
        if let Some(op) = self.inner1.as_any().downcast_ref::<IROperand>() { 
            op.is_var() 
        } else { panic!() }
    }

    /// Returns the constant the node returns (else panics)
    pub fn getRetConst(&self) -> Type {
        if let Some(op) = self.inner1.as_any().downcast_ref::<IROperand>() { 
            op.get_typeconst() 
        } else { panic!() }
    }

    /// Returns the variable the node returns (else panics)
    pub fn getRetVar(&self) -> Var {
        if let Some(op) = self.inner1.as_any().downcast_ref::<IROperand>() { 
            op.get_var() 
        } else { panic!() }
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
            .push_ir(Return::new(IROperand::Type(val)))
    }
}

impl BuildReturn<Var> for Function {
    fn BuildRet(&mut self, var: Var) {
        self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(IROperand::Var(var)))
    }
}