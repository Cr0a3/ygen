use std::sync::Mutex;

use once_cell::sync::OnceCell;

use super::Arch;

/// The Target Registry: stores if a target was already initialized
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRegistry {
    pub(crate) inited_targets: Vec<Arch>,
}

pub(crate) static TARGETS: OnceCell<Mutex<TargetRegistry>> = OnceCell::new();

impl TargetRegistry {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            inited_targets: vec![],
        }
    }

    /// Sets an architecture as initialized
    pub fn set_inited(&mut self, arch: Arch) {
        if !self.inited_targets.contains(&arch) {
            self.inited_targets.push(arch);
        }
    }
}