#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorLoc {
    pub line: u64,
    pub col: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YccError {
    pub loc: ErrorLoc,

    pub head: &'static str,
    
    pub where_string: String,
}

impl YccError {
    pub fn print(&self, code: &str, file_name: &str) {
        let mut fab = ygen::Support::Error::new(
            self.head, 
            file_name,
            self.loc.line.to_string(), 
            self.loc.col.to_string()
        );

        if let Some(line) = code.split('\n').map(|x| x.to_owned()).collect::<Vec<String>>().get(self.loc.line as usize){
            fab.setCodeLine(line.to_owned());
        }

        fab.addWhere(self.where_string.to_owned(), self.loc.col, self.loc.length);

        fab.print();
    }
}