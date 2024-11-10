use super::*;

impl Ir for Cast {
    fn dump(&self) -> String {
        format!("{} = cast {} {} to {}", self.inner3.name, self.inner1.get_ty(), self.inner1, self.inner2)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {} {} {}", 
            profile.markup(&self.inner3.name, ColorClass::Var), 
            profile.markup(&"cast", ColorClass::Instr),
            profile.markup(&self.inner1.get_ty().to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.to_string(), ColorClass::Var), 
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

        if let IROperand::Var(value) = &self.inner1 {
            if var.name == value.name {
                return true;
            } 
        }
        
        false
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
        let mut inputs = Vec::new();

        if let IROperand::Var(var) = &self.inner1 {
            inputs.push(var.to_owned());
        }

        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();

        if let IROperand::Var(var) = &mut self.inner1 {
            inputs.push(var);
        }

        inputs
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner3.to_owned())
    }
}

impl Cast {
    /// Returns the input as a variable
    pub fn getInputVar(&self) -> Var {
        self.inner1.get_var()
    }

    /// Returns if the input is a var
    pub fn isInputVar(&self) -> bool {
        self.inner1.is_var()
    }

    /// Returns the input as a constant number
    pub fn getInputConst(&self) -> Type {
        self.inner1.get_typeconst()
    }

    /// Returns if the input is a constant number
    pub fn isInputConst(&self) -> bool {
        self.inner1.is_type()
    }

    /// Returns the output variable
    pub fn getOutput(&self) -> Var {
        self.inner3.to_owned()
    }

    /// Returns the type to which we cast
    pub fn getCastType(&self) -> TypeMetadata {
        self.inner2.to_owned()
    }

    /// Returns the type from which we cast
    pub fn getFromType(&self) -> TypeMetadata {
        self.inner1.get_ty()
    }
}

impl EvalOptVisitor for Cast {
    fn maybe_inline(&self, vars: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let IROperand::Var(value) = &self.inner1 {
            if let Some(var) = vars.get(&value.name) {
                return Some(Assign::new(self.inner3.to_owned(), *var));
            } 
        }
        
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.inner2 == self.inner1.get_ty() {
            match &self.inner1 {
                IROperand::Type(ty) => Some(Assign::new(self.inner3.to_owned(), ty.to_owned())),
                IROperand::Var(var) => Some(Assign::new(self.inner3.to_owned(), var.to_owned())),
            }
        } else { None }
    }
}

/// Trait for the cast instruction
/// Used for overloading the BuildCast function
pub trait BuildCast<T, U> {
    /// builds an cast to form one variable into another type
    fn BuildCast(&mut self, value: T, ty: U) -> Var;
}

impl BuildCast<Var, TypeMetadata> for Function {
    fn BuildCast(&mut self, var: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir(Cast::new(IROperand::Var(var), ty, out.clone()));

        out
    }
}

impl BuildCast<Type, TypeMetadata> for Function {
    fn BuildCast(&mut self, value: Type, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir(Cast::new(IROperand::Type(value), ty, out.clone()));

        out
    }
}