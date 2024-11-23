use std::error::Error;

use ygen::Support::Colorize;
use ygen::Target::{initializeAllTargets, Triple};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = ygen::Support::Cli::new(
        "ygen-mc", "Ygens machine code playground", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_opt("no-clr", "no-color", "Disables colors in the ouput");
    cli.add_arg("as", "asm", "The assembly instruction to assemble", /*required*/ true);
    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);
    
    cli.add_opt("no-out", "dont-show-output", "Disables compilation output printing");
    cli.add_opt("no-asm", "dont-show-asm-string", "Disables pretty asm string printing printing");

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }


    let triple = {
        if let Some(triple) = cli.arg_val("triple") {
            Triple::parse(&triple)?
        } else {
            Triple::host()
        }
    };

    let registry = initializeAllTargets(triple)?;
    let asm = registry.parse_asm(&cli.arg_val("asm").unwrap())?;

    if !cli.opt("no-asm") {
        println!("Asm string: {}", asm);
    }

    let out = asm.encode();

    if cli.opt("no-out") {
        return Ok(());
    }

    if !cli.opt("no-color") {
        println!("{}: {}", "Compilation result".gray(), {
            let mut fmt = String::from("0x");
    
            for byte in out {
                fmt += &format!("{:#04X}", byte).replace("0x", "");
            }
    
            fmt
        }.magenta());
    } else {   
        println!("Compilation result: 0x{}", {
            let mut fmt = String::new();

            for byte in out {
                fmt += &format!("{:#04X}", byte).replace("0x", "");
            }

            fmt
        });
    }

    Ok(())
}
