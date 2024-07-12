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
//!
//! **Arguments:** <br>
//!
//! > **-as=<val>, --assemble-string=<val>** The assembly instruction to assemble <br>
//! > **-triple=<val>, --triple=<val>**      The target triple <br>

use Ygen::Support::Cli;

fn main() {
    let mut cli = Cli::new(
        "ygen-mc", "Ygens machine code playground", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_opt("clr", "color", "Colorates the ouput");
    cli.add_arg("as", "assemble-string", "The assembly instruction to assemble", /*required*/ true);
    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }

    println!("string to assemble: {}", cli.arg_val("as").expect("we said it was required"));
}
