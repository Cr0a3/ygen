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

/// Debugging source languages
#[allow(missing_docs)]
#[allow(non_upper_case_globals)]
pub mod Lang {
    use gimli::{DwLang, constants::*};

    pub const C: DwLang = DW_LANG_C;
    pub const D: DwLang = DW_LANG_D;
    pub const Go: DwLang = DW_LANG_Go;
    pub const C11: DwLang = DW_LANG_C11;
    pub const C17: DwLang = DW_LANG_C17;
    pub const C89: DwLang = DW_LANG_C89;
    pub const C99: DwLang = DW_LANG_C99;
    pub const PLI: DwLang = DW_LANG_PLI;
    pub const UPC: DwLang = DW_LANG_UPC;
    pub const Zig: DwLang = DW_LANG_Zig;
    pub const Java: DwLang = DW_LANG_Java;
    pub const ObjC: DwLang = DW_LANG_ObjC;
    pub const Rust: DwLang = DW_LANG_Rust;
    pub const Ada83: DwLang = DW_LANG_Ada83;
    pub const Ada95: DwLang = DW_LANG_Ada95;
    pub const Bliss: DwLang = DW_LANG_BLISS;
    pub const Dylan: DwLang = DW_LANG_Dylan;
    pub const Julia: DwLang = DW_LANG_Julia;
    pub const OCamel: DwLang = DW_LANG_OCaml;
    pub const Swift: DwLang = DW_LANG_Swift;
    pub const Kotlin: DwLang = DW_LANG_Kotlin;
    pub const OpenCL: DwLang = DW_LANG_OpenCL;
    pub const Python: DwLang = DW_LANG_Python;
    pub const Ada2005: DwLang = DW_LANG_Ada2005;
    pub const Ada2012: DwLang = DW_LANG_Ada2012;
    pub const Cobol74: DwLang = DW_LANG_Cobol74;
    pub const Cobol85: DwLang = DW_LANG_Cobol85;
    pub const Crystal: DwLang = DW_LANG_Crystal;
    pub const Haskell: DwLang = DW_LANG_Haskell;
    pub const hi_user: DwLang = DW_LANG_hi_user;
    pub const lo_user: DwLang = DW_LANG_lo_user;
    pub const Modula2: DwLang = DW_LANG_Modula2;
    pub const Pascal83: DwLang = DW_LANG_Pascal83;
    pub const Fortan03: DwLang = DW_LANG_Fortran03;
    pub const Fortan08: DwLang = DW_LANG_Fortran08;
    pub const Fortran18: DwLang = DW_LANG_Fortran18;
    pub const Fortran90: DwLang = DW_LANG_Fortran90;
    pub const Fortran95: DwLang = DW_LANG_Fortran95;
    pub const Cpp: DwLang = DW_LANG_C_plus_plus;
    pub const RenderScript: DwLang = DW_LANG_RenderScript;
    pub const SunAsm: DwLang = DW_LANG_SUN_Assembler;
    pub const BorlandDelphi: DwLang = DW_LANG_BORLAND_Delphi;
    pub const Cpp03: DwLang = DW_LANG_C_plus_plus_03;
    pub const Cpp11: DwLang = DW_LANG_C_plus_plus_11;
    pub const Cpp14: DwLang = DW_LANG_C_plus_plus_14;
    pub const Cpp20: DwLang = DW_LANG_C_plus_plus_20;
    pub const MipsAsm: DwLang = DW_LANG_Mips_Assembler;
    pub const ObjCpp: DwLang = DW_LANG_ObjC_plus_plus;
    pub const AltiumAsm: DwLang = DW_LANG_ALTIUM_Assembler;
    pub const GoogleRenderScript: DwLang = DW_LANG_GOOGLE_RenderScript;
}