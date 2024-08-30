use std::{collections::HashMap, error::Error, fmt::Display};

use crate::CodeGen::{MachineInstr, MachineMnemonic};

/// Stores allowed instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteList {
    instrs: HashMap<String, AllowmentOption>,
}

impl WhiteList {
    /// Creates a new instruction whitelist
    pub fn new() -> Self {
        Self {
            instrs: HashMap::new()
        }
    }

    /// Allowes a specifc mnemonic
    pub fn allow(&mut self, mnemonic: MachineMnemonic) {
        if let Some(option) = self.instrs.get_mut(&mnemonic.name()) {
            *option = AllowmentOption::Allowed;
        } else {
            self.instrs.insert(mnemonic.name(), AllowmentOption::Allowed);
        }
    }

    /// Forbids a specfic mnemonic
    pub fn forbid(&mut self, mnemonic: MachineMnemonic) {
        if let Some(option) = self.instrs.get_mut(&mnemonic.name()) {
            *option = AllowmentOption::NotAllowed;
        } else {
            self.instrs.insert(mnemonic.name(), AllowmentOption::NotAllowed);
        }
    }

    /// Checks if the mnemonic is allowed
    pub fn is_allowed(&self, mnemonic: MachineMnemonic) -> AllowmentOption {
        if let Some(option) = self.instrs.get(&mnemonic.name()) {
            *option
        } else {
            AllowmentOption::Unknown
        }
    }

    /// Checks for forbidden mnemonics
    pub fn check_for_forbidden_mnemonics(&self, vec: &Vec<MachineInstr>) -> Result<(), WhiteListError> {
        for instr in vec {
            if self.is_allowed(instr.mnemonic.clone()) == AllowmentOption::NotAllowed {
                Err(WhiteListError::NotAllowed(instr.mnemonic.clone()))?
            }
        }

        Ok(())
    }
}

/// how strong allowed the object is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AllowmentOption {
    Allowed,
    NotAllowed,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum WhiteListError {
    NotAllowed(MachineMnemonic)
}

impl Display for WhiteListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WhiteListError::NotAllowed(mne) => format!("the instruction {} is not allowed but was suppyled", mne),
        })
    }
}

impl Error for WhiteListError {}