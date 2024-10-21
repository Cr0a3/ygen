use crate::Support::Colorize;
use crate::Support::Pad;

use std::collections::VecDeque;
use std::env;
use std::process::exit;

/// A cli option
struct CliOpt {
    pub(crate) short: String,
    pub(crate) long: String,
    pub(crate) desc: String,

    pub(crate) was_there: bool, // if the scan is already done, this var saves if it was seen
}

/// A cli argument seperated by a =
struct CliArg {
    pub(crate) short: String,
    pub(crate) long: String,
    pub(crate) desc: String,

    pub(crate) required: bool,

    pub(crate) value: String, // if the scan is already done, this var saves what was there
    pub(crate) was_there: bool, // if the scan is already done, this var saves if it was seen
}

/// A command line parser
pub struct Cli {
    opts: Vec<CliOpt>,
    args: Vec<CliArg>,

    app_desc: String,
    app_version: String,
    app_name: String,
    app_author: String,
}

impl Cli {
    /// Creates a new CLI parser
    pub fn new(name: &str, desc: &str, version: &str, author: &str) -> Self {
        Self {
            opts: vec![],
            args: vec![],

            app_name: name.to_string(),
            app_desc: desc.to_string(),
            app_version: version.to_string(),
            app_author: author.to_string(),
        }
    }

    /// Adds an option
    pub fn add_opt(&mut self, short: &str, long: &str, desc: &str) {
        self.opts.push(CliOpt { short: short.to_string(), long: long.to_string(), desc: desc.to_string(), was_there: false })
    }

    /// Adds an argument which is seperated by a `=`
    pub fn add_arg(&mut self, short: &str, long: &str, desc: &str, required: bool,) {
        self.args.push(CliArg { short: short.to_string(), long: long.to_string(), desc: desc.to_string(), required: required, was_there: false, value: String::new() })
    }

    /// Prints the version and description
    pub fn version(&self) {
        println!("{} v{} (c) {}", self.app_name, self.app_version, self.app_author);
    }

    /// Prints help
    pub fn help(&self) {
        let args: Vec<String> = env::args().collect();

        self.version();
        println!("{}", self.app_desc);

        println!();

        println!("{} {} [OPTIONS] [ARGS]", "Usage:".bold(), args[0]);

        println!("{}", "Options:".bold());

        let mut fmt: Vec<String> = vec![];

        for opt in &self.opts {
            fmt.push(format!("  -{}, --{}", opt.short, opt.long))
        }

        let mut longest = 0;

        for fmt in &fmt {
            let len = fmt.chars().count() as isize;

            if len > longest {
                longest = len;
            }
        }

        let mut index = 0;
        for opt in &self.opts {
            println!("{} {}", fmt[index].pad_to_len(longest).bold(), opt.desc);
            index += 1;
        }
        

        println!();

        println!("{}", "Arguments:".bold());

        let mut fmt: Vec<String> = vec![];

        for arg in &self.args {
            fmt.push(format!("  -{}=<val>, --{}=<val>", arg.short, arg.long))
        }

        let mut longest = 0;

        for fmt in &fmt {
            let len = fmt.chars().count() as isize;

            if len > longest {
                longest = len;
            }
        }

        let mut index = 0;
        for arg in &self.args {
            println!("{} {}", fmt[index].pad_to_len(longest).bold(), arg.desc);
            index += 1;
        }
    }

    /// Scans the arguments
    pub fn scan(&mut self) {
        let mut args: VecDeque<String> = env::args().collect();

        args.pop_front(); // Pop exctuable path

        for arg in args {
            let mut handeld = false;

            if arg.contains("=") {
                for cliarg in self.args.iter_mut() {
                    let splited = arg.split('=').collect::<Vec<&str>>();
                    let arg: String = splited[0].to_string();
                    let val: String = splited[1].to_string();
                    if arg == format!("-{}", cliarg.short) || arg == format!("--{}", cliarg.long) {
                        cliarg.was_there = true;
                        cliarg.value = val;
                        handeld = true;
                    }
                }
            } else {
                for opt in self.opts.iter_mut() {
                    if arg == format!("-{}", opt.short) || arg == format!("--{}", opt.long) {
                        opt.was_there = true;
                        handeld = true;
                    }
                }
            }

            if !handeld {
                println!("{} unexpected arg/opt: '{}'", "ERROR:".bold().red(), arg);
                self.help();
                exit(-1);
            }
        }

        for arg in &self.opts {
            if arg.long == "help".to_owned() && arg.was_there {
                self.help();
                exit(0);
            }
            if arg.long == "version".to_owned() && arg.was_there {
                self.version();
                exit(0);
            }
        } 

        for arg in &self.args {
            if arg.required && !arg.was_there {
                println!("{} required argument: '-{}=<val>' wasn't given", "ERROR:".bold().red(), arg.short);
                self.help();
                exit(-1);
            }
        }

    }

    /// Returns true if option was seen
    pub fn opt(&self, name: &str) -> bool {
        for opt in &self.opts {
            if (opt.short == name || opt.long == name) && opt.was_there {
                return true;
            }
        }

        false
    }
    
    /// Returns true if argument was seen
    pub fn arg(&self, name: &str) -> bool {
        for arg in &self.args {
            if (arg.short == name || arg.long == name) && arg.was_there {
                return true;
            }
        }

        false
    }
    
    /// Returns the value of the seen argument
    pub fn arg_val(&self, name: &str) -> Option<String> {
        for arg in &self.args {
            if (arg.short == name || arg.long == name) && arg.was_there {
                return Some(arg.value.clone());
            }
        }

        None
    }
}