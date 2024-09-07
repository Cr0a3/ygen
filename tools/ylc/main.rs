use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::error::Error;

use ygen::Support::Colorize;
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

    cli.add_arg("in", "input", "Input file", /*required*/ true);
    cli.add_arg("o", "out", "The output file to write too", /*required*/ false);

    cli.add_opt("lex", "show-lexed", "Shows the assembly tokens");
    cli.add_opt("exprs", "show-parser-result", "Shows the parsed result");

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

    let outfile = match File::options().create(true).write(true).open(&outfile) {
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

    IrSemnatic::new(&parser.out).verify()?;

    let mut gen = IrGen::new(parser.out);

    gen.gen_funcs();
    gen.gen_consts();

    let module: Module = gen.module();

    module.emitMachineCode(
        triple, 
        &mut initializeAllTargets()
    )?.emit(outfile, None)?;

    Ok(())
}