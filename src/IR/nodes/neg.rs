use super::*;

impl Ir for Neg {
    fn dump(&self) -> String {
        format!("{} = neg {} {}", self.inner2.name, self.inner1.get_ty(), self.inner1)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {}", 
            profile.markup(&self.inner2.name, ColorClass::Var), 
            profile.markup("neg", ColorClass::Instr),
            profile.markup(&self.inner1.get_ty().to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.to_string(), ColorClass::Var),
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

    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();
        if let IROperand::Var(value) = &self.inner1 { inputs.push(value.to_owned()); }
        inputs
    }

    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();
        if let IROperand::Var(value) = &mut self.inner1 { inputs.push( value); }
        inputs
    }

    fn output(&self) -> Option<Var> {
        Some(self.inner2.to_owned())
    }

    fn ty(&self) -> Option<TypeMetadata> {
        Some(self.inner1.get_ty())
    }
}

impl EvalOptVisitor for Neg {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let IROperand::Var(var) = &self.inner1 {
            if let Some(value) = const_values.get(&var.name) {
                if !var.ty.float() {
                    let ty = Type::from_int(var.ty, -(value.val() as i64) as f64);
        
                    return Some(Assign::new(self.inner2.to_owned(), ty));
                }
            }
        }

        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
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

        block.push_ir(Neg::new(IROperand::Var(var), out.clone()));

        out
    }
}