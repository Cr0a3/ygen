use crate::Obj::Linkage;


#[derive(Debug, Clone, PartialEq, Eq)]
/// A constant
pub struct Const {
    pub(crate) name: String,
    pub(crate) data: Vec<u8>,
    pub(crate) linkage: Linkage,
}

impl Const {
    /// creates a new constant
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            data: vec![],
            linkage: Linkage::Internal,
        }
    }

    /// sets the data of the constant
    pub fn set(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// gets the data of the constant
    pub fn get(&mut self) -> Vec<u8>{
        self.data.clone()
    }
    
    /// Sets that the constant is externally visible
    pub fn extrn(&mut self) {
        self.linkage = Linkage::External;
    }

    /// Sets that the constant is imported from another object file
    pub fn import(&mut self) {
        self.linkage = Linkage::Extern;
    }

    /// Sets that the constant is only internally visible
    pub fn private(&mut self) {
        self.linkage = Linkage::Internal;
    }
}