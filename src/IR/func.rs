use super::Builder;
use super::Type;
use super::TypeMetadata;
use crate::prelude::Ir::*;
use crate::Support::Colorize;

/// A ir function with a known variable and arg size and count
#[derive(Debug, Clone)]
pub struct Function {
    #[allow(unused)]
    pub(crate) builder: Builder,

    pub(crate) args: Vec<(String, TypeMetadata)>,
    pub(crate) vars: Vec<(String, TypeMetadata)>,
    
    pub(crate) ret: TypeMetadata,
    
    pub(crate) name: String,
    
    pub(crate) inline: bool,
    
    pub(crate) ir: Vec<Box<dyn Ir>>,
}

impl Function {
    /// Creates an new Function
    pub fn new(builder: Builder, name: String) -> Self {
        Self {
            builder: builder,

            args: vec![],
            vars: vec![],

            ir: vec![],

            ret: TypeMetadata::Void,

            name: name,
            inline: false,
        }
    }

    /// Makes the function inline
    pub fn inline(&mut self) {
        self.inline = true;
    }

    /// Sets the function args (name: String, type: TypeMetadata)
    pub fn args(&mut self, args: Vec<(String, TypeMetadata)>) {
        self.args = args;
    }

    /// Sets the function vars (name: String, type: TypeMetadata)
    pub fn vars(&mut self, vars: Vec<(String, TypeMetadata)>) {
        self.vars = vars;
    }
    
    /// Sets the function return type
    pub fn ret(&mut self, ret: TypeMetadata) {
        self.ret = ret;
    }

    /// Adds a new Ir node to the function
    pub fn push(&mut self, ir: Box<dyn Ir>) {
        self.ir.push(ir);
    }

    /// Emits the Ir of the function into a string
    pub fn emitToString(&self) -> String {
        let mut string = String::new();

        string += &format!("define {} @{}({}) {{\n", self.ret, self.name, {
            let mut fmt = String::new();

            for (name, metadata) in &self.args {
                fmt += &format!("{} %{},", metadata, name);
            }

            fmt
        });

        for instr in &self.ir {
            string += &format!("    {}\n", instr.text_rep());
        }

        string += "}";

        string
    }

    /// Emits the Ir of the function into an colored string
    pub fn emitToColoredString(&self) -> String {
        let mut string = String::new();

        string += &format!("{} {} @{}({}) {{\n", "define".blue(), self.ret.to_string().green(), self.name.cyan(), {
            let mut fmt = String::new();

            for (name, metadata) in &self.args {
                fmt += &format!("{} {},", metadata.to_string().blue(), format!("{}", name).green());
            }

            fmt
        });

        for instr in &self.ir {
            string += &format!("    {}\n", instr.text_rep_colored());
        }

        string += "}";

        string
    }

    /// Builds an return
    pub fn BuildReturn(&mut self, val: Type) {
        self.ir.push( Return::new( val) );
    }
}
