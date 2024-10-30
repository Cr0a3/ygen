use gimli::write::{Address, DwarfUnit, EndianVec, FileId, Range, RelocateWriter};
use gimli::LittleEndian;
use object::write::{Object, Relocation, SectionId, Symbol, SymbolId, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, FileFlags, RelocationEncoding, RelocationFlags, RelocationKind, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

use crate::debug::DebugRegistry;
use crate::prelude::Triple;
use crate::Target::{self, Arch};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::error::Error;
use std::io::Write;

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

#[derive(Clone)]
struct Section {
    data: EndianVec<LittleEndian>,
    relocations: Vec<gimli::write::Relocation>,
    id: Option<object::write::SectionId>,
}

impl Section {
    fn new() -> Self {
        Self {
            data: EndianVec::new(LittleEndian),
            relocations: Vec::new(),
            id: None,
        }
    }
}

impl RelocateWriter for Section {
    type Writer = EndianVec<LittleEndian>;

    fn writer(&self) -> &Self::Writer {
        &self.data
    }

    fn writer_mut(&mut self) -> &mut Self::Writer {
        &mut self.data
    }

    fn relocate(&mut self, relocation: gimli::write::Relocation) {
        self.relocations.push(relocation);
    }
}

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
    /// If it is a special relocation (only internal usage)
    pub special: bool,
    /// the type
    pub kind: RelocationEncoding,
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

impl Display for Linkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Linkage::External => "",
            Linkage::Extern => "extern",
            Linkage::Internal => "local",
        })
    }
}

/// Builds object files.
/// It also supports debugging information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectBuilder {
    pub(crate) defines: BTreeMap<String, Vec<u8>>,
    pub(crate) links: Vec<Link>,

    pub(crate) decls: Vec<(String, Decl, Linkage)>,

    pub(crate) triple: Triple,

    pub(crate) flags: Option<FileFlags>,

    /// include debugging information
    pub debug: bool,

    pub(crate) just_write_bytes: Option<Vec<u8>>,
}

impl ObjectBuilder {
    /// Creates an new object builder
    pub fn new(triple: Triple) -> Self {
        Self {
            defines: BTreeMap::new(),

            links: vec![],
            decls: vec![],

            flags: None,

            triple: triple,

            debug: false,
            just_write_bytes: None,
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
    pub fn emit(&self, mut file: File, debug: Option<DebugRegistry>) -> Result<(), Box<dyn Error>> {
        if let Some(bytes) = &self.just_write_bytes {
            return Ok(file.write_all(&bytes)?);
        }

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

        if let Some(flags) = self.flags {
            obj.flags = flags;
        }

        let secText = obj.add_section(vec![], ".text".as_bytes().to_vec(), SectionKind::Text);
        let secData = obj.add_section(vec![], ".data".as_bytes().to_vec(), SectionKind::Data);
        let secConsts = obj.add_section(vec![], ".rodata".as_bytes().to_vec(), SectionKind::ReadOnlyData);

        let mut syms: BTreeMap<String, (Option<SectionId>, Option</*offsest*/u64>, SymbolId, Decl, /*size*/u64, /*link*/Linkage)> = BTreeMap::new();

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
                        Decl::Constant => SymbolKind::Data,
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
    
                syms.insert(name.clone(), (None, Some(def_offset), sym, *decl, data.len() as u64, *link));
            } else {
                syms.insert(name.clone(), (None, None, sym, *decl, 0, *link));
            }
        }

        for (name, decl, link) in &self.decls {
            if syms.contains_key(name) {
                continue;
            }

            if *link != Linkage::Extern {
                panic!("symbols which aren't imported need their own data. consider defining one using 'obj.define(\"symbol\", vec![])'");
            }

            let sym = obj.add_symbol(Symbol {
                name: name.clone().as_bytes().to_vec(),
                value: 0,
                size: 0,
                kind: {
                    match decl {
                        Decl::Function => SymbolKind::Text,
                        Decl::Data => SymbolKind::Data,
                        Decl::Constant => SymbolKind::Data,
                    }
                },
                scope: SymbolScope::Dynamic,
                weak: false,
                section: SymbolSection::Undefined,
                flags: SymbolFlags::None,
            });
            
            syms.insert(name.clone(), (None, None, sym, *decl, 0, *link));
        }

        for link in &self.links {
            let (_, off, _, _, _, _) = syms.get(&link.from).expect("expectd valid link source");
            let (_, _, to_sym, _decl, _, _) = syms.get(&link.to).expect("expected valid link destination");

            let addend = 0;/*match decl {
                Decl::Function => if self.triple.getCallConv()? == Target::CallConv::SystemV { 4 } else {0},
                _ => 0,
            };*/
            let offset = -3;

            obj.add_relocation(secText, Relocation {
                offset: (link.at as i64 + offset) as u64 + {if let Some(off) = off { *off } else { 0 }},
                symbol: to_sym.to_owned(),
                addend: link.addend + addend,
                flags: RelocationFlags::Generic { 
                    kind: RelocationKind::PltRelative, 
                    encoding: link.kind, 
                    size: 32, 
                },
            })?;
        }

        if let Some(debug) = debug {
            if self.debug {
                self.emit_dwarf(&mut obj, &syms, debug)?;
            }
        }

        obj.write_stream(file)?;

        Ok(())
    }

    fn emit_range(&self, id: &SymbolId, size: u64) -> Range {
        let id: usize = unsafe {
            std::mem::transmute(*id)
        };

        let adress = Address::Symbol { symbol: id, addend: 0 };

        
        Range::StartLength { begin: adress, length: size }
    }

    fn emit_debug_info(&self, dwarf: &mut DwarfUnit, name: &String, link: &Linkage, id: &SymbolId, size: u64, debug: &DebugRegistry, file_id: FileId) {
        use gimli::write::*;

        let id: usize = unsafe {
            std::mem::transmute(*id)
        };

        let adress = Address::Symbol { symbol: id, addend: 0 };
        
        let subprogram = dwarf.unit.add(dwarf.unit.root(), gimli::DW_TAG_subprogram);
        let entry = dwarf.unit.get_mut(subprogram);

        if *link == Linkage::External {
            entry.set(gimli::DW_AT_external, AttributeValue::Flag(true));
        }

        entry.set(gimli::DW_AT_name, AttributeValue::String(name.as_bytes().to_vec()));
        entry.set(
            gimli::DW_AT_decl_file,
            AttributeValue::FileIndex(Some(file_id)),
        );
        entry.set(gimli::DW_AT_decl_line, AttributeValue::Udata(3));
        entry.set(gimli::DW_AT_decl_column, AttributeValue::Udata(0));

        entry.set(gimli::DW_AT_low_pc, AttributeValue::Address(adress));
        entry.set(gimli::DW_AT_high_pc, AttributeValue::Udata(size));

        dwarf.unit.line_program.begin_sequence(Some(adress));

        if let Some(lines) = debug.locs.get(name) {
            for loc in lines {
                dwarf.unit.line_program.row().line = loc.line;
                dwarf.unit.line_program.row().column = loc.col;
                dwarf.unit.line_program.row().epilogue_begin = loc.epilog;
                dwarf.unit.line_program.row().prologue_end = loc.prolog;
                dwarf.unit.line_program.row().address_offset = loc.adr;
                dwarf.unit.line_program.generate_row();
            }
        } else { todo!() }

        dwarf.unit.line_program.end_sequence(size);

    }

    fn emit_dwarf(&self, obj: &mut Object<'_>, syms: &BTreeMap<String, (Option<SectionId>, Option</*offsest*/u64>, SymbolId, Decl, u64, Linkage)>, debug: DebugRegistry) -> Result<(), Box<dyn Error>> {
        use gimli::write::*;
        let encoding = gimli::Encoding {
            address_size: 8,
            format: gimli::Format::Dwarf32,
            version: 5,
        };

        let mut dwarf = DwarfUnit::new(encoding.clone());

        let mut range_vec = vec![];

        for (_, (_, _, id, decl, size, link)) in syms {
            if *decl == Decl::Function && (*link == Linkage::External || *link == Linkage::Internal) {
                range_vec.push( self.emit_range(id, *size) );
            }
        }

        let range_list_id = dwarf.unit.ranges.add(RangeList(range_vec));

        let root = dwarf.unit.root();
        let entry = dwarf.unit.get_mut(root);

        entry.set(
            gimli::DW_AT_ranges,
            AttributeValue::RangeListRef(range_list_id),
        );

        entry.set(gimli::DW_AT_producer, 
            AttributeValue::String(debug.producer.as_bytes().to_vec())
        );

        entry.set(gimli::DW_AT_language,
            AttributeValue::Language(debug.lang)
        );
        entry.set(gimli::DW_AT_name, AttributeValue::String(debug.file.as_bytes().to_vec()));
        entry.set(gimli::DW_AT_comp_dir, AttributeValue::String(debug.dir.as_bytes().to_vec()));

        dwarf.unit.line_program = LineProgram::new(
            encoding,
            gimli::LineEncoding::default(),
            LineString::new(debug.dir.to_string(), encoding, &mut dwarf.line_strings),
            LineString::new(debug.file.to_string(), encoding, &mut dwarf.line_strings),
            None,
        );

        let dir_id = dwarf.unit.line_program.default_directory();
        let file_string = LineString::new(debug.file.as_bytes(), encoding, &mut dwarf.line_strings);
        let file = dwarf.unit.line_program.add_file(file_string, dir_id, None);

        for (name, (_, _, id, decl, size, link)) in syms {
            if *decl == Decl::Function && (*link == Linkage::External || *link == Linkage::Internal) {
                self.emit_debug_info(&mut dwarf, name, link, id, *size, &debug, file);
            }
        }

        let mut sections = Sections::new(self::Section::new());
        dwarf.write(&mut sections)?;

        sections.for_each_mut(|id, section| -> object::write::Result<()> {
            if section.data.len() == 0 {
                return Ok(());
            }

            let section_id = obj.add_section(Vec::new(), id.name().into(), object::SectionKind::Debug);
            obj.set_section_data(section_id, section.data.take(), 1);
    
            section.id = Some(section_id);
            Ok(())
        })?;

        sections.for_each(|_, section| -> object::write::Result<()> {
            let Some(section_id) = section.id else {
                debug_assert!(section.relocations.is_empty());
                return Ok(());
            };
            for reloc in &section.relocations {
                // The `eh_pe` field is not used in this example because we are not writing
                // unwind information.
                debug_assert!(reloc.eh_pe.is_none());
                let symbol = match reloc.target {
                    RelocationTarget::Section(id) => {
                        obj.section_symbol(sections.get(id).unwrap().id.unwrap())
                    }
                    RelocationTarget::Symbol(id) => {
                        let sym: SymbolId = unsafe {
                            std::mem::transmute(id)
                        };

                        sym
                    },
                };
                obj.add_relocation(
                    section_id,
                    object::write::Relocation {
                        offset: reloc.offset as u64,
                        symbol,
                        addend: reloc.addend,
                        flags: object::RelocationFlags::Generic {
                            kind: match reloc.target {
                                RelocationTarget::Section(_) => object::RelocationKind::SectionOffset,
                                _ => object::RelocationKind::Absolute
                            },
                            encoding: object::RelocationEncoding::Generic,
                            size: reloc.size * 8,
                        },
                    },
                )?;
            }
            Ok(())
        })?;

        Ok(())
    }
}