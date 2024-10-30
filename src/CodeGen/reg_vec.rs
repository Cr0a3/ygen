use std::collections::HashMap;

use crate::Target::Arch;
use super::reg::Reg;

/// A register vector.
/// Used for getting platform specific registers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegVec {
    regs: HashMap<Arch, Vec<Reg>>,
}

impl RegVec {
    /// Creates a new register vector
    pub fn new() -> Self {
        Self {
            regs: HashMap::new(),
        }
    }

    /// pushes a specific register back to the register of the arch
    pub fn push(&mut self, arch: Arch, reg: Reg) {
        if let Some(entry) = self.regs.get_mut(&arch) {
            entry.push(reg);
        } else {
            self.regs.insert(arch, vec![reg]);
        }
    }

    /// pops a specific register from the register of the arch
    pub fn pop(&mut self, arch: Arch) -> Option<Reg> {
        if let Some(entry) = self.regs.get_mut(&arch) {
            entry.pop()
        } else { None }
    }

    /// reversees the registers of the arch
    pub fn reverse(&mut self, arch: Arch) {
        if let Some(entry) = self.regs.get_mut(&arch) {
            entry.reverse();
        }
    }

    /*pub(crate) fn inner(&mut self, arch: Arch) -> &mut Vec<Reg> {
        let keys = self.regs.clone();
        let keys = keys.keys();

        if let Some(entry) = self.regs.get_mut(&arch) {
            entry
        } else {
            panic!("unkown entry: {:?} (known entrys: {:?})", arch, keys)
        }
    }*/
}
