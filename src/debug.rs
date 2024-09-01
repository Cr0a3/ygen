use std::{collections::HashMap, path::Path};

use gimli::DwLang;

/// The debugging location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugLocation {
    /// the line which the debug location is refering to
    pub line: u64,
    /// the coloumn which the debug location is refering to  
    pub col: u64,
    /// If the location is the end of the prolog
    pub epilog: bool,
    /// If the location is the start of the epilog
    pub prolog: bool,
    /// The addres of the location relativ from the start of the assembly code
    pub adr: u64,
}

/// A variable to debug
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugVariable {
    pub(crate) name: String,
}

impl DebugVariable {
    /// Creates a new debug variable
    pub fn new(name: String) -> Self {
        Self {
            name: name
        }
    }
}

/// The debug register is used to store files and their file id
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugRegistry {
    pub(crate) file: String,
    pub(crate) dir: String,
    current_id: u64,

    pub(crate) locs: HashMap<String, Vec<DebugLocation>>,
    pub(crate) vars: HashMap<String, Vec<DebugVariable>>,

    pub(crate) producer: String,
    pub(crate) lang: DwLang
}

impl DebugRegistry {
    /// Creates a new debug registry
    pub fn new(producer: String, lang: DwLang, infile: &Path) -> Self {
        let dir = infile.parent()
                        .map(|p| p.to_str().unwrap_or("").to_string())
                        .unwrap_or_else(|| "".to_string());

        let file = infile.file_name()
                        .map(|f| f.to_str().unwrap_or("").to_string())
                        .unwrap_or_else(|| "".to_string());

        Self {
            file: file,
            dir: dir,
            locs: HashMap::new(),
            vars: HashMap::new(),

            lang: lang,

            producer: producer,

            current_id: 0,
        }
    }

    /// adds a debugging location
    pub fn add_location(&mut self, symbol: &String, location: DebugLocation) {
        if let Some(entry) = self.locs.get_mut(symbol) {
            entry.push(location);
        } else {
            self.locs.insert(symbol.to_string(), vec![location]);
        }
    }
}