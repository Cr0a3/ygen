use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::error::Error;

use ygen::prelude::PassManager;
use ygen::Optimizations::Passes;
use ygen::Support::{ColorProfile, Colorize};
use ygen::Target::initializeAllTargets;
use ygen::{Support::Cli, Target::Triple};
use ygen::IR::parser::{gen::IrGen, lexer::IrLexer, parser::IrParser, semnatic::IrSemnatic};
use ygen::IR::Module;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::new(
        "ylc", "ygen ir compiler", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");

    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);

    cli.add_opt("asm", "emit-assembly", "Instead of emitting generated machine code into the file, it will put the generated assembly there");
    cli.add_opt("asm-clr", "print-colored-assembly", "Prints out the generated assembly to stderr");

    cli.add_arg("in", "input", "Input file", /*required*/ true);
    cli.add_arg("o", "out", "The output file to write too", /*required*/ false);

    cli.add_opt("lex", "show-lexed", "Shows the assembly tokens");
    cli.add_opt("exprs", "show-parser-result", "Shows the parsed result");

    cli.add_opt("fmt-clr", "format-colored", "Reprints the ir to stderr with color information");
    cli.add_opt("fmt", "format", "Prints the ir formatted to stdout");
    
    cli.add_opt("O", "optimize-simple", "Run simple optimizations");
    
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

    
    let infile = cli.arg_val("in").expect("we said it was required");
    let outfile;

    if let Some(out) = cli.arg_val("out") {
        outfile = out;
    } else {
        let file = infile.split("/").collect::<Vec<&str>>().last().unwrap_or(&&infile.as_str()).to_string();
        let slices = file.split(".").collect::<Vec<&str>>();
        
        let mut name = String::new();

        for slice in &slices {
            if slices.last() == Some(slice) {
                break;
            }

            name.push_str(slice);
        }

        outfile = format!("{}.o", name);
    }

    let mut infile = match File::open(&infile) {
        Ok(file) => file,
        Err(err) => {
            println!("{}: {} {}", "Error".red().bold(), infile, err);
            exit(-1);
        },
    };

    let mut outfile = match File::options().create(true).write(true).open(&outfile) {
        Ok(file) => file,
        Err(err) => {
            println!("{}: {} {}", "Error".red().bold(), outfile, err);
            exit(-1);
        },
    };

    let mut input = String::new();
    infile.read_to_string(&mut input)?;

    let mut lexer = IrLexer::new(input);
    match lexer.lex() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(-1)
        },
    }

    if cli.opt("lex") {
        println!("Tokens: {:?}", lexer.out);
    }


    let mut parser = IrParser::new(lexer.out);
    match parser.parse() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(-1)
        }
    }

    if cli.opt("exprs") {
        println!("{:?}", parser.out);
    } 

    match IrSemnatic::new(&parser.out).verify() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(-1)
        }
    }

    let mut gen = IrGen::new(parser.out);

    gen.gen();

    let mut module: Module = gen.module();

    if cli.opt("O") {
        let mut opts = PassManager::new();

        opts.add( Passes::PreComputeValue() );

        module.runPassMngr(opts);
    }

    if cli.opt("fmt-clr") {
        eprintln!("{}", module.dumpColored(ColorProfile::default()));
    }

    if cli.opt("fmt") {
        println!("{}", module.dump());
    }

    if cli.opt("asm-clr") {
        let asm = module.emitAsm(triple, &mut initializeAllTargets())?;

        println!("{}", asm);
    }

    if cli.opt("asm") {
        let asm = module.emitAsm(triple, &mut initializeAllTargets())?;

        outfile.write_all(asm.as_bytes())?
    } else {
        module.emitMachineCode(
            triple, 
            &mut initializeAllTargets()
        )?.emit(outfile, None)?;
    }

    Ok(())
}