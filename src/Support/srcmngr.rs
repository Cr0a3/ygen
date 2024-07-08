use std::{collections::HashMap, error::Error, fmt::Display};

/// The Source Manager: easly chars out of a file 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrcMngr {
    srcFiles: HashMap</*name*/String, (/*content*/String, /*current line*/usize, /*current col*/usize)>,
}

/// An error which can occure in the Source Manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SrcMngrError {
    UnknownFile(String),
}

impl Display for SrcMngrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            SrcMngrError::UnknownFile(name) => format!("unknown file: '{}'", name),
        })
    }
}

impl Error for SrcMngrError {}

impl SrcMngr {
    /// Creats a new SrcMngr
    pub fn new() -> Self {
        Self {
            srcFiles: HashMap::new(),
        }
    }

    /// Registers a new file
    pub fn register(&mut self, name: String, content: String) {
        self.srcFiles.insert(name, (content, 0, 0));
    }

    /// Returns the current char in the specified file
    /// 
    /// Also updates the current line and current colum in the file
    pub fn get_next_char(&mut self, file: String) -> Result<Option<char>, SrcMngrError> {
        if let Some((cont, _line, _col)) = self.srcFiles.get_mut(&file) {
            let mut line = 0;
            let mut col = 0;
            let mut chr = None;

            let mut iter = cont.chars().into_iter(); // not using chars_indicies cuz the _col is relative to the line not to the start of the file
            for it_chr in iter.clone() { // clone original iter so we don't get used after already used error
                iter.next(); // update original iter
                if it_chr == '\n' {
                    line += 1;
                    col = 0;
                };

                if line == *_line && col == *_col {
                    chr = Some(it_chr);
                    break;
                };

                col = 0; 
            };

            if let Some('\n') = iter.next() {
                *_line += 1;
                *_col = 0;
            }

            Ok(chr)
        } else {
            Err( SrcMngrError::UnknownFile(file) )?
        }
    }

    /// Returns the current position in the specified file
    /// 
    /// returns: `Result<(/*line*/usize, /*col*/usize), SrcMngrError>`
    pub fn get_cur_pos(&self, file: String) -> Result<(/*line*/usize, /*col*/usize), SrcMngrError> {
        if let Some((_, line, col)) = self.srcFiles.get(&file) {
            Ok( (*line, *col) )
        } else {
            Err( SrcMngrError::UnknownFile(file) )?
        }
    }
}