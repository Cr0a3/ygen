use std::collections::HashMap;

use crate::Obj::Link;

use super::{JitFunction, JitLinker};

/// A jit map is a structure which is used to easily map multiple symbols into an jit function (uses the jit linker)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitMap {
    symbols: HashMap</*name*/String, Vec<u8>>,
    symbol_types: HashMap</*name*/String, /*true = func; false = data*/bool>,
    entry_symbol: String,
    relocs: Vec<Link>,
}

impl JitMap {
    /// Creates an new jit map
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            entry_symbol: String::new(),
            symbol_types: HashMap::new(),
            relocs: Vec::new(),
        }
    }

    /// adds a function to the map
    pub fn define_func(&mut self, name: &String, data: Vec<u8>) {
        self.symbols.insert(name.to_owned(), data);
        self.symbol_types.insert(name.to_owned(), true);
    }

    /// adds a data to the map
    pub fn define_data(&mut self, name: &String, data: Vec<u8>) {
        self.symbols.insert(name.to_owned(), data);
        self.symbol_types.insert(name.to_owned(), false);
    }

    /// adds a relocation to the map
    pub fn reloc(&mut self, reloc: Link) {
        self.relocs.push( reloc );
    }

    /// links all the data into an jit linker
    pub fn link(&self) -> JitLinker {
        let mut linker = JitLinker::new();

        for (name, data) in &self.symbols {
            let isfunc = *self.symbol_types.get(name).unwrap();

            if isfunc {
                let entry = name == &self.entry_symbol;

                linker.add_func(&name, data.to_owned(), entry);
            } else { // is data
                linker.add_label(&name, data.to_owned());
            }
        }
        
        for reloc in &self.relocs {
            linker.add_reloc(reloc.to_owned());
        }

        linker
    }

    /// Returns the function as an callable jit function
    pub fn get_function<T>(&mut self, name: &str) -> Option<JitFunction<T>> {
        let name = name.to_owned();

        if !self.symbols.contains_key(&name) {
            // NO FURTHER PROCESSING IS REQUIRED
            return None;
        }

        self.entry_symbol = name;

        Some(unsafe {
            self.link().engine::<T>()
        })
    }
}