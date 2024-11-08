use ygen::{Support::Cli, Target::Triple};

mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod utils;

fn main() {
    let mut cli = Cli::new("ycc", "Ygens c compiler", "(latest)", "Cr0a3");

    // Standart stuff

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_opt("vv", "vversion", "Displays the version with more information");

    // Files

    cli.add_arg("in", "input", "The input-file path", true);

    cli.add_arg("o", "out", "The output path", false);
    
    // Target

    cli.add_arg("triple", "triple", "The target triple (same as -target=...)", false);
    cli.add_arg("target", "target", "The target triple (same as -triple=...)", false);

    // Output

    cli.add_opt("S", "asm", "Outputs assembly code");
    cli.add_opt("emit-ir", "emit-ir", "Outputs ygen ir");

    // Optimization

    cli.add_opt("O0", "no-opt", "Disables all optimizations");
    cli.add_opt("O3", "max-opt", "Activates all optimizations");
    cli.add_arg("p", "passes", "The optimization passes to run", false);
    
    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    } else if cli.opt("vv") { // verbose version information
        println!("ycc v(latest) (c) Cr0a3");
        println!("Host-Target: {}", Triple::host());
    }
    
    let triple = {
        if let Some(triple) = cli.arg_val("triple") {
            match Triple::parse(&triple) {
                Ok(triple) => triple,
                Err(err) => {
                    println!("Error: {}", err);
                    std::process::exit(-1);
                },
            }
        } else {
            Triple::host()
        }
    };

    let infile = cli.arg_val("in").unwrap();

    let code = utils::read_in_file(&infile);
    let out = utils::out_file(&infile, cli.arg_val("out"));

    /////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////
    // 
    // Lexing Phase
    // 
    /////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////

    let mut lexer = lexer::Lexer::new(&code);

    lexer.lex();

    let encountered_errors = lexer.errors.len() > 0; 

    for error in &lexer.errors {
        error.print(&code, &infile);
    }

    if encountered_errors {
        std::process::exit(-1);
    }

    println!("{:#?}", lexer.tokens())

}
