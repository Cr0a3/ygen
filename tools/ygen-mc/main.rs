#![allow(rustdoc::invalid_html_tags)]

//! # `ygen-mc`: ygen machine code code playground
//! With the ygen machine code playground you can play around with assembler instructions
//! and see their opcodes. <br>
//! It's suppused to being the ygen variant of the `llvm-mc`
//! ### Usage
//! **Options:** <br>
//! 
//! > **-h, --help**    Displays help<br>
//! > **-v, --version** Displays the version<br>
//! > **-clr, --color** Colorates the ouput<br>
//! > **-lex, --show-lexer** Displays lexing output<br>
//!
//! **Arguments:** <br>
//!
//! > **-as=<val>, --assemble-string=<val>** The assembly instruction to assemble <br>
//! > **-triple=<val>, --triple=<val>**      The target triple <br>

use std::{error::Error, process::exit};

use ygen::{self, Support::{ColorProfile, Colorize}, Target::{initializeAllTargets, Triple}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = ygen::Support::Cli::new(
        "ygen-mc", "Ygens machine code playground", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_opt("no-clr", "no-color", "Disables colors in the ouput");
    cli.add_arg("as", "assemble-string", "The assembly instruction to assemble", /*required*/ true);
    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);

    cli.add_opt("lex", "show-lexed", "Shows the assembly tokens");

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

    let mut registry = initializeAllTargets(triple)?;
    let backend = registry.getBasedOnArch(triple.arch)?;

    let asm =  cli.arg_val("as").expect("we said it was required");
    let lexed = backend.lexer().lex(asm.clone())?;

    if cli.opt("lex") {
        if !cli.opt("no-color") {
            println!("{}: {:?}", "Lexing result".gray(), lexed);
        } else {
            println!("{}: {:?}", "Lexing result", lexed);
        }
    }

    let mut comp = backend.compiler().new(lexed);
    match comp.parse() {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}: {}", "Error", err); 
            exit(-1);
        },
    };

    if !cli.opt("no-asm") {
        if !cli.opt("no-color") {
            println!("{}: {}", "Asm string".gray(), comp.coloredOut(ColorProfile::default()));
        } else {
            println!("Asm string: {}", comp.printOut());
        }
    }

    let out = comp.out();

    let out = match out {
        Ok(out) => out,
        Err(err) => {
            eprintln!("{}: {}", "Error", err); 
            exit(-1);
        },
    };

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
