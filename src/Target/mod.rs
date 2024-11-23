/// logic for triple parsing
pub mod triple;
pub use triple::*;

/// The x86 backend
pub mod x86;

/// Bundler for all target structs
pub mod registry;
pub use registry::*;

use crate::{prelude::Ir, CodeGen::reg::Reg, IR::TypeMetadata};

/// target specific assembly instruction
pub mod instr;
/// compilation struct for targets
pub mod compile;
/// asemmbly printing
pub mod asm_printer;
/// asm parser
pub mod parser;

/// allows/forbidds target specific stuff
pub mod black_list;

/// Initializes all targets
pub fn initializeAllTargets(triple: Triple) -> Result<TargetRegistry, triple::TripleError> {
    let mut registry = TargetRegistry::new(&triple);

    registry.insert(Arch::X86_64, x86::initializeX86Target(triple.getCallConv()?));

    Ok(registry)
}

/// Target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    /// Unknown Architecture
    Unknown,
    /// Arm
    Arm,
    /// Arm eb
    ArmEB,
    /// Aarch 64
    Aarch64,
    /// Aarch64_be
    Aarch64BE,
    /// arc
    Arc,
    /// Avr
    Avr,
    /// Bpfel
    Bpfel,
    /// Bpfeb
    Bpfeb,
    /// Hexagon
    Hexagon,
    /// Mips
    Mips,
    /// Mipsel
    Mipsel,
    /// Mips64
    Mips64,
    /// Mips64EL
    Mips64EL,
    /// Msp420
    Msp420,
    /// Power Pc
    Ppc,
    /// Power PC 64
    Ppc64,
    /// Power PC 64 Little endian
    Ppc64LE,
    /// R600
    R600,
    /// Amd GCN
    AmdGCN,
    /// Riscv32
    Riscv32,
    /// Riscv64
    Riscv64,
    /// Sparc
    Sparc,
    /// Sparc v9
    Sparcv9,
    /// Sparcel
    Sparcel,
    /// SystemZ
    SystemZ,
    /// Tce
    Tce,
    /// Tce le
    TceLe,
    /// Thumb
    Thumb,
    /// Thumb EB
    Thumbeb,
    /// x86
    X86,
    /// x86 64Bit
    X86_64,
    /// Xcore
    Xcore,
    /// Nvptx
    Nvptx,
    /// Nvptx64
    Nvptx64,
    /// Le32
    Le32,
    /// Le64
    Le64,
    /// AmdIL
    AmdIL,
    /// AmdIL64
    AmdIL64,
    /// Hsail
    Hsail,
    /// Hsail64
    Hsail64,
    /// Spir
    Spir,
    /// Spir64
    Spir64,
    /// Kalimba
    Kalimba,
    /// Shave
    Shave,
    /// Lanai
    Lanai,
    /// Wasm32
    Wasm32,
    /// Wasm64
    Wasm64,
    /// RenderScript 32
    Renderscript32,
    /// RenderScript 64
    Renderscript64,
}

/// Target calling convention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallConv {
    /// Windows standart
    WindowsFastCall,
    /// Linux standart
    SystemV,
    /// Apple version of the aarch64 calling convention
    AppleAarch64,
    /// The webassembly calling convention
    WasmBasicCAbi,
}

/// Vendor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vendor {
    /// Unknown Vendor
    Unknown,
    /// Apple
    Apple,
    /// PC
    Pc,
    /// Scei
    Scei,
    /// Bgp
    Bgp,
    /// Freescale
    Freescale,
    /// Ibm
    Ibm,
    /// Imagination Technologies
    ImaginationTechnologies,
    /// Mips Technologies
    MipsTechnologies,
    /// Nvidia
    Nvidia,
    /// Csr
    Csr,
    /// Myriad
    Myriad,
    /// Amd
    Amd,
    /// Mesa
    Mesa,
    /// Suse
    Suse,
    /// Open Embedded
    OpenEmbedded,
}

/// Target OS 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OS {
    /// Unknown OS
    Unknown,
    /// Ananas
    Ananas,
    /// CloudABI
    CloudABI,
    /// Darwin
    Darwin,
    /// DragonFly
    DragonFly,
    /// FreeBSD
    FreeBSD,
    /// Fuchsia
    Fuchsia,
    /// IOS
    Ios,
    /// kFreeBSD
    KFreeBSD,
    /// Linux
    Linux,
    /// Lv2
    Lv2,
    /// MacOS
    MacOS,
    /// NetBSD
    NetBSD,
    /// OpenBSD
    OpenBSD,
    /// Solaris
    Solaris,
    /// Win32
    Win32,
    /// Haiku
    Haiku,
    /// Minix
    Minix,
    /// Rtems
    Rtems,
    /// NaCl
    NaCl,
    /// Cnk
    Cnk,
    /// Aix
    Aix,
    /// Cuda
    Cuda,
    /// Nvcl
    Nvcl,
    /// AmdHSA
    AmdHSA,
    /// Ps4
    Ps4,
    /// ElfIAMCU
    ElfIAMCU,
    /// TvOS
    TvOS,
    /// WatchOS
    WatchOS,
    /// Mesa3D
    Mesa3D,
    /// Contiki
    Contiki,
    /// AmdPAL
    AmdPAL,
    /// HermitCore
    HermitCore,
    /// Hurd
    Hurd,
    /// Wasi
    Wasi,
}

/// Target environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Environment {
    /// Unknown environment
    Unknown,
    /// Gnu
    Gnu,
    /// Gnu ABIN32
    GnuABIN32,
    /// Gnu ABI64
    GnuABI64,
    /// Gnu EABI
    GnuEABI,
    /// Gnu EABIHF
    GnuEABIHF,
    /// Gnu X32
    GnuX32,
    /// Code16
    Code16,
    /// EABI
    Eabi,
    /// EABIHF
    EabiHF,
    /// Android
    Android,
    /// Musl
    Musl,
    /// Musl EABI
    MuslEABI,
    /// Musl EABIHF
    MuslEABIHF,
    /// Msvc
    Msvc,
    /// Itanium
    Itanium,
    /// Cygnus
    Cygnus,
    /// Core CLR
    CoreCLR,
    /// Simulator
    Simulator,
}

/// Target object format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjFormat {
    /// Unknown 
    Unknown,
    /// Coff
    Coff,
    /// Elf
    Elf,
    /// MachO
    MachO,
    /// Wasm
    Wasm,
    /// Xcoff
    XCoff,
    /// Platforms default (e.g: Windows -> Coff)
    Default
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Arch::Unknown => "unknown",
            Arch::Arm => "arm",
            Arch::ArmEB => "armeb",
            Arch::Aarch64 => "aarch64",
            Arch::Aarch64BE => "aarch64_be",
            Arch::Arc => "arc",
            Arch::Avr => "avr",
            Arch::Bpfel => "bpfel",
            Arch::Bpfeb => "bpfeb",
            Arch::Hexagon => "hexagon",
            Arch::Mips => "mips",
            Arch::Mipsel => "mipsel",
            Arch::Mips64 => "mips64",
            Arch::Mips64EL => "mips64el",
            Arch::Msp420 => "msp430",
            Arch::Ppc => "ppc",
            Arch::Ppc64 => "ppc64",
            Arch::Ppc64LE => "ppc64le",
            Arch::R600 => "r600",
            Arch::AmdGCN => "amdgcn",
            Arch::Riscv32 => "riscv32",
            Arch::Riscv64 => "riscv64",
            Arch::Sparc => "sparc",
            Arch::Sparcv9 => "sparcv9",
            Arch::Sparcel => "sparcel",
            Arch::SystemZ => "systemz",
            Arch::Tce => "tce",
            Arch::TceLe => "tcele",
            Arch::Thumb => "thumb",
            Arch::Thumbeb => "thumbeb",
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Xcore => "xcore",
            Arch::Nvptx => "nvptx",
            Arch::Nvptx64 => "nvptx64",
            Arch::Le32 => "le32",
            Arch::Le64 => "le64",
            Arch::AmdIL => "amdil",
            Arch::AmdIL64 => "amdil64",
            Arch::Hsail => "hsail",
            Arch::Hsail64 => "hsail64",
            Arch::Spir => "spir",
            Arch::Spir64 => "spir64",
            Arch::Kalimba => "kalimba",
            Arch::Shave => "shave",
            Arch::Lanai => "lanai",
            Arch::Wasm32 => "wasm32",
            Arch::Wasm64 => "wasm64",
            Arch::Renderscript32 => "renderscript32",
            Arch::Renderscript64 => "renderscript64",
        })
    }
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Vendor::Unknown => "unknown",
            Vendor::Apple => "apple",
            Vendor::Pc => "pc",
            Vendor::Scei => "scei",
            Vendor::Bgp => "bgp",
            Vendor::Freescale => "freescale",
            Vendor::Ibm => "ibm",
            Vendor::ImaginationTechnologies => "imagination",
            Vendor::MipsTechnologies => "mips",
            Vendor::Nvidia => "nvidia",
            Vendor::Csr => "csr",
            Vendor::Myriad => "myriad",
            Vendor::Amd => "amd",
            Vendor::Mesa => "mesa",
            Vendor::Suse => "suse",
            Vendor::OpenEmbedded => "oe",
        })
    }
}
impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OS::Unknown => "unknown",
            OS::Ananas => "ananas",
            OS::CloudABI => "cloudabi",
            OS::Darwin => "darwin",
            OS::DragonFly => "dragonfly",
            OS::FreeBSD => "freebsd",
            OS::Fuchsia => "fuchsia",
            OS::Ios => "ios",
            OS::KFreeBSD => "kfreebsd",
            OS::Linux => "linux",
            OS::Lv2 => "lv2",
            OS::MacOS => "macos",
            OS::NetBSD => "netbsd",
            OS::OpenBSD => "openbsd",
            OS::Solaris => "solaris",
            OS::Win32 => "windows",
            OS::Haiku => "haiku",
            OS::Minix => "minix",
            OS::Rtems => "rtems",
            OS::NaCl => "nacl",
            OS::Cnk => "cnk",
            OS::Aix => "aix",
            OS::Cuda => "cuda",
            OS::Nvcl => "nvcl",
            OS::AmdHSA => "amdhsa",
            OS::Ps4 => "ps4",
            OS::ElfIAMCU => "elfiamcu",
            OS::TvOS => "tvos",
            OS::WatchOS => "watchos",
            OS::Mesa3D => "mesa3d",
            OS::Contiki => "contiki",
            OS::AmdPAL => "amdpal",
            OS::HermitCore => "hermitcore",
            OS::Hurd => "hurd",
            OS::Wasi => "wasi",
        })
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Environment::Unknown => "unknown",
            Environment::Gnu => "gnu",
            Environment::GnuABIN32 => "gnun32",
            Environment::GnuABI64 => "gnuabi64",
            Environment::GnuEABI => "gnueabi",
            Environment::GnuEABIHF => "gnueabihf",
            Environment::GnuX32 => "gnux32",
            Environment::Code16 => "code16",
            Environment::Eabi => "eabi",
            Environment::EabiHF => "eabihf",
            Environment::Android => "android",
            Environment::Musl => "musl",
            Environment::MuslEABI => "musleabi",
            Environment::MuslEABIHF => "musleabihf",
            Environment::Msvc => "msvc",
            Environment::Itanium => "itanium",
            Environment::Cygnus => "cygnus",
            Environment::CoreCLR => "coreclr",
            Environment::Simulator => "simulator",
        })
    }
}

/// Returns the custom visitor for the target if it requires a custom visitor
pub fn own_visitor(target: &Arch, _node: &Box<dyn Ir>) -> Option<fn(&Box<dyn Ir>, &mut Vec<crate::CodeGen::dag::DagNode>)> {
    match target {
        _ => None,
    }
}

/// Returns the return register for the given architecture
pub fn get_ret_reg(arch: &Arch, ty: TypeMetadata) -> Reg {
    match arch {
        Arch::X86_64 => x86::ret_reg(ty),
        _ => todo!("unimplemented target: {:?}", arch)        
    }
}