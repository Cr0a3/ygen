use super::parser::IrStmt;
use super::IrError;

/// semantic analaysiz for ir stmts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrSemnatic<'a> {
    input: &'a Vec<IrStmt>,
}

impl<'a> IrSemnatic<'a> {
    /// Creates an new ir semnatic analyzizer
    pub fn new(exprs: &'a Vec<IrStmt>) -> Self {
        Self {
            input: exprs,
        }
    }

    /// verifys the input
    pub fn verify(&mut self) -> Result<(), IrError> {
        todo!();

        //Ok(())
    }
}