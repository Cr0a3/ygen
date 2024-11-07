#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorLoc {
    pub line: u64,
    pub col: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YccError {
    loc: ErrorLoc,

    head: &'static str,
    
    where_string: String,
}

impl YccError {
    pub fn print(&self, code: &str, file_name: &str) {
        let mut fab = ygen::Support::Error::new(
            self.head, 
            file_name,
            self.loc.line.to_string(), 
            self.loc.col.to_string()
        );

        fab.setCodeLine(code.to_string());

        fab.addWhere(self.where_string.to_owned(), self.loc.col, self.loc.length);

        fab.print();
    }
}