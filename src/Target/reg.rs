use std::fmt::{Display, Debug};

pub(crate) trait Reg: Display + ToString + Debug {
    fn sub64(&self) -> String;
    fn sub32(&self) -> String;
    fn sub16(&self) -> String;
    fn sub8(&self) -> String;

    fn is_gr64(&self) -> bool;
    fn is_gr32(&self) -> bool;
    fn is_gr16(&self) -> bool;
    #[allow(dead_code)]
    fn is_gr8(&self) -> bool;

    fn boxed(&self) -> Box<dyn Reg>;

    fn enc(&self) -> u8;

    fn from(&self, string: String) -> Box<dyn Reg>;
}

impl PartialEq for dyn Reg {
    fn eq(&self, other: &Self) -> bool {
        other.sub64() == self.sub64()
    }
}