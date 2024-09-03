use std::{fs::File, io::Read, process::{exit, Command}};

use ygen::Support::{Cli, Colorize};

mod parse;
use parse::*;

fn main() {
    let mut cli = Cli::new(
        "ytest", "The testing tool for ygen", "1.0", "Cr0a3"
    );
    
    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");

    cli.add_arg("t", "test", "The file to the testcase", true);

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }

    let file = cli.arg_val("t").expect("We said it was required");
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    };

    let parsed = parse(buf);
    
    let args = parsed.cmd.replace("%s", &format!("\"{}\"", &parsed.input));
    let mut args = args.split(" ").collect::<Vec<&str>>();

    let program = args.get(0).expect("expected valid excutable name").to_string();

    args.reverse();
    args.pop();
    args.reverse();


    println!("{}: executing following commandline: '{}{}'", "Info".blue().bold(), program, {
        let mut fmt = String::new();

        for arg in &args {
            fmt.push_str(&format!(" {}", arg));
        }

        fmt
    });

    let mut cmd = Command::new( program );
    
    for arg in args {
        cmd.arg(arg);
    }

    match cmd.spawn() {
        Ok(cmd) => cmd,
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        }
    };

    let out = cmd.output().expect("failed to execute process");

    if out.stdout != parsed.expected_out.as_bytes().to_vec() {
        println!("{}: expected stdout didn't match real stdout", "Error".red().bold());
        println!("\texpected: \"{:#?}\"\n", out.stdout.iter()                                      
            .filter_map(|&byte| {
                if byte >= 32 && byte <= 126 {
                    Some(byte as char)
                } else {
                    None
                }
            }).collect::<String>());
        println!("\tfound: \"{:#?}\"\n", parsed.expected_out.as_bytes().to_vec().iter()                                      
            .filter_map(|&byte| {
                if byte >= 32 && byte <= 126 {
                    Some(byte as char)
                } else {
                    None
                }
            }).collect::<String>());
    }
}