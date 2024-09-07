use std::fmt::Display;

use super::{Colorize, Pad};

/// # An Error: an Error struct can store Error informations
/// It is used for printing out errors
pub struct Error {
    line: String,
    loc: String,
    msg: String,
    fmtLines: Vec<String>,

    display_location: bool,
}

impl Error {
    /// Creates an new error
    pub fn new<T, U, Z, Y>(msg: T, file: U, line: Z, col: Y) -> Self where 
        T: ToString + Clone + Display + Into<String>,
        U: ToString + Clone + Display + Into<String>,
        Z: ToString + Clone + Display + Into<String>,  
        Y: ToString + Clone + Display + Into<String>
     {
        Self {
            line: line.clone().into(),
            loc: format!("{}:{}:{}:", file, line, col).gray(),
            msg: msg.into(),
            fmtLines: vec![],
            display_location: true,
        }
    }

    /// Deactivates location display
    pub fn deactivateLocationDisplay(&mut self) {
        self.display_location = false;
    }

    /// Sets the code line
    pub fn setCodeLine(&mut self, line: String) {
        self.fmtLines.push( format!("{} {}", format!("  {} | ", self.line).blue(), line) );
    }

    /// Adds a where arrow
    pub fn addWhere<T: ToString + Clone + Display + Into<String>>(&mut self, msg: T, col: u64, size: u64) {
        let msg = msg.to_string();
        let offset = format!("  {} | ", self.line).chars().count() as u64;
        self.fmtLines.push(format!("{}{} {}", String::new().pad_to_len((offset + col + 1) as isize), "^".repeat(size as usize).red(), msg.gray()));
    }

    /// Prints the error to stderr
    pub fn print(&self) {
        
        eprintln!("{} {} {}", self.loc, "error:".bold().red(), self.msg.gray());

        for line in &self.fmtLines {
            eprintln!("{}", line);
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if self.display_location {
            out += &format!("{} {} {}\n", self.loc, "error:".bold().red(), self.msg.gray());
        } else {
            out += &format!("{} {}\n", "error:".bold().red(), self.msg.gray());            
        }

        for line in &self.fmtLines {
            out += &format!("{}\n", line);
        }

        write!(f, "{}", out)
    }
}