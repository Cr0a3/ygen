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

    fn compile(&self, registry: &mut TargetBackendDescr) {
        registry.compile_ret_ty(&self)
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

    fn compile(&self, registry: &mut TargetBackendDescr) {
        registry.compile_ret_var(&self)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
}

/// Trait for the return instruction
/// Used for overloading the BuildRet function
pub trait BuildReturn<T> {
    /// Returns specified value
    fn BuildRet(&mut self, val: T);
}

impl BuildReturn<Type> for IRBuilder<'_> {
    fn BuildRet(&mut self, val: Type) {
        self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(val))
    }
}

impl BuildReturn<Var> for IRBuilder<'_> {
    fn BuildRet(&mut self, var: Var) {
        self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(var))
    }
}