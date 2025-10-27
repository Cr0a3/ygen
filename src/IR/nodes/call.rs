use crate::IR::func::FuncId;

use super::*;

impl Ir for Call {
    fn dump(&self) -> String {
        let mut fmt = String::new();
        
        for arg in &self.args {
            fmt.push_str(&format!("{} ", arg))
        }

        format!("{} = call {} {} {}", self.out.name, self.func.ty.ret, self.func.name, fmt)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let mut fmt = String::new();
        
        for arg in &self.args {
            fmt.push_str(&profile.markup(&arg.to_string(), ColorClass::Var));
            fmt.push(' ');
        }

        format!("{} = {} {} {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("call", ColorClass::Instr),
            profile.markup(&self.func.ty.ret.to_string(), ColorClass::Ty),
            profile.markup(&self.func.name, ColorClass::Name),
            fmt
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        if self.out.ty != self.func.ty.ret {
            Err(VerifyError::Op0Op1TyNoMatch(self.out.ty, self.func.ty.ret))?
        }

        let mut index = 0;
        let args = &self.func.ty.args;
        for arg in &self.args {
            if index < args.len() {
                if matches!(args.get(index), Some((_, argty)) if *argty != (*arg).get_ty()) {
                    Err(VerifyError::InvalidArgumentTypeFound)?
                }
            } else {
                if !self.func.ty.any_args {
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

        for arg in &self.args {
            if let IROperand::Var(arg) = &arg {
                if arg.name == var.name {
                    uses = true;
                }
            }
        }

        uses
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_call(&self, &block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();

        for arg in &self.args {
            if let IROperand::Var(arg) = &arg {
                inputs.push(arg.to_owned());
            }
        }

        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();

        for arg in &mut self.args {
            if let IROperand::Var(arg) = arg {
                inputs.push(arg);
            }
        }

        inputs
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.out.to_owned())
    }
}

impl IsNode for Call {
    fn is_call(&self) -> bool {
        true
    }
}

impl Call {
    /// Returns the call target
    pub fn getCallTarget(&self) -> FuncId {
        self.func.to_owned()
    }

    /// Returns the arguments
    pub fn getArgs(&self) -> Vec<IROperand> {
        self.args.to_owned()
    }

    /// Returns the variable which stores the result of the call
    pub fn getOutputVar(&self) -> Var {
        self.out.to_owned()
    }
}

impl EvalOptVisitor for Call {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

/// Trait for the call instruction
/// Used for overloading the BuildCall function
pub trait BuildCall<T, U> {
    /// builds a function call
    fn BuildCall(&mut self, func: T, args: U) -> Var;
}
impl BuildCall<&FuncId, Vec<IROperand>> for Function {
    fn BuildCall(&mut self, func: &FuncId, args: Vec<IROperand>) -> Var {
        let block = self.blocks.get_mut(self.curr_block).expect("invalid current block");
        
        let out = Var::new(block, func.ty.ret);

        block.push_ir(Box::new(Call {
            out: out.to_owned(),
            func: func.clone(),
            args: args,
        }));

        out 
    }
}
