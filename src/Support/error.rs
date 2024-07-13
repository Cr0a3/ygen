use super::{Colorize, Pad};

/// # An Error: an Error struct can store Error informations
/// It is used for printing out errors
pub struct Error {
    line: String,
    loc: String,
    msg: String,
    fmtLines: Vec<String>,
}

impl Error {
    /// Creates an new error
    pub fn new(msg: String, file: String, line: String, col: String) -> Self {
        Self {
            line: line.clone(),
            loc: format!("{}:{}:{}:", file, line, col).gray(),
            msg: msg,
            fmtLines: vec![],
        }
    }

    /// Sets the code line
    pub fn setCodeLine(&mut self, line: String) {
        self.fmtLines.push( format!("{} {}", format!("  {} | ", self.line).blue(), line) );
    }

    /// Adds a where arrow
    pub fn addWhere(&mut self, msg: String, col: isize, size: usize) {
        let offset = format!("  {} | ", self.line).chars().count();
        self.fmtLines.push(format!("{}{} {}", String::new().pad_to_len(offset as isize + col), "^".repeat(size).red(), msg.gray()));
    }

    /// Prints the error to stderr
    pub fn print(&self) {
        eprintln!("{} {} {}", self.loc, "error:".bold().red(), self.msg.gray());

        for line in &self.fmtLines {
            eprintln!("{}", line);
        }
    }
}