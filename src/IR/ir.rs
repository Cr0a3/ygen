use std::fmt::Debug;
use crate::IR::TypeMetadata;

use super::Type;

/*macro_rules! IrTypeWith2 {
    ($name:tt, $param1:tt, $param2:tt) => {
        #[derive(Debug, Clone)]
        pub struct $name<$param1, $param2> {
            pub inner1: $param1,
            pub inner2: $param2,
        }


        impl<$param1, $param2> $name<$param1, $param2> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                    }
                )
            }
        }
    };
}*/
macro_rules! IrTypeWith1 {
    ($name:tt, $param1:tt) => {
        /// An Ir node
        #[derive(Debug, Clone)]
        pub struct $name<$param1> {
            /// inner value
            pub inner1: $param1,
        }


        impl<$param1> $name<$param1> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                    }
                )
            }
        }
    };
}

IrTypeWith1!(Return, T);

use crate::Support::Colorize;

impl Ir for Return<Type> {
    fn name(&self) -> String {
        format!("ret {}", self.inner1)
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn text_rep(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("ret {} {}", metadata, self.inner1.val())
    }

    fn text_rep_colored(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("{} {} {}", "ret".blue(), metadata.to_string().green(), self.inner1.val().to_string().blue())
    }
}

/// The ir trait
pub trait Ir: Debug {
    /// Returns the name of the ir node type
    fn name(&self) -> String {
        String::from("undefined")
    }

    /// Returns the ir node as his textual representation
    fn text_rep(&self) -> String;
    /// Returns the ir node as his textual representation with colors
    fn text_rep_colored(&self) -> String;

    /// Clones the node into a box of `Box<dyn Ir>`
    fn clone_box(&self) -> Box<dyn Ir>;
}

impl Clone for Box<dyn Ir> {
    fn clone(&self) -> Box<dyn Ir> {
        self.clone_box()
    }
}
