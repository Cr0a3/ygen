/// A memory reference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory {
    /// The offset
    pub offset: i32,

    /// is the offset realtive to the stack pointer
    pub sp_relativ: bool,
    /// is the offset realtive to the base pointer
    pub fp_relativ: bool,

    /// The size of the displacment
    pub size: usize,
}