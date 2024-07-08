use std::{ops::Range, error::Error};
use super::SrcMngr;

type TokenMgrCallback<T> = fn(&mut SrcMngr) -> Result<Option<(T, usize, Range<usize>)>, Box<dyn Error>>;

/// ## The token manager
/// You maybe asking for what is this usefull?
/// It is very useful, for integrating line number information. Which helps by:
///  * Printing errors in the sematic analasys
///  * Including debug information 
/// 
/// ### Example
/// 
pub struct TokenMgr<T> {
    token_vec: Vec<(T, /*lines*/usize, /*col*/Range<usize>)>,
    callback: TokenMgrCallback<T>,
}


impl<T> TokenMgr<T> {
    /// Creates a new token manager
    pub fn new(callback: TokenMgrCallback<T>) -> Self {
        Self {
            token_vec: vec![],
            callback: callback,
        }
    }

    /// Sets the callback of the TokenMgr which is called for scanning each token
    /// The input function type is like follows:
    ///  `type TokenMgrCallback<T> = fn(&mut SrcMngr) -> Result<Option<(T, /*line*/usize, /*colums*/Range<usize>)>, Box<dyn Error>>;`
    /// #### ! Important Note !
    ///  The callback only scans one token
    pub fn set_backend(&mut self, callback: TokenMgrCallback<T>) {
        self.callback = callback; 
    }


    /// Scans all tokens of an specific input file
    pub fn scan(&mut self, srcMngr: &mut SrcMngr) -> Result<(), Box<dyn Error>> {
        while let Some((tok, line, cols)) = (self.callback)(srcMngr)? {
            self.token_vec.push((tok, line, cols));
        }
        Ok(())
    }
}