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
        let mut ret: Vec<u8> = vec![];
        let mut ret_hash: HashMap<&String, Vec<u8>> = HashMap::new();

        let mut funcs_p: HashMap<&String, (&Vec<u8>, usize)> = HashMap::new();

        let cloned = self.funcs.clone();

        for func in &cloned {
            if func.1.1 { // func is first
                let code = &func.1.0;
    
                for byte in code {
                    ret.push(*byte);
                }
    
                ret_hash.insert(&func.0, code.to_vec());
    
                let offset = ret.len() as isize - code.len() as isize;
                let offset = offset as usize;
    
                funcs_p.insert(&func.0, (&code, offset));
            }
        }

        for func in &cloned {
            if func.1.1 { continue; } // func allready added
            let code = &func.1.0;

            for byte in code {
                ret.push(*byte);
            }

            ret_hash.insert(func.0, code.to_vec());

            let offset = ret.len() as isize - code.len() as isize;
            let offset = offset as usize;

            funcs_p.insert(func.0, (&code, offset));
        }

        ret.push(0xC3); // ret so code | labels are split

        for label in &self.labels {
            for byte in label.1 {
                ret.push(*byte);
            }

            ret_hash.insert(&label.0, label.1.to_vec());

            let offset = ret.len() as isize - label.1.len() as isize;
            let offset = offset as usize;

            funcs_p.insert(&label.0, (&label.1, offset));
        }

        for link in self.relocs.iter() {
            let offset = funcs_p.get(&link.from).unwrap().1;
            let target = funcs_p.get(&link.to).unwrap();


            let at = (offset + link.at) as i64;

            let mut pos: Vec<u8> = vec![];

            
            let _pos = target.1 as i32;
            let _pos = _pos - -link.addend as i32;
            let _pos = _pos - at as i32;
            //let _pos  = _pos + 1;
            
            let _pos = _pos.to_le_bytes();

            for i in 0..(-link.addend - 1) {
                let given = _pos.get(i as usize);
                match given {
                    Some(x) => pos.push(*x),
                    None => pos.push(0),
                }
            }
            
            for b in 0..(-link.addend - 1) {
                ret[(at + b) as usize] = pos[b as usize];
            }
        }

        ret
    }

    /// Links the code and puts it into a page aligned `JitFunction`
    pub unsafe fn engine<T>(&mut self) -> JitFunction<T> {
        let func: JitFunction<T> = JitFunction::new(self.link());

        func
    }
}