use std::collections::VecDeque;

use super::Block;
use super::TypeMetadata;
use super::VerifyError;
use crate::Support::Colorize;

/// Stores the function type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    /// The function arguments (stored as: num, type)
    pub args: Vec<(/*num*/usize, TypeMetadata)>,
    /// The return type
    pub ret: TypeMetadata,
}

impl FunctionType {
    /// Creates a new function type
    pub fn new(args: Vec<TypeMetadata>, ret: TypeMetadata) -> Self {
        Self {
            args: {
                let mut ret = vec![];

                let mut index = 0;
                for arg in args {

                    ret.push((index, arg));

                    index += 1;
                }
                ret
            },
            ret: ret,
        }
    }
}

/// A ir function with a known variable and arg size and count
#[derive(Debug, Clone)]
pub struct Function {
    /// The function type
    pub ty: FunctionType,
    
    pub(crate) ret: TypeMetadata,
    pub(crate) name: String,
    
    pub(crate) inline: bool,
    pub(crate) blocks: VecDeque<Block>,
}

impl Function {
    /// Creates an new Function
    pub fn new(name: String, ty: FunctionType) -> Self {
        Self {
            ty: ty,
            ret: TypeMetadata::Void,

            blocks: VecDeque::new(),

            name: name,
            inline: false,
        }
    }

    /// Makes the function inline
    pub fn inline(&mut self) {
        self.inline = true;
    }

    /// Adds a new block to the function
    pub fn addBlock(&mut self, name: &str) -> &mut Block {
        self.blocks.push_back(Block::new(name, &self));
        self.blocks.back_mut().expect("unreachable") // we previusly pushed something so here it is safe
    }

    /// Emits the Ir of the function into a string
    pub fn dump(&self) -> String {
        let mut string = String::new();

        string += &format!("define {} @{}({}) {{\n", self.ret, self.name, {
            let mut fmt = String::new();

            for (name, metadata) in &self.ty.args {
                fmt += &format!("{} %{},", metadata, name);
            }

            fmt
        });

        for block in &self.blocks {
            string += &format!(" {}\n", block.dump());
        }

        string += "}";

        string
    }

    /// Emits the Ir of the function into an colored string
    pub fn dumpColored(&self) -> String {
        let mut string = String::new();

        string += &format!("{} {} @{}({}) {{\n", "define".blue(), self.ret.to_string().green(), self.name.cyan(), {
            let mut fmt = String::new();

            for (name, metadata) in &self.ty.args {
                fmt += &format!("{} {},", metadata.to_string().blue(), format!("{}", name).green());
            }

            fmt
        });

        for block in &self.blocks {
            string += &format!(" {}\n", block.dumpColored());
        }

        string += "}";

        string
    }

    /// Verifys if the function and all of its blocks are correct:
    ///  * Checks if the return type is the actual specified return type of the function
    ///  * Checks all ir nodes
    pub fn verify(&self) -> Result<(), VerifyError> {
        for block in &self.blocks {
            block.verify(self)?
        }

        Ok(())
    }
}
