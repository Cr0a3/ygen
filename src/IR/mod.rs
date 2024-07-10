mod module;
mod func;
mod typ;
mod builder;
mod block;
/// Stores all ir nodes and the ir trait
pub mod ir;

pub use module::Module;
pub use func::{Function, FunctionType};
pub use typ::Type;
pub use typ::TypeMetadata;
pub use builder::IRBuilder;
pub use block::Block;