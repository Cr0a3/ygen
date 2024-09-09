mod wrapper;
mod dll;
mod exe;

pub use wrapper::{
    ObjectBuilder,
    Decl, Link, Linkage,
};
pub use dll::*;
pub use exe::*;