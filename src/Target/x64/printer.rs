use crate::{Obj::Linkage, Target::{Arch, AsmPrinter, CallConv}};

/// prints assembly in x64 style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X64AsmPrinter {}

impl X64AsmPrinter {
    /// creates an new x64 assembly printer
    pub fn new() -> Box<dyn AsmPrinter> { Box::new( Self {} ) }
}

impl AsmPrinter for X64AsmPrinter {
    fn print(&self, module: &mut crate::prelude::Module, registry: &mut crate::Target::TargetRegistry, call_conv: CallConv) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = Vec::new();

        lines.push("section .text\n\n".to_string());

        for (name, func) in module.funcs.clone() {
            if func.linkage == Linkage::Extern {
                lines.push( format!("global {}\n", name) );
                continue;
            }

            if func.linkage == Linkage::External {
                lines.push( format!("global {}\n", name) );
            } 

            lines.push( format!("{}:\n", name) );

            let mut index = 0;

            for block in &func.blocks {
                if index != 0 {
                    lines.push( format!(" {}:\n", block.name) );
                }
                
                let asm_lines = registry.buildAsmForTarget(Arch::X86_64, &block, &func, module)?;
                
                if registry.requires_prolog(&func) && index == 0 {
                    let mut helper = registry.getBackendForFuncOrFork(Arch::X86_64, &func).helper.expect("expected valid helper");

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

        lines.push("section .rodata\n\n".to_string());

        for (_, consta) in &module.consts {
            let mut data = String::new();

            let mut first = true;

            for byte in &consta.data {
                if !first {
                    data.push(',');
                    data.push(' ');
                }

                data.push_str(&format!("\t.byte {:02x?}\n", byte));
                first = true;
            }

            lines.push(format!("{}: \n{}", consta.name, data));
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