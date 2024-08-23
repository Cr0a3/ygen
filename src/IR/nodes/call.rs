use super::*;

impl Ir for Call<Function, Vec<Var>, Var> {
    fn dump(&self) -> String {
        let mut fmt = String::new();
        
        for arg in &self.inner2 {
            fmt.push_str(&format!("{} ", arg))
        }

        format!("{} = call {} {} {}", self.inner3.name, self.inner1.ty.ret, self.inner1.name, fmt)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let mut fmt = String::new();
        
        for arg in &self.inner2 {
            fmt.push_str(&arg.to_colored_string(profile));
            fmt.push(' ');
        }

        format!("{} = {} {} {} {}", 
            profile.markup(&self.inner3.name, ColorClass::Var),
            profile.markup("call", ColorClass::Instr),
            profile.markup(&self.inner1.ty.ret.to_string(), ColorClass::Ty),
            profile.markup(&self.inner1.name, ColorClass::Name),
            fmt
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        if self.inner3.ty != self.inner1.ty.ret {
            Err(VerifyError::Op0Op1TyNoMatch(self.inner3.ty, self.inner1.ty.ret))?
        }

        let mut index = 0;
        let args = &self.inner1.ty.args;
        for arg in &self.inner2 {
            if index < args.len() {
                if matches!(args.get(index), Some((_, argty)) if *argty != (*arg).ty.into()) {
                    Err(VerifyError::InvalidArgumentTypeFound)?
                }
            } else {
                if !self.inner1.ty.any_args {
                    Err(VerifyError::ToManyArgumentsWereSupplyed)?
                }
            }
            
            index += 1;
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForCall()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        let mut uses = false;

        if self.inner3 == *var {
            uses = true;
        }


        for arg in &self.inner2 {
            if *arg == *var {
                uses = true;
            }
        }

        uses
    }
}



/// Trait for the call instruction
/// Used for overloading the BuildCall function
pub trait BuildCall<T, U> {
    /// builds a function call
    fn BuildCall(&mut self, func: T, args: U) -> Var;
}
impl BuildCall<&Function, Vec<Var>> for IRBuilder<'_> {
    fn BuildCall(&mut self, func: &Function, args: Vec<Var>) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, func.ty.ret);

        block.push_ir(Call::new(func.clone(), args, out.clone()));

        out 
    }
}
