use super::*;

impl Ir for VecInsert {
    fn dump(&self) -> String {
        format!("{} = vec_insert {} {}, {}, {}", self.out.name, self.vec.ty, self.vec.name, self.elem, self.position)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {}, {}, {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("vec_insert", ColorClass::Instr),
            profile.markup(&self.vec.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.vec.name.to_string(), ColorClass::Var),
            profile.markup(&self.elem.to_string(), ColorClass::Value),
            profile.markup(&self.position.to_string(), ColorClass::Value),
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

    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();

        inputs.push(&mut self.vec);

        if let IROperand::Var(inner2) = &mut self.elem { inputs.push(inner2); }

        inputs
    }

    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();

        inputs.push(self.vec.to_owned());

        if let IROperand::Var(inner2) = &self.elem { inputs.push(inner2.to_owned()); }

        inputs
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.to_owned()) 
    }

    fn ty(&self) -> Option<TypeMetadata> {
        Some(self.out.ty)
    }
}

impl EvalOptVisitor for VecInsert {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let IROperand::Var(elem) = &self.elem {
            if let Some(elem) = const_values.get(&elem.name) {
                return Some(Box::new(VecInsert {
                    out: self.out.to_owned(),
                    vec: self.vec.to_owned(),
                    elem: IROperand::Type(*elem),
                    position: self.position,
                }));
            }
        }

        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}