mod triple;
mod target_descr;
pub mod x64;
pub mod wasm;
mod registry;
mod whitelist;
pub use x64::initializeX64Target;
pub use wasm::initializeWasmTarget;
pub use whitelist::*;
pub use triple::Triple;
pub use target_descr::TargetBackendDescr;
pub use registry::TargetRegistry;
pub use registry::RegistryError;
mod lexer;
mod compiler;
mod printer;
pub use lexer::Lexer;
pub use compiler::Compiler;
pub use printer::AsmPrinter;

/// Initializes all targets
pub fn initializeAllTargets(triple: Triple) -> Result<TargetRegistry, triple::TripleError> {
    let mut registry = TargetRegistry::new(triple);

    registry.add( Arch::X86_64, initializeX64Target(triple.getCallConv()?) );
    registry.add( Arch::Wasm64, wasm::initializeWasmTarget(triple.getCallConv()?) );

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