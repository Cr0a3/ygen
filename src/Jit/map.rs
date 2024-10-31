use std::collections::HashMap;

use crate::Obj::Link;

use super::{JitFunction, JitLinker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolType {
    Data,
    Func
}

/// A jit map is a structure which is used to easily map multiple symbols into an jit function (uses the jit linker)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitMap {
    symbols: HashMap<String, Vec<u8>>,
    symbol_types: HashMap<String, SymbolType>,
    entry_symbol: String,
    relocs: Vec<Link>,

    abs_symbols: HashMap<String, usize>,

    pub(crate) deal_with_abs_symbols: Option<Box<dyn AbsSymDealer>>,
}

impl JitMap {
    /// Creates an new jit map
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            entry_symbol: String::new(),
            symbol_types: HashMap::new(),
            relocs: Vec::new(),
            abs_symbols: HashMap::new(),
            deal_with_abs_symbols: None,
        }
    }

    /// adds a function to the map
    pub fn define_func(&mut self, name: &String, data: Vec<u8>) {
        self.symbols.insert(name.to_owned(), data);
        self.symbol_types.insert(name.to_owned(), SymbolType::Func);
    }

    /// adds a data to the map
    pub fn define_data(&mut self, name: &String, data: Vec<u8>) {
        self.symbols.insert(name.to_owned(), data);
        self.symbol_types.insert(name.to_owned(), SymbolType::Data);
    }

    /// adds a relocation to the map
    pub fn reloc(&mut self, reloc: Link) {
        self.relocs.push( reloc );
    }

    /// Sets the absolute adress of 
    pub fn setAbsAdr(&mut self, symbol: &String, adr: usize) {
        if self.deal_with_abs_symbols.is_none() {
            panic!("the jit map has no handler to deal with absolute symbols");
        }

        self.abs_symbols.insert(symbol.to_owned(), adr);
    }

    /// links all the data into an jit linker
    pub fn link(&self) -> JitLinker {
        let mut linker = JitLinker::new();

        for (name, data) in &self.symbols {
            let symbol_type = *self.symbol_types.get(name).unwrap();

            if symbol_type == SymbolType::Func {
                let entry = name == &self.entry_symbol;

                linker.add_func(&name, data.to_owned(), entry);
            } else { // is data
                linker.add_label(&name, data.to_owned());
            }
        }
        
        for reloc in &self.relocs {
            if let Some(adr) = self.abs_symbols.get(&reloc.to) {
                let adr = *adr;
                if let Some(dealer) = &self.deal_with_abs_symbols {
                    let cloned = dealer.clone();
                    linker.add_reloc_with_custom_action(
                        reloc.to_owned(), 
                        move |reloc, code, start_pos| {
                            cloned.handle(code, start_pos + reloc.at, adr)
                        },
                    );
                    continue;
                } else {
                    panic!("the jit map has no handler to deal with absolute symbols")
                }
            }

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

pub(crate) trait AbsSymDealer {
    fn handle(&self, code: &mut Vec<u8>, pos: usize, adr: usize);

    fn dbg(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn cl(&self) -> Box<dyn AbsSymDealer>;
    fn eqal(&self, other: &Box<dyn AbsSymDealer>) -> bool;
}

impl std::fmt::Debug for Box<dyn AbsSymDealer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dbg(f)
    }
}

impl PartialEq for Box<dyn AbsSymDealer> {
    fn eq(&self, other: &Self) -> bool {
        self.eqal(&other)
    }
}

impl Eq for Box<dyn AbsSymDealer> {}

impl Clone for Box<dyn AbsSymDealer> {
    fn clone(&self) -> Self {
        self.cl()
    }
}