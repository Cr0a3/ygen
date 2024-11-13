use std::path::PathBuf;

use crate::{Support::ColorClass, IR::Function};

use super::{EvalOptVisitor, Ir, IsNode};

/// A node which startes a debugging line programm
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugNode {
    /// the code line
    pub line: i64,
    /// the code coloumn
    pub coloumn: i64,
    /// the file path
    pub file: PathBuf,
}

impl Ir for DebugNode {
    fn dump(&self) -> String {
        format!("!dbg {}:{} in ^{}", self.line, self.coloumn, self.file.to_str().unwrap())
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{}: {}:{} {} ^{}", 
            profile.markup("!dbg", ColorClass::Instr),
            profile.markup(&self.line.to_string(), ColorClass::Value), 
            profile.markup(&self.coloumn.to_string(), ColorClass::Value), 
            profile.markup("in", ColorClass::Instr),
            profile.markup(&self.file.to_str().unwrap(), ColorClass::Value), 
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

    fn inputs(&self) -> Vec<crate::prelude::Var> {
        vec![]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut crate::prelude::Var> {
        vec![]
    }

    fn output(&self) -> Option<crate::prelude::Var> {
        None
    }
}

impl EvalOptVisitor for DebugNode {
    fn maybe_inline(&self, _: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl IsNode for DebugNode {
    fn is_debug(&self) -> bool {
        true
    }
}

impl Function {
    /// Sets the source location for debugging (all of the ir nodes will respond to the location till an new location is set)
    pub fn BuildDebug(&mut self, line: i64, coloumn: i64, file: PathBuf) {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
       
        block.push_ir( Box::new( DebugNode { 
            line: line, 
            coloumn: coloumn, 
            file: file 
        } ));
    }
}