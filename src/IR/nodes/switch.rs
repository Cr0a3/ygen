use std::collections::HashMap;

use crate::{Support::ColorClass, IR::{BlockId, Function, Type, TypeMetadata, Var}};

use super::{Br, Ir};

/// The switch node is used to switch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub(crate) to_switch: Var,
    pub(crate) typ: TypeMetadata,
    pub(crate) cases: HashMap<Type, BlockId>,
    pub(crate) default: BlockId,
}

impl Switch {
    pub(crate) fn new(var: Var, cases: HashMap<Type, BlockId>, default: BlockId) -> Switch {
        Self {
            to_switch: var.to_owned(),
            typ: var.ty,
            cases: cases,
            default: default
        }
    }
}

impl Ir for Switch {
    fn dump(&self) -> String {
        let mut fmt_cases = String::from("[ ");

        for (val, block) in &self.cases {
            let meta: TypeMetadata = (*val).into();

            fmt_cases.push_str(&format!("{} {}, {} ", meta, val.val(), block.name));
        }

        fmt_cases.push(']');
        format!("switch {} {}, default {} {}", self.typ, self.to_switch.name, self.default.name, fmt_cases)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let mut fmt_cases = String::from("[");

        for (val, block) in &self.cases {
            let meta: TypeMetadata = (*val).into();

            fmt_cases.push_str(&format!(" {} {}, {} ", 
                profile.markup(&meta.to_string(), ColorClass::Ty), 
                profile.markup(&val.val().to_string(), ColorClass::Value),
                profile.markup(&block.name, ColorClass::Name)
            ));
        }

        fmt_cases.push(']');
        format!("switch {} {}, default {} {}", 
        profile.markup(&self.typ.to_string(), ColorClass::Ty), 
            profile.markup(&self.to_switch.name, ColorClass::Var), 
            profile.markup(&self.default.name, ColorClass::Name),
            fmt_cases
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_switch(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_switch(self, block, module)
    }

    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.cases.len() == 0 {

            Some(Br::new(self.default.to_owned()))

        } else { None }
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.to_switch.to_owned()]
    }

    fn output(&self) -> Option<Var> {
        None
    }
}

impl Function {
    /// Builds an switch statement
    pub fn BuildSwitch(&mut self, source: Var, default: &BlockId, cases: HashMap<Type, &BlockId>) {
        let block = self.blocks.back_mut().expect("expected current block");

        let mut owned_cases = HashMap::new();

        for case in cases {
            owned_cases.insert(case.0, case.1.to_owned());
        }

        block.push_ir( Box::new( Switch::new(source.to_owned(), owned_cases, default.to_owned()) ));
    }
}