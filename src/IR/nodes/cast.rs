use super::*;

impl Ir for Cast<Var, TypeMetadata, Var> {
    fn dump(&self) -> String {
        format!("{} = cast {} {} to {}", self.inner3.name, self.inner1.ty, self.inner1.name, self.inner2)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {} {} {}", 
            profile.markup(&self.inner3.name, ColorClass::Var), 
            profile.markup(&"cast", ColorClass::Instr),
            profile.markup(&self.inner1.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&"to", ColorClass::Instr),
            profile.markup(&self.inner2.to_string(), ColorClass::Ty),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        if self.inner3.ty != self.inner2 {
            Err(VerifyError::Op0Op1TyNoMatch(self.inner3.ty, self.inner2))?
        }
        Ok(())
    }

    fn uses(&self, var: &Var) -> bool {
        let var = var.to_owned();

        if var == self.inner1 || var == self.inner3 {
            true
        } else { false }
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_cast_var(&self, module)
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_cast(&self, &block, module)
    }

    
    fn inputs(&self) -> Vec<Var> {
        vec![self.inner1.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.inner1]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner3.to_owned())
    }
}

impl EvalOptVisitor for Cast<Var, TypeMetadata, Var> {
    fn maybe_inline(&self, vars: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(var) = vars.get(&self.inner1.name) {
            Some(Assign::new(self.inner3.to_owned(), *var))
        } else { return None; }
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.inner2 == self.inner1.ty {
            Some(Assign::new(self.inner3.to_owned(), self.inner1.to_owned()))
        } else { None }
    }
}

/// Trait for the cast instruction
/// Used for overloading the BuildCast function
pub trait BuildCast<T, U> {
    /// builds an cast to form one variable into another type
    fn BuildCast(&mut self, var: T, ty: U) -> Var;
}

impl BuildCast<Var, TypeMetadata> for Function {
    fn BuildCast(&mut self, var: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir(Cast::new(var, ty, out.clone()));

        out
    }
}
