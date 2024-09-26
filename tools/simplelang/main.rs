use std::{error::Error, fs::{File, OpenOptions}, io::Read, process::exit};
use logos::Logos;
use ygen::{prelude::*, Support::{Cli, ColorProfile, Colorize}, Target::initializeAllTargets};

mod lexer;
mod parser;
mod ast;
mod macros;
mod semnatic;
mod codegen;

/// syntax: with (a: i32, b: i32) func: { 
///     return a + b;
/// } 
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::new("simplelang", "Simple language example for ygen", "1.0", "Cr0a3");
    
    cli.add_opt("ir", "emit-ir", "Emits the ir");
    cli.add_opt("asm", "emit-asm", "Emits assembly");

    cli.add_opt("lex", "show-lexical-result", "Shows the tokens");
    cli.add_opt("parse", "show-parser-result", "Shows the parsed expressions");

    cli.add_opt("mc", "show-machine-instrs", "Shows the generated portable machine instrs");

    cli.add_arg("in", "input", "The input file", true);
    cli.add_arg("o", "out", "The output file", false);
    cli.add_arg("triple", "triple", "The target triple", false);

    cli.scan();

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

    let mut triple = Triple::host();

    if let Some(target) = cli.arg_val("triple") {
        triple = Triple::parse(&target)?;
    }

    let outfile = match OpenOptions::new().write(true).create(true).open(outfile) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e); 
            exit(-1);
        },
    };

    let mut infile = match File::open(infile) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e); 
            exit(-1);
        },
    };

    let mut input = String::new();
    infile.read_to_string(&mut input)?;

    let mut tokens = vec![];
    let mut lexer = lexer::Token::lexer(&input);

    while let Some(token) = lexer.next() {
        tokens.push( match token {
                Ok(token) => token,
                Err(_) => {
                    let remainder = lexer.slice();
                    let first_char = remainder.chars().next().unwrap();
                    
                    let error = if !first_char.is_ascii() {
                        lexer::LexingError::NonAsciiCharacter
                    } else {
                        lexer::LexingError::UnexpectedCharacter(first_char)
                    };

                    let _tmp;
                    err!(_tmp, "{}", error);
                    exit(-1);
                }
            }
        );
    }

    if cli.opt("lex") {
        eprintln!("{}: {:?}", "Tokens".bold().gray(), &tokens);
    }

    let mut parser = parser::Parser::new(tokens);
    parser.parse();

    if parser.had_errors() {
        exit(-1);
    }

    if cli.opt("parse") {
        eprintln!("{}: {:?}", "Expressions".bold().gray(), parser.out);
    }

    let mut sem = semnatic::Semnatic::new(&parser.out);
    sem.analyze();

    if sem.had_errors() {
        exit(-1);
    }

    let mut code = codegen::CodeGenerator::new(parser.out);
    code.gen();

    let module = code.module();

    //module.verify()?;

    if cli.opt("ir") {
        eprintln!("{}", module.dumpColored(ColorProfile::default()));
    }

    let registry = &mut initializeAllTargets(Triple::host())?;

    if cli.opt("mc") {
        let mc_map = module.emitMachineInstrs(triple, registry)?;

        for (func, instrs) in &mc_map {
            println!("{}:", func);

            for instr in instrs {
                println!("\t{}", instr);
            }

            println!();
        }
    }

    if cli.opt("asm") {
        println!("{}", module.emitAsm(triple, registry)?);
    }

    let obj = module.emitMachineCode(triple, registry, false)?.0;
    obj.emit(outfile, None)?;

    Ok(())

}