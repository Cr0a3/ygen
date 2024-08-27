use object::write::{Relocation, SectionId, Symbol, SymbolId, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationFlags, RelocationKind, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

use crate::prelude::Triple;
use crate::Target::{self, Arch, CallConv};
use std::collections::BTreeMap;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ObjectError {
    UnsupportedArch(Arch),
    DefWithoutDecl(String),
}

impl std::fmt::Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ObjectError::UnsupportedArch(arch) => format!("unsupported architecture for writing to object files: {:?}", arch),
            ObjectError::DefWithoutDecl(name) => format!("definition without an corresponding decleration: {}", name),
        })
    }
}

impl std::error::Error for ObjectError {}

/// A decl to say what's the label/func
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decl {
    /// A function
    Function,
    /// A mutable data (E.g: a global variable)
    Data,
    /// A constant data
    Constant,
}

/// Links from one symbol to another
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The link source
    pub from: String,
    /// The link destination
    pub to: String,
    /// The binary offset of the start of the function
    pub at: usize,
    /// The addend to use
    pub addend: i64,
}

/// The linkage of the target symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Linkage {
    /// Can be seen outside and inside of object file
    External,
    /// From another object file
    Extern,
    /// Only aviable in the object file
    Internal,
}

/// Builds object files.
/// It also supports debugging information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectBuilder {
    defines: BTreeMap<String, Vec<u8>>,
    links: Vec<Link>,

    decls: Vec<(String, Decl, Linkage)>,

    triple: Triple
}

impl ObjectBuilder {
    /// Creates an new object builder
    pub fn new(triple: Triple) -> Self {
        Self {
            defines: BTreeMap::new(),

            links: vec![],

            decls: vec![],

            triple: triple
        }
    }

    /// Sets the decls of the function
    pub fn decls(&mut self, decls: Vec<(&str, Decl, Linkage)>) {
        for decl in decls {
            self.decl(decl);
        }
    }

    /// Adds one decl to the function
    pub fn decl(&mut self, decl: (&str, Decl, Linkage)) {
        self.decls.push((decl.0.to_string(), decl.1, decl.2));
    }

    /// Defines a symbol
    pub fn define(&mut self, name: &str, data: Vec<u8>) {
        self.defines.insert(name.to_string(), data);
    }

    /// Links from one symbol to another
    pub fn link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Writes the object file into the the specified file
    pub fn emit(&self, file: File) -> Result<(), Box<dyn Error>> {

        let align = 1;

        let mut obj = object::write::Object::new({
            match self.triple.bin {
                Target::ObjFormat::Unknown => BinaryFormat::native_object(),
                Target::ObjFormat::Coff => BinaryFormat::Coff,
                Target::ObjFormat::Elf => BinaryFormat::Elf,
                Target::ObjFormat::MachO => BinaryFormat::MachO,
                Target::ObjFormat::Wasm => BinaryFormat::Wasm,
                Target::ObjFormat::XCoff => BinaryFormat::Xcoff,
                Target::ObjFormat::Default => BinaryFormat::native_object(),
            }
        }, {
            match self.triple.arch {
                Target::Arch::Arm => Architecture::Arm,
                Target::Arch::Aarch64 => Architecture::Aarch64,
                Target::Arch::Avr => Architecture::Avr,
                Target::Arch::Bpfel => Architecture::Bpf,
                Target::Arch::Bpfeb => Architecture::Bpf,
                Target::Arch::Hexagon => Architecture::Hexagon,
                Target::Arch::Mips => Architecture::Mips,
                Target::Arch::Mips64 => Architecture::Mips64,
                Target::Arch::Msp420 => Architecture::Msp430,
                Target::Arch::Ppc => Architecture::PowerPc,
                Target::Arch::Ppc64 => Architecture::PowerPc64,
                Target::Arch::Riscv32 => Architecture::Riscv32,
                Target::Arch::Riscv64 => Architecture::Riscv64,
                Target::Arch::Sparc => Architecture::Sparc,
                Target::Arch::X86 => Architecture::X86_64_X32,
                Target::Arch::X86_64 => Architecture::X86_64,
                Target::Arch::Wasm32 => Architecture::Wasm32,
                Target::Arch::Wasm64 => Architecture::Wasm64,
                other => Err( ObjectError::UnsupportedArch(other) )?,
            }
        }, {
            match self.triple.arch {
                Target::Arch::Arm => Endianness::Little,    
                Target::Arch::Aarch64 => Endianness::Little,
                Target::Arch::Avr => Endianness::Little,
                Target::Arch::Bpfel => Endianness::Little,
                Target::Arch::Bpfeb => Endianness::Big,
                Target::Arch::Hexagon => Endianness::Little,
                Target::Arch::Mips => Endianness::Big,
                Target::Arch::Mips64 => Endianness::Big,
                Target::Arch::Msp420 => Endianness::Little,
                Target::Arch::Ppc => Endianness::Big,
                Target::Arch::Ppc64 => Endianness::Big,
                Target::Arch::Riscv32 => Endianness::Little,
                Target::Arch::Riscv64 => Endianness::Little,
                Target::Arch::Sparc => Endianness::Big,
                Target::Arch::X86 => Endianness::Little,
                Target::Arch::X86_64 =>  Endianness::Little,
                Target::Arch::Wasm32 =>  Endianness::Little,
                Target::Arch::Wasm64 =>  Endianness::Little,
                _ => unreachable!(), // cannot panic cuz the archs are filtered out by the prefius call
            }
        });

        let secText = obj.add_section(vec![], ".text".as_bytes().to_vec(), SectionKind::Text);
        let secData = obj.add_section(vec![], ".data".as_bytes().to_vec(), SectionKind::Data);
        let secConsts = obj.add_section(vec![], ".rodata".as_bytes().to_vec(), SectionKind::ReadOnlyData);

        let mut syms: BTreeMap<String, (Option<SectionId>, Option</*offsest*/u64>, SymbolId, Decl)> = BTreeMap::new();

        for (name, data) in &self.defines {
            let name = name.to_owned();
            let data = data.to_owned();

            let mut decl = None;
            let mut link = None;

            for (declName, declDecl, declLink) in &self.decls {
                if *declName == name {
                    decl = Some(declDecl); 
                    link = Some(declLink);
                }
            }

            if decl == None {
                Err( ObjectError::DefWithoutDecl(name.clone()) )?
            }
            let decl = decl.unwrap();
            let link = link.unwrap();

            let sym = obj.add_symbol(Symbol {
                name: name.clone().as_bytes().to_vec(),
                value: 0,
                size: {
                    if data.len() > 1 {
                        (data.len() - 1) as u64
                    } else {
                        0
                    }
                },
                kind: {
                    match decl {
                        Decl::Function => SymbolKind::Text,
                        Decl::Data => SymbolKind::Data,
                        Decl::Constant => SymbolKind::Label,
                    }
                },
                scope: {
                    match link {
                        Linkage::Extern => SymbolScope::Dynamic,
                        Linkage::External => SymbolScope::Linkage,
                        Linkage::Internal => SymbolScope::Compilation,
                    }
                },
                weak: false,
                section: {
                    if *link != Linkage::Extern {
                        match decl {
                            Decl::Function => SymbolSection::Section(secText),
                            Decl::Data => SymbolSection::Section(secData),
                            Decl::Constant => SymbolSection::Section(secConsts),
                        }
                    } else {
                        SymbolSection::Undefined
                    }
                },
                flags: SymbolFlags::None,
            });

            /*let def_section = match decl {
                Decl::Function => obj.add_subsection(StandardSection::Text, name.as_bytes()),
                Decl::Data => obj.add_subsection(StandardSection::Data, name.as_bytes()),
                Decl::Constant => obj.add_subsection(StandardSection::ReadOnlyData, name.as_bytes()),
            };*/


            if *link != Linkage::Extern {
                let def_offset = match decl {
                    Decl::Function => obj.add_symbol_data(sym, secText, &data, align),
                    Decl::Data => obj.add_symbol_data(sym, secData, &data, align),
                    Decl::Constant => obj.add_symbol_data(sym, secConsts, &data, align),
                };
    
                syms.insert(name.clone(), (None, Some(def_offset), sym, *decl));
            } else {
                syms.insert(name.clone(), (None, None, sym, *decl));
            }
        }

        for link in &self.links {
            let (_, off, _, _) = syms.get(&link.from).unwrap();
            let (_, _, to_sym, ty) = syms.get(&link.to).unwrap();

            let mut addend = 0;
            let mut offset = 0;

            if self.triple.getCallConv() == Ok(CallConv::WindowsFastCall) {
                addend = match ty {
                    Decl::Function => 0,
                    _ => -1,
                };
                offset = -4;
            } else if self.triple.getCallConv() == Ok(CallConv::SystemV) {
                addend = 0;
                offset = -4;
            }

            obj.add_relocation(secText, Relocation {
                offset: (link.at as i64 + offset) as u64 + {if let Some(off) = off { *off } else { 0 }},
                symbol: to_sym.to_owned(),
                addend: link.addend + addend,
                flags: RelocationFlags::Generic { 
                    kind: RelocationKind::PltRelative, 
                    encoding: {
                        match &self.triple.arch {
                            Target::Arch::Aarch64 => RelocationEncoding::AArch64Call,
                            Target::Arch::Aarch64BE => RelocationEncoding::AArch64Call,
                            Target::Arch::X86 => RelocationEncoding::X86Branch,
                            Target::Arch::X86_64 =>  RelocationEncoding::X86Branch,
                            _ => RelocationEncoding::Generic,
                        }
                    }, 
                    size: 32, 
                },
            })?;
        }

        obj.write_stream(file)?;

        Ok(())
    }
}