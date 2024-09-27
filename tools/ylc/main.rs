use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::error::Error;

use ygen::prelude::{DebugNode, PassManager};
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
    cli.add_arg("passes", "optimization-passes", "The optimization passes to run", false);
    
    cli.add_opt("g", "debug", "Adds debugging metadata");

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

    let mut dbg_file = PathBuf::from("");

    for stmt in &parser.out {
        match stmt {
            ygen::IR::parser::parser::IrStmt::Func { name: _, ret: _, args: _, body, scope: _, location: _ } => {
                for (_, block) in body {
                    for node in &block.body {
                        if let Some(dbg) = node.inst.as_any().downcast_ref::<DebugNode>() {
                            dbg_file = dbg.file.to_owned();
                        }
                    }
                }
            },
            _ => {},
        }
    }

    let mut gen = IrGen::new(parser.out);

    gen.gen();

    let mut module: Module = gen.module();

    module.init_dbg(
        "ygen ir language compiler (ylc)".to_owned(), 
        /*The dwarf std doesn't have ygen ir*/ygen::debug::Lang::Rust, 
        &std::path::PathBuf::from(dbg_file)
    );

    if let Some(passes) = cli.arg_val("passes") {
        let mut opts = PassManager::new();

        let passes = passes.split(',').collect::<Vec<&str>>();

        for pass in passes {
            let pass = match pass.to_lowercase().as_str() {
                "cp" | "const_eval" | "const_evaluation" | "const-eval" | "const-evaluation" =>     Some( Passes::ConstantEvaluation() ),
                "dne" | "dead_node" | "dead_node_elim" | "dead-node" | "dead-node-elimination" =>   Some( Passes::DeadNodeElimination() ),
                _ => {eprintln!("unkown pass: {}", pass); None },
            };

            if let Some(pass) = pass {
                opts.add( pass );
            }
        }

        module.runPassMngr(opts);
    } else if cli.opt("O") {
        let mut opts = PassManager::new();

        opts.add( Passes::ConstantEvaluation() );
        opts.add( Passes::DeadNodeElimination() );

        module.runPassMngr(opts);
    }

    if cli.opt("fmt-clr") {
        eprintln!("{}", module.dumpColored(ColorProfile::default()));
    }

    if cli.opt("fmt") {
        println!("{}", module.dump());
    }

    if cli.opt("asm-clr") {
        let asm = module.emitAsm(triple, &mut initializeAllTargets(triple)?)?;

        println!("{}", asm);
    }

    let debug = cli.opt("g");

    if cli.opt("asm") {
        let asm = module.emitAsm(triple, &mut initializeAllTargets(triple)?)?;

        outfile.write_all(asm.as_bytes())?
    } else {
        let (mut object, debug_registry) = module.emitMachineCode(
            triple, 
            &mut initializeAllTargets(triple)?,
            debug
        )?;
        object.debug = debug;
        object.emit(outfile, debug_registry)?;
    }

    Ok(())
}