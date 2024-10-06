use crate::Support::ColorClass;
use crate::IR::{Block, Function, TypeMetadata, Var};

use super::{Phi, Ir};

impl Ir for Phi {
    fn dump(&self) -> String {
        let mut fmt_recis = String::from("[");

        for (block, var) in &self.recive_from_blocks {
            fmt_recis.push_str(&format!(" {}, {} ", var.name, block.name));
        }

        fmt_recis.push(']');
        format!("{} = phi {} {}", self.out.name, self.typ, fmt_recis)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let mut fmt_recis = String::from("[");

        for (block, var) in &self.recive_from_blocks {
            fmt_recis.push_str(&format!(" {}, {} ", 
                profile.markup(&var.name, ColorClass::Var),
                profile.markup(&block.name, ColorClass::Name),
            ));
        }

        fmt_recis.push(']');
        format!("{} = {} {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("phi", ColorClass::Instr),
            profile.markup(&self.typ.to_string(), ColorClass::Ty),
            fmt_recis,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, _registry: &mut crate::Target::TargetBackendDescr) {
        // phi mostly influences the register allocator
        // so we don't need to call any `compile_phi` func

        // registry.compile_phi(self)
    }

    fn compile_dir(&self, _compiler: &mut crate::CodeGen::IrCodeGenHelper, _: &crate::prelude::Block) {
        // phi mostly influences the register allocator
        // so we don't need to call any `compile_phi` func

        // compiler.compile_phi(self)
    }

    fn maybe_inline(&self, _: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }

    fn inputs(&self) -> Vec<crate::prelude::Var> {
        let mut inputs = Vec::new();

        for (_, reciv) in &self.recive_from_blocks {
            inputs.push( reciv.to_owned() );
        }

        inputs
    }

    fn output(&self) -> Option<crate::prelude::Var> {
        Some(self.out.to_owned())
    }
}

impl Function {
    /// Builds the phi node which recives variables from different blocks
    pub fn BuildPhi(&mut self, typ: TypeMetadata, recipients: Vec<(&Block, Var)>) -> Var {
        let block = self.blocks.back_mut().expect("expected valid current block.\nConsider creating one");
        
        let mut owned_recipients = Vec::new();

        for (block, var) in recipients {
            let owned = block.clone();
            owned_recipients.push((owned, var.to_owned()));
        }

        let out = Var::new(block, typ);
        block.push_ir( Box::new(
            Phi::new(out.clone(), owned_recipients, typ) 
        ));

        out
    }
}