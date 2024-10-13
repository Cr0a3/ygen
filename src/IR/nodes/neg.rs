use super::*;

impl Ir for Neg<Var, /*out*/Var> {
    fn dump(&self) -> String {
        format!("{} = neg {} {}", self.inner2.name, self.inner1.ty, self.inner1.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {}", 
            profile.markup(&self.inner2.name, ColorClass::Var), 
            profile.markup("neg", ColorClass::Instr),
            profile.markup(&self.inner1.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.name, ColorClass::Var),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new( self.clone() )
    }

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_neg(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_neg(self, block, module)
    }

    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(value) = const_values.get(&self.inner1.name) {
            if self.inner1.ty != TypeMetadata::f32 || self.inner1.ty != TypeMetadata::f64 {
                let ty = Type::from_int(self.inner1.ty, -(value.val() as i64) as f64);
    
                Some(Assign::new(self.inner2.to_owned(), ty))
            } else { None}
        } else { None }
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.inner1.to_owned()]
    }

    fn output(&self) -> Option<Var> {
        Some(self.inner2.to_owned())
    }
}

impl Function {
    /// Builds a negate (negates the content of the variable)
    pub fn BuildNeg(&mut self, var: Var) -> Var {
        if !var.ty.signed() {
            panic!("variables need to be signed to get negated");
        }

        let block = self.blocks.back_mut().expect("expects current block");

        let out = Var::new(block, var.ty);

        block.push_ir(Neg::new(out.clone(), var));

        out
    }
}