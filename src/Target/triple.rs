use std::{error::Error, fmt::Display, process::Command};
use crate::Support::Colorize;

use super::*;

/// The target triple
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triple {
    /// The target architecture to use
    pub arch: Arch,
    /// The Vendor (If anyone knowns what this is please create an issue or a pr and fix the docs)
    pub vendor: Vendor,
    /// The target calling convention to use
    pub os: OS,
    /// The runtime environment
    pub env: Environment,
    /// Object format
    pub bin: ObjFormat,
}

/// An error which can occure during parsing the target triple
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripleError {
    /// An unknown architecture
    UnkonwnArchitecture(String),
    /// An operating system
    UnknownOs(String),
    /// An environment
    UnknownEnv(String),
    /// An unknown object file format
    UnknownObj(String),
    /// An unsupported target triple
    UnsuportedTriple(Triple),
}

impl Display for TripleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", "Error:".bold().red(), {
            match self {
                TripleError::UnkonwnArchitecture(name) => format!("Unknown architecture: '{}'", name),
                TripleError::UnknownOs(name) => format!("Unknown operating system: '{}'", name),
                TripleError::UnknownEnv(name) => format!("Unknown environment (maybe unknown vendor cuz error handling isn't implemented the right way): '{}'", name),
                TripleError::UnknownObj(name) => format!("Unknown object file format: '{}'", name),
                TripleError::UnsuportedTriple(triple) => format!("Unsupported triple: {:?}", triple),
            }
        })
    }
}

impl Error for TripleError {}

impl Triple {
    /// Parses the target triple string. 
    /// Returns the target triple or TripleError
    pub fn parse(string: &str) -> Result<Triple, TripleError> {
        let mut parts = string.split('-');

        let mut arch: Arch = Arch::Unknown;

        if let Some(archstr) = parts.next() {
            arch = match archstr {
                "unknown" => Arch::Unknown,
                "arm" => Arch::Arm,
                "aarch64" => Arch::Aarch64,
                "aarch64_be" => Arch::Aarch64BE,
                "arc" => Arch::Arc,
                "avr" => Arch::Avr,
                "bpfel" => Arch::Bpfel,
                "bpfeb" => Arch::Bpfeb,
                "hexagon" => Arch::Hexagon,
                "mips" => Arch::Mips,
                "mipsel" => Arch::Mipsel,
                "mips64" => Arch::Mips64,
                "mips64el" => Arch::Mips64EL,
                "msp430" => Arch::Msp420,
                "ppc" => Arch::Ppc,
                "ppc64" => Arch::Ppc64,
                "ppc64le" => Arch::Ppc64LE,
                "r600" => Arch::R600,
                "amdgcn" => Arch::AmdGCN,
                "riscv32" => Arch::Riscv32,
                "riscv64" => Arch::Riscv64,
                "sparc" => Arch::Sparc,
                "sparcv9" => Arch::Sparcv9,
                "sparcel" => Arch::Sparcel,
                "systemz" => Arch::SystemZ,
                "tce" => Arch::Tce,
                "thumb" => Arch::Thumb,
                "thumbeb" => Arch::Thumbeb,
                "x86" => Arch::X86,
                "x86_64" => Arch::X86_64,
                "xcore" => Arch::Xcore,
                "nvptx" => Arch::Nvptx,
                "nvptx64" => Arch::Nvptx64,
                "le32" => Arch::Le32,
                "le64" => Arch::Le64,
                "amdil" => Arch::AmdIL,
                "amdil64" => Arch::AmdIL64,
                "hsail" => Arch::Hsail,
                "hsail64" => Arch::Hsail64,
                "spir" => Arch::Spir,
                "spir64" => Arch::Spir64,
                "kalimba" => Arch::Kalimba,
                "shave" => Arch::Shave,
                "lanai" => Arch::Lanai,
                "wasm32" => Arch::Wasm32,
                "wasm64" => Arch::Wasm64,
                "renderscript32" => Arch::Renderscript32,
                "renderscript64" => Arch::Renderscript64,
                _ => Err( TripleError::UnkonwnArchitecture(archstr.to_string()))?,
            };
        }

        let mut vendor: Vendor = Vendor::Unknown;

        let mut has_vendor = true;

        let mut curr = parts.next();

        if let Some(vendorstr) = curr {
            vendor = match vendorstr {
                "unknown" => Vendor::Unknown,
                "apple" => Vendor::Apple,
                "pc" => Vendor::Pc,
                "scei" => Vendor::Scei,
                "bgp" => Vendor::Bgp,
                "freescale" => Vendor::Freescale,
                "ibm" => Vendor::Ibm,
                "imaginationTechnologies" => Vendor::ImaginationTechnologies,
                "mipsTechnologies" => Vendor::MipsTechnologies,
                "nvidia" => Vendor::Nvidia,
                "csr" => Vendor::Csr,
                "myriad" => Vendor::Myriad,
                "amd" => Vendor::Amd,
                "mesa" => Vendor::Mesa,
                "suse" => Vendor::Suse,
                "openEmbedded" => Vendor::OpenEmbedded,
                _ => {has_vendor = false; Vendor::Unknown},
            };
        }

        if has_vendor { curr = parts.next(); }

        let mut os: OS = OS::Unknown;

        if let Some(osstr) = curr {
            os = match osstr {
                "unknown" => OS::Unknown,
                "ananas" => OS::Ananas,
                "cloudabi" => OS::CloudABI,
                "darwin" => OS::Darwin,
                "dragonfly" => OS::DragonFly,
                "freeBSD" => OS::FreeBSD,
                "fuchsia" => OS::Fuchsia,
                "ios" => OS::Ios,
                "kFreeBSD" => OS::KFreeBSD,
                "linux" => OS::Linux,
                "lv2" => OS::Lv2,
                "macOS" => OS::MacOS,
                "netBSD" => OS::NetBSD,
                "openBSD" => OS::OpenBSD,
                "solaris" => OS::Solaris,
                "win32" => OS::Win32,
                "windows" => OS::Win32,
                "haiku" => OS::Haiku,
                "minix" => OS::Minix,
                "rtems" => OS::Rtems,
                "naCL" => OS::NaCl,
                "cnk" => OS::Cnk,
                "aix" => OS::Aix,
                "cuda" => OS::Cuda,
                "nvcl" => OS::Nvcl,
                "amdHSA" => OS::AmdHSA,
                "Ps4" => OS::Ps4,
                "ElfIAMCU" => OS::ElfIAMCU,
                "tvOS" => OS::TvOS,
                "watchOS" => OS::WatchOS,
                "mesa3D" => OS::Mesa3D,
                "contiki" => OS::Contiki,
                "amdPAL" => OS::AmdPAL,
                "hermitCore" => OS::HermitCore,
                "hurd" => OS::Hurd,
                "wasi" => OS::Wasi, 
                "unkown" => OS::Unknown,
                _ => Err( TripleError::UnknownOs(osstr.to_string()) )?
            };
        }

        let mut env = Environment::Unknown;

        if let Some(envstr) = parts.next() {
            env = match envstr {
                "gnu" => Environment::Gnu,
                "gnuABIN32" => Environment::GnuABIN32,
                "gnuABI64" => Environment::GnuABI64,
                "gnuEABI" => Environment::GnuEABI,
                "gnuEABIHF" => Environment::GnuEABIHF,
                "gnuX32" => Environment::GnuX32,
                "code16" => Environment::Code16,
                "eabi" => Environment::Eabi,
                "eabiHF" => Environment::EabiHF,
                "android" => Environment::Android,
                "musl" => Environment::Musl,
                "muslEABI" => Environment::MuslEABI,
                "muslEABIHF" => Environment::MuslEABIHF,
                "msvc" => Environment::Msvc,
                "itanium" => Environment::Itanium,
                "cygnus" => Environment::Cygnus,
                "coreCLR" => Environment::CoreCLR,
                "simulator" => Environment::Simulator,
                _ => Err( TripleError::UnknownEnv(envstr.to_string()) )?,
            };
        }

        let mut bin = ObjFormat::Default;

        if let Some(binstr) = parts.next() {
            bin = match binstr {
                "unknown" => ObjFormat::Unknown,
                "coff" => ObjFormat::Coff,
                "elf" => ObjFormat::Elf,
                "machO" => ObjFormat::MachO,
                "wasm" => ObjFormat::Wasm,
                "xcoff" => ObjFormat::XCoff,
                _ => Err( TripleError::UnknownObj(binstr.to_string()) )?,
            };
        }

        Ok(Triple {
            arch: arch,
            vendor: vendor,
            os: os,
            env: env,
            bin: bin,
        })
    }

    /// Just another name for the parse function
    pub fn from(value: &str) -> Result<Triple, TripleError> {
        Triple::parse(value)
    }

    /// returns the calling convention used by the triple
    pub fn getCallConv(&self) -> Result<CallConv, TripleError> {
        Ok(match self.os {
            OS::Darwin | OS::Ios | OS::TvOS | OS::MacOS | OS::WatchOS => {
                match self.arch {
                    Arch::Aarch64 => CallConv::AppleAarch64,
                    _ => CallConv::SystemV,
                }
            },

            OS::Win32 => CallConv::WindowsFastCall,

            OS::Unknown => {
                match self.arch {
                    Arch::Wasm32 | Arch::Wasm64 => CallConv::WasmBasicCAbi,
                    _ => Err(TripleError::UnsuportedTriple(self.clone()))?
                }
            }

            
            _ => CallConv::SystemV,
        })
    }

    /// Returns the host target triple
    pub fn host() -> Triple {
        Triple::parse(&getHostTargetTripleViaRustc()).unwrap()
    }
}
use std::str;

fn getHostTargetTripleViaRustc() -> String {
    let output = Command::new("rustc")
    .arg("--version")
    .arg("--verbose")
    .output()
    .expect("Failed to execute rustc");

    if output.status.success() {
        let mut out = String::new();
        let stdout = str::from_utf8(&output.stdout).expect("Failed to parse output");
        for line in stdout.lines() {
            if line.starts_with("host:") {
                let target_triple = line.split_whitespace().nth(1).expect("Failed to parse target triple");
                out = target_triple.to_string();
            }
        }

        return out;
    } else {
        panic!()
    }
}