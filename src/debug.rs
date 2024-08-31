use std::path::Path;

use gimli::DwLang;

/// The debugging location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugLocation {
    /// the line which the debug location is refering to
    pub line: u64,
    /// the coloumn which the debug location is refering to  
    pub col: u64,
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

    pub(crate) locs: Vec<DebugLocation>,
    pub(crate) vars: Vec<DebugVariable>,

    pub(crate) producer: String,
    pub(crate) lang: DwLang
}

impl DebugRegistry {
    /// Creates a new debug registry
    pub fn new(producer: String, lang: DwLang, infile: &Path) -> Self {
        let file = infile.file_name().expect("expected filename").to_str().unwrap().to_string();
        let mut tmp = format!("{}", file).chars().rev().collect::<String>();
        tmp.remove(file.chars().count());
        Self {
            file: file,
            dir: tmp.chars().rev().collect::<String>(),
            locs: vec![],
            vars: vec![],

            lang: lang,

            producer: producer,

            current_id: 0,
        }
    }
}