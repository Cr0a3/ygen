use std::fmt::{Display, Debug};

/// A register
pub trait Reg: Display + ToString + Debug {
    /// sub64 variant (e.g: eax -> rax (x64) or x0 -> x0 (aarch64))
    fn sub64(&self) -> String;
    /// sub32 variant (e.g: rax -> eax (x64) or x0 -> x0 (aarch64))
    fn sub32(&self) -> String;
    /// sub16 variant (e.g: rax -> ax (x64) or x0 -> x0 (aarch64))
    fn sub16(&self) -> String;
    /// sub8 variant (e.g: rax -> al (x64) or x0 -> x0 (aarch64))
    fn sub8(&self) -> String;

    /// is general pourpus 64 bit wide
    fn is_gr64(&self) -> bool;
    /// is general pourpus 32 bit wide
    fn is_gr32(&self) -> bool;
    /// is general pourpus 16 bit wide
    fn is_gr16(&self) -> bool;
    #[allow(dead_code)]
    /// is general pourpus 8 bit wide
    fn is_gr8(&self) -> bool;

    /// return self as Box<dyn Reg>
    fn boxed(&self) -> Box<dyn Reg>;

    /// encodes the register
    fn enc(&self) -> u8;

    /// parses the string variant
    fn from(&self, string: String) -> Box<dyn Reg>;
}

impl PartialEq for Box<dyn Reg> {
    fn eq(&self, other: &Self) -> bool {
        other.sub64() == self.sub64()
    }
}

impl Eq for Box<dyn Reg> {}

impl Clone for Box<dyn Reg> {
    fn clone(&self) -> Self {
        self.boxed()
    }
}