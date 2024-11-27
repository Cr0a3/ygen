use core::str;
use std::{fs::File, io::{Read, Write}};
use std::process::{exit, Command};

use ygen::Support::{Cli, Colorize};

mod parse;
use parse::*;

fn main() {
    let mut cli = Cli::new(
        "ytest", "The testing tool for ygen", "1.0", "Cr0a3"
    );
    
    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");

    cli.add_opt("no-exit", "no-exit-on-error", "Ytest does not quite when an error occurs");
    cli.add_opt("neg-exit", "exit-code-neg", "Ytest exits automaticly even with `no-exit` if the programm returned with code -1");

    cli.add_unpositional("test", true);

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }

    let file = cli.arg_val("test").expect("We said it was required");
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

    let path_str = "./tmp.yl";
    let path2_str = "./tmp2.c";

    let path = std::path::PathBuf::from(path_str);

    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }

    if let Some(input) = parsed.input {
        let mut file = match File::options().write(true).create(true).open(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err);
                exit(-1)
            },
        };
    
        match file.write_all(input.as_bytes()) {
            Ok(_) => {},
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err);
                exit(-1)
            },
        }
    }
    
    let path2 = std::path::PathBuf::from(path2_str);

    if path2.exists() {
        let _ = std::fs::remove_file(&path2);
    }

    if let Some(input) = parsed.input2 {
        let mut file = match File::options().write(true).create(true).open(&path2) {
            Ok(file) => file,
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err);
                exit(-1)
            },
        };
    
        match file.write_all(input.as_bytes()) {
            Ok(_) => {},
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err);
                exit(-1)
            },
        }
    }

    let mut found = String::new();
    let mut found_stderr = String::new();

    let mut code = 0;

    let cmds = parsed.cmd.len();
    let mut index = 0;

    for cmd in parsed.cmd {
        index += 1;

        let mut args = cmd  .replace("%s", path_str)
                                .replace("%c", path2_str);
        if cfg!(target_os = "windows") {
            args = args.replace("./", "");
        }
        let args = unescaper::unescape(&args).unwrap();
        let args = args.trim();


        let mut cmd = Command::new( if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        });
        cmd.arg(if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        });

        cmd.arg(args);

        let out = cmd.output().expect("failed to execute the process");
        let stdout = out.stdout;

        let stdout = str::from_utf8(&stdout).unwrap();
        let stdout = stdout.chars().filter(|x| !x.is_whitespace()).collect::<String>();

        found.push_str(&stdout);

        let stderr = out.stderr;

        let stderr = str::from_utf8(&stderr).unwrap();
        let stderr = stderr.chars().filter(|x| !x.is_whitespace()).collect::<String>();

        found_stderr.push_str( &stderr );


        let status = cmd.status().expect("execution error").code().expect("expected exit code");

        code = status;

        if parsed.ignore_fail {
            continue;
        }

        if status != 0 && cmds != index {
            println!("{}: the programm didn't exit sucessfull with code {}", "Error".red().bold(), status);
            exit(-1);
        }
    }

    let _ = std::fs::remove_file(&path);

    if let Some(expected) = parsed.expected_out {
        if expected != found {
            println!("{}: expected output didn't match actual output", "Error".red().bold());
            println!("found:    {:?}", found);
            println!("expected: {:?}", expected);
            if !cli.opt("no-exit") {
                exit(-1)
            }
        }
    }
    if let Some(expected) = parsed.expected_stderr {
        if expected != found_stderr {
            println!("{}: expected stderr didn't match actual output", "Error".red().bold());
            println!("found:    {:?}", found);
            println!("expected: {:?}", expected);
            if !cli.opt("no-exit") {
                exit(-1)
            }
        }
    }

    if let Some(expected_code) = parsed.expected_code {
        if expected_code != code {
            println!("{}: expected exit code: {} found {}", "Error".red().bold(), expected_code, code);
            if !cli.opt("no-exit") {
                exit(-1)
            }
        } else {
            println!("expected exit code {} matched with found one {}", expected_code, code);
        }
    }

    if parsed.ignore_fail && code == 0 { // expected fail but it did not fail
        println!("{}: expect fail", "Error".red().bold());  
        exit(-1);
    }
}