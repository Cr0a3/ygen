mod builder;
mod func;
mod typ;
/// Stores all ir nodes and the ir trait
pub mod ir;

pub use builder::Builder;
pub use func::Function;
pub use typ::Type;
pub use typ::TypeMetadata;
pub use ir::Ir;