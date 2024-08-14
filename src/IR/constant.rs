
#[derive(Debug, Clone, PartialEq, Eq)]
/// A constant
pub struct Const {
    pub(crate) name: String,
    pub(crate) data: Vec<u8>,
}

impl Const {
    /// creates a new constant
    pub fn new(name: String) -> Self {
        Self {
            name: format!("$@c{}", name),
            data: vec![],
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
}