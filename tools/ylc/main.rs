use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{/*collections::HashMap, */error::Error};

use ygen::{Support::Cli/*, Target::Triple*/};
use ygen::IR::parser::{/*gen::IrGen, */lexer::IrLexer, parser::IrParser, semnatic::IrSemnatic};
//use ygen::IR::Module;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::new(
        "ylc", "ygen ir compiler", "1.0", "Cr0a3"
    );

    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");
    cli.add_arg("triple", "triple", "The target triple", /*required*/ false);
    cli.add_arg("in", "input", "Input file", /*required*/ true);

    cli.add_opt("lex", "show-lexed", "Shows the assembly tokens");

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }


    /*let triple = {
        if let Some(triple) = cli.arg_val("triple") {
            Triple::parse(&triple)?
        } else {
            Triple::host()
        }
    };*/

    
    let infile = cli.arg_val("in").expect("we said it was required");
    
    let mut infile = match File::open(infile) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e); 
            exit(-1);
        },
    };

    let mut input = String::new();
    infile.read_to_string(&mut input)?;

    //let mut functions = HashMap::new();
    //let mut consts = HashMap::new();

    let mut lexer = IrLexer::new(input);
    match lexer.lex() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(-1)
        },
    }

    if cli.opt("lex") {
        println!("Tokens: {:#?}", lexer.out);
    }


    let mut parser = IrParser::new(lexer.out);
    match parser.parse() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(-1)
        }
    }

    IrSemnatic::new(&parser.out).verify()?;

    /*let mut gen = IrGen::new(parser.out);

    gen.gen_funcs(&mut functions);
    gen.gen_consts(&mut consts);

    let module = Module {
        funcs: functions,
        consts: consts,
    };*/

    Ok(())
}