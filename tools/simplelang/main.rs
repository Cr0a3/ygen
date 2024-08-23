use std::{error::Error, fs::{File, OpenOptions}, io::Read, process::exit};
use logos::Logos;
use Ygen::{prelude::*, Support::{Cli, ColorProfile, Colorize}, Target::initializeAllTargets};

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
    cli.add_opt("parse", "show-parser-result", "Shows the pared expressions");

    cli.add_arg("in", "input", "The input file", true);
    cli.add_arg("o", "out", "The output file", false);
    cli.add_arg("triple", "triple", "The target triple", false);

    cli.scan();

    let infile = cli.arg_val("in").expect("we said it was required");
    let outfile;
    
    if let Some(out) = cli.arg_val("out") {
        outfile = out;
    } else {
        outfile = format!("{}.o", infile.split('.').collect::<Vec<&str>>()[0]);
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

    if cli.opt("ir") {
        eprintln!("{}", module.dumpColored(ColorProfile::default()));
    }

    let registry = &mut initializeAllTargets();

    if cli.opt("asm") {
        println!("{}", module.emitAsm(triple, registry)?);
    }

    module.emitMachineCode(triple, registry)?.emit(outfile)?;

    Ok(())

}