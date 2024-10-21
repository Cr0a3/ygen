use crate::{Obj::Linkage, Target::{Arch, AsmPrinter, CallConv}};

/// prints assembly in wasm style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmAsmPrinter {}

impl WasmAsmPrinter {
    /// creates an new wasm assembly printer
    pub fn new() -> Box<dyn AsmPrinter> { Box::new( Self {} ) }
}

impl AsmPrinter for WasmAsmPrinter {
    fn print(&self, module: &mut crate::prelude::Module, registry: &mut crate::Target::TargetRegistry, call_conv: CallConv) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = Vec::new();

        lines.push(".text\n".to_string());

        for (name, func) in module.funcs.clone() {

            let mut fmt_ty = String::from("(");

            let mut index = 0;

            for arg in &func.ty.args {
                if index != 0 {
                    fmt_ty.push(',');
                    fmt_ty.push(' ');
                }
                fmt_ty.push_str(&arg.to_string());

                index += 1;
            }

            fmt_ty.push_str(&format!(") -> ({})", func.ty.ret));

            lines.push(format!(".functype {} {}\n", name, fmt_ty));

            if func.linkage == Linkage::Extern {
                lines.push( format!(".globl {}\n", name) );
                continue;
            }

            if func.linkage == Linkage::External {
                lines.push( format!(".globl {}\n", name) );
            } else {
                lines.push( format!(".hidden {}\n", name) );
            }

            lines.push( format!("{}:\n", name) );

            let mut index = 0;

            for block in &func.blocks {
                if index != 0 {
                    lines.push( format!(" {}:\n", block.name) );
                }
                
                let asm_lines = registry.buildAsmForTarget(Arch::Wasm64, &block, &func, module)?;
                
                if registry.requires_prolog(&func) && index == 0 {
                    let mut helper = registry.getBackendForFuncOrFork(Arch::Wasm64, &func).helper.expect("expected valid helper");

                    let mut prolog = vec![];

                    helper.compile_prolog(&mut prolog);

                    let mc_instrs = helper.lower.unwrap()(call_conv, prolog);

                    for instr in mc_instrs {
                        for line in instr.dump()?  {
                            lines.push( format!("\t{}\n", line));
                        }
                    }
                }

                for line in asm_lines {
                    lines.push( format!("\t{}\n", line) );
                }
            
                index += 1;
            }
        }

        lines.push(".data\n\n".to_string());

        for (_, consta) in &module.consts {
            lines.push(format!("{}: {:02X?} # {}\n", consta.name, consta.data, consta.data.iter()                                      
                .filter_map(|&byte| {
                    if byte >= 32 && byte <= 126 {
                        Some(byte as char)
                    } else {
                        None
                    }
                }).collect::<String>()));
        }

        let mut out = String::new();

        for line in lines {
            if !line.is_empty() && line != "\n".to_owned() {
                out.push_str(&line);
            }
        }

        Ok(out)

    }

    fn clone_box(&self) -> Box<dyn AsmPrinter> {
        Box::new( self.clone() )
    }
}