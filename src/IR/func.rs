use std::collections::VecDeque;

use super::block::BlockId;
use super::Block;
use super::TypeMetadata;
use super::Var;
use super::VerifyError;
use crate::prelude::PassManager;
use crate::Obj::Linkage;
use crate::Support::ColorClass;
use crate::Support::ColorProfile;

/// Stores the function type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    /// The function arguments (stored as: num, type)
    pub args: Vec<(String, TypeMetadata)>,
    /// The return type
    pub ret: TypeMetadata,
    /// After the given arguments any argument type can be supplied (like the printf function - is in c ...)
    pub any_args: bool,
}

impl FunctionType {
    /// Creates a new function type
    pub fn new(args: Vec<(String, TypeMetadata)>, ret: TypeMetadata) -> Self {
        Self {
            args: args,
            ret: ret,
            any_args: false,
        }
    }

    /// Activates dynamic arguments
    /// Makes that you can supply any argument after the fixed given arguments
    pub fn activate_dynamic_arguments(&mut self) {
        self.any_args = true;
    }

    /// Returns the argument as a var
    /// If the num doesn't exists, it panics
    pub fn arg(&self, num: usize) -> Var {
        let mut index = 0;
        for (name, meta) in &self.args {
            if index == num {
                return Var { name: name.to_owned(), ty: *meta }
            }

            index += 1;
        }

        panic!("the func has {} args but args {} is accesed", self.args.len(), num)
    }
}

/// A ir function with a known variable and arg size and count
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// The function type
    pub ty: FunctionType,

    pub(crate) name: String,
    
    pub(crate) linkage: Linkage,
    pub(crate) blocks: VecDeque<Block>,
}

impl Function {
    /// Creates a new Function
    pub fn new(name: String, ty: FunctionType) -> Self {
        Self {
            ty: ty,

            blocks: VecDeque::new(),

            name: name,

            linkage: Linkage::Internal,
        }
    }

    /// Sets that the function is externally visible (same as: `extern "C"`)
    pub fn extrn(&mut self) {
        self.linkage = Linkage::External;
    }

    /// Sets that the function is imported from another object file (same as: `extern "C" fn abc(i32, i32) -> i32;`)
    pub fn import(&mut self) {
        self.linkage = Linkage::Extern;
    }

    /// Sets that the function is only internally visible (same as a normal function)
    pub fn private(&mut self) {
        self.linkage = Linkage::Internal;
    }

    /// Adds a new block to the function
    pub fn addBlock(&mut self, name: &str) -> BlockId {
        self.blocks.push_back(Block::new(name, &self));
        BlockId(name.to_owned())
    }

    /// Emits the Ir of the function into a string
    pub fn dump(&self) -> String {
        if self.linkage == Linkage::Extern {
            let string = format!("declare {} @{}({})\n",
                self.ty.ret,
                self.name, {
                    let mut fmt = String::new();
        
                    for index in 0..self.ty.args.len() {
                        let arg = self.ty.arg(index);
                        fmt += &format!("{} {}, ", arg.ty, arg.name);
                    }

                    if self.ty.args.len() > 0 {
                        fmt.remove(fmt.chars().count() - 1); // The last space
                        fmt.remove(fmt.chars().count() - 1); // The last comma
                    }
        
                    fmt
                });
            return string;
        }

        let mut string = String::new();

        string += &format!("define {} {} @{}({}) {{\n", self.linkage, self.ty.ret, self.name, {
            let mut fmt = String::new();

            for index in 0..self.ty.args.len() {
                let arg = self.ty.arg(index);
                fmt += &format!("{} {}, ", arg.ty, arg.name);
            }

            if self.ty.args.len() > 0 {
                fmt.remove(fmt.chars().count() - 1); // The last space
                fmt.remove(fmt.chars().count() - 1); // The last comma
            }

            fmt
        });

        for block in &self.blocks {
            string += &format!("{}\n", block.dump());
        }

        string += "}";

        string
    }

    /// Emits the Ir of the function into an colored string
    pub fn dumpColored(&self, profile: ColorProfile) -> String {
        if self.linkage == Linkage::Extern {
            let string = format!("{} {} {} @{}( {})\n",
                profile.markup("declare", ColorClass::Instr),
                profile.markup(&format!("{}", self.linkage), ColorClass::Ty),
                profile.markup(&self.ty.ret.to_string(), ColorClass::Ty),
                profile.markup(&self.name, ColorClass::Name), {
                    let mut fmt = String::new();
        
                    for index in 0..self.ty.args.len() {
                        let arg = self.ty.arg(index);
                        fmt += &format!("{} {}, ", 
                                profile.markup(&arg.ty.to_string(), ColorClass::Ty),
                                profile.markup(&format!("{}", arg.name), ColorClass::Var)
                            );
                    }

                    if self.ty.args.len() != 0 {
                        fmt.remove(fmt.len() - 2); // The last comma
                    }
        
                    fmt
                });
            return string;
        }

        let mut string = String::new();

        string += &format!("{} {} @{}({}) {{\n", 
                        profile.markup("define", ColorClass::Instr),
                        profile.markup(&self.ty.ret.to_string(), ColorClass::Ty), 
                        profile.markup(&self.name, ColorClass::Name), {
            let mut fmt = String::new();

            for index in 0..self.ty.args.len() {
                let arg = self.ty.arg(index);
                fmt += &format!("{} {}, ", 
                        profile.markup(&arg.ty.to_string(), ColorClass::Ty),
                        profile.markup(&format!("{}", arg.name), ColorClass::Var)
                    );
            }
            
            if self.ty.args.len() != 0 {
                fmt.remove(fmt.len() - 2); // The last comma
            }

            fmt
        });

        for block in &self.blocks {
            string += &format!(" {}\n", block.dumpColored(profile));
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
    
    /// Runs the pass manager over all blocks
    pub fn runPassMngr(&mut self, mngr: &PassManager) {
        for block in &mut self.blocks {
            mngr.run(block);
        }
    }

    /// Constructs an id for the function
    pub fn id(&self) -> FuncId {
        FuncId {
            name: self.name.to_owned(),
            ty: self.ty.to_owned(),
        }
    }

    /// Returns the given block for the given id
    pub(crate) fn get_block_for(&self, id: &BlockId) -> Option<&Block> {
        for block in &self.blocks {
            if &block.id() == id {
                return Some(block);
            }
        }

        None
    }
}

/// Creates a new function type
pub fn FnTy(args: Vec<TypeMetadata>, ret: TypeMetadata) -> FunctionType {
    let mut processed = Vec::new();
    let mut index = 0;

    for arg in args {
        processed.push((format!("%{index}"), arg));
        index += 1;
    }

    FunctionType::new(processed, ret)
}

/// Creates a new Function
pub fn Func(name: String, ty: FunctionType) -> Function {
    Function::new(name, ty)
}

/// A function id (describes arguments and the name)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncId {
    pub(crate) name: String,
    pub(crate) ty: FunctionType,
}