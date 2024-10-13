use crate::IR::func::FuncId;

use super::*;

impl Ir for Call<FuncId, Vec<Var>, Var> {
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
                if matches!(args.get(index), Some(argty) if *argty != (*arg).ty.into()) {
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

    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_call(&self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        let mut uses = false;

        for arg in &self.inner2 {
            if arg.name == var.name {
                uses = true;
            }
        }

        uses
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_call(&self, &block, module)
    }
    
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn inputs(&self) -> Vec<Var> {
        self.inner2.to_owned()
    }
    
    fn output(&self) -> Option<Var> {
        None // yes it has an output but it will get optimized away, so i just say "it has no output"
    }
}



/// Trait for the call instruction
/// Used for overloading the BuildCall function
pub trait BuildCall<T, U> {
    /// builds a function call
    fn BuildCall(&mut self, func: T, args: U) -> Var;
}
impl BuildCall<&FuncId, Vec<Var>> for Function {
    fn BuildCall(&mut self, func: &FuncId, args: Vec<Var>) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, func.ty.ret);

        block.push_ir(Call::new(func.clone(), args, out.clone()));

        out 
    }
}
