use super::Function;
use std::collections::HashMap;

/// ## The Builder
/// The main class for handeling functions
#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) funcs: HashMap<String, Function>,
}

impl Builder {
    /// Creates a new Builder
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    /// Adds a new function to the builder
    pub fn add(&mut self, name: &str) -> &mut Function {
        self.funcs
            .insert(name.to_string(), Function::new(self.clone(), name.to_string()));
        self.funcs.get_mut(name).unwrap()
    }

    #[allow(dead_code)]
    /// Returns a read only reference to the given function name
    /// ### Used for passes
    pub(crate) fn getFunc(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    #[allow(dead_code)]
    /// Returns a mutable reference to the given function name
    /// ### Used for passes
    pub(crate) fn getMutFunc(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.get_mut(name)
    }

    /// Emits the ir of the entire builder into one string
    /// Maybe save to an file
    pub fn emitToString(&self) -> String {
        let mut string = String::new();

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.emitToString());
        }

        string
    }
    
    /// Emits the ir of the entire builder into a colored string
    /// Maybe output to stdout
    pub fn emitToColoredString(&self) -> String {
        let mut string = String::new();

        for (_, func) in &self.funcs {
            string += &format!("{}\n", func.emitToColoredString());
        }

        string
    }
}
