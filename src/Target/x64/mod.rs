//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::sync::Mutex;

use super::{registry::TARGETS, Arch, TargetRegistry};

pub(crate) mod ir;
pub(crate) mod call;

/// Initializes the x86-64 target
pub fn initializeX64Target() {
    TARGETS.get_or_init( || {
        Mutex::new(TargetRegistry::new())
    });

    TARGETS.get().unwrap().lock().unwrap().set_inited(Arch::X86_64);
}