use std::collections::HashMap;
use crate::Obj::Link;

use super::JitFunction;

/// The JitLink dynamiclly links multible functions into one JitFunction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitLinker {
    funcs: HashMap<String, (Vec<u8>, /*entry*/bool)>,
    labels: HashMap<String, Vec<u8>>,
    
    pub(crate) relocs: Vec<Link>,
}

impl JitLinker {
    /// Creates a new linker
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            labels: HashMap::new(),

            relocs: vec![],
        }
    }

    /// Adds a function
    pub fn add_func(&mut self, name: &str, code: Vec<u8>, entry: bool) {
        self.funcs.insert(name.to_string(), (code, entry));
    }

    /// Adds a label
    pub fn add_label(&mut self, name: &str, data: Vec<u8>) {
        self.labels.insert(name.to_string(), data);
    }

    /// Adds an relocation
    pub fn add_reloc(&mut self, link: Link) {
        self.relocs.push(link);
    }

    /// Links the code into a `Vec<u8>`
    pub fn link(&mut self) -> Vec<u8> {
        let mut out = vec![];

        let mut positions = HashMap::new();

        for (func, (data, entry)) in &self.funcs {
            if !*entry { continue }

            // WE NOW HAVE THE ENTRY FUNCTION

            positions.insert(func.as_str(), out.len());

            out.extend_from_slice(&data);
        }
        
        for (func, (data, entry)) in &self.funcs {
            if *entry { continue } // we already added the entry function

            positions.insert(func.as_str(), out.len());

            out.extend_from_slice(&data);
        }

        for (label, data) in &self.labels {
            positions.insert(label.as_str(), out.len());
            out.extend_from_slice(&data);
        }

        for reloc in &self.relocs {
            let from = *positions.get(reloc.from.as_str()).expect(&format!("Unkown symbol: {}", reloc.from));
            let mut to = *positions.get(&reloc.to.as_str()).expect(&format!("Unkown symbol: {}", reloc.to));
        
            println!("from: {}", from);
            println!("to: {}", to);

            let offset = reloc.at + from;
            let offset = offset as i64 + reloc.addend;

            to -= from + reloc.at + 1;
            //to -= (reloc.at as i64) as usize;
            //to += 2;
            let to = to.to_be_bytes();

            let mut set_byte = |idx: i64, to: u8| {
                println!("setting byte {:?} (pos: {}) to {}", out.get(idx as usize), idx, to);
                *out.get_mut(idx as usize).unwrap() = to;
            };

            set_byte(offset + 1, to[7]);
            set_byte(offset + 2, to[6]);
            set_byte(offset + 3, to[5]);
            set_byte(offset + 4, to[4]);
        }

        println!("out: {:02x?}", out);

        out
    }

    /// Links the code and puts it into a page aligned `JitFunction`
    pub unsafe fn engine<T>(&mut self) -> JitFunction<T> {
        let func: JitFunction<T> = JitFunction::new(self.link());

        func
    }
}