#![allow(rustdoc::invalid_html_tags)]

//! # `ygen-mc`: Ygen machine code code playground
//! With the ygen machine code playground you can play around with assembler instructions
//! and see their opcodes. <br>
//! It's suppused to being the Ygen variant of the `llvm-mc`
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

use std::error::Error;

use Ygen::{self, Support::Colorize};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Ygen::Support::Cli::new(
        "ygen-mc", "Ygens machine code playground", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_opt("clr", "color", "Colorates the ouput");
    cli.add_arg("as", "assemble-string", "The assembly instruction to assemble", /*required*/ true);
    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);

    cli.add_opt("lex", "show-lexed", "Shows the assembly tokens");

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }

    let asm =  cli.arg_val("as").expect("we said it was required");
    let lexed = Ygen::Target::lex(asm.clone())?;

    if cli.opt("lex") {
        println!("{}: {:?}", "Lexing result".gray(), lexed);
    }

    let mut comp = Ygen::Target::Compiler::new(lexed);
    comp.parse()?;

    println!("{}: {}", "Asm string".gray(), asm);

    println!("{}: {}", "Compilation result".gray(), {
        let mut fmt = String::from("0x");

        for byte in comp.out {
            fmt += &format!("{:#04X}", byte).replace("0x", "");
        }

        fmt
    }.magenta());

    Ok(())
}
